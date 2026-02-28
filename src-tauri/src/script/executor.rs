use azalea::{SprintDirection, WalkDirection};
use once_cell::sync::Lazy;
use rhai::Engine;
use std::sync::{
  atomic::{AtomicBool, Ordering},
  Arc, RwLock,
};

use crate::{
  common::{set_bot_on_ground, set_bot_velocity_y},
  core::{current_options, BOT_REGISTRY, PROFILES},
  emit::{send_log, send_message},
  generators::randfloat,
  methods::SafeClientMethods,
  webhook::send_webhook,
};

pub static SCRIPT_EXECUTOR: Lazy<Arc<RwLock<ScriptExecutor>>> =
  Lazy::new(|| Arc::new(RwLock::new(ScriptExecutor::new())));

pub struct ScriptExecutor {
  pub active: Arc<AtomicBool>,
}

impl ScriptExecutor {
  pub fn new() -> Self {
    Self {
      active: Arc::new(AtomicBool::new(false)),
    }
  }

  fn set_direction(y_rot: f32, x_rot: f32) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY
          .get_bot(username, async |bot| {
            let y = if y_rot == 255.0 {
              randfloat(-180.0, 180.0) as f32
            } else {
              y_rot
            };

            let x = if x_rot == 255.0 {
              randfloat(-90.0, 90.0) as f32
            } else {
              x_rot
            };

            bot.set_direction(y, x);
          })
          .await;
      }
    });
  }

  fn set_velocity_y(velocity_y: f64) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY
          .get_bot(username, async |bot| {
            set_bot_velocity_y(bot, velocity_y);
          })
          .await;
      }
    });
  }

  fn set_on_ground(on_ground: bool) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY
          .get_bot(username, async |bot| {
            set_bot_on_ground(bot, on_ground);
          })
          .await;
      }
    });
  }

  fn set_jumping(state: bool) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY
          .get_bot(username, async |bot| {
            if state {
              bot.start_jumping();
            } else {
              bot.stop_jumping();
            }
          })
          .await;
      }
    });
  }

  fn set_crouching(state: bool) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY
          .get_bot(username, async |bot| {
            if state {
              bot.start_crouching();
            } else {
              bot.stop_crouching();
            }
          })
          .await;
      }
    });
  }

  fn chat(message: String) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY
          .get_bot(username, async |bot| {
            bot.chat(&message);
          })
          .await;
      }
    });
  }

  fn start_walking(direction_name: String) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY
          .get_bot(username, async |bot| {
            let direction = match direction_name.as_str() {
              "forward" => WalkDirection::Forward,
              "backward" => WalkDirection::Backward,
              "left" => WalkDirection::Left,
              "right" => WalkDirection::Right,
              "forward-left" => WalkDirection::ForwardLeft,
              "forward-right" => WalkDirection::ForwardRight,
              "backward-left" => WalkDirection::BackwardLeft,
              "backward-right" => WalkDirection::BackwardRight,
              _ => return,
            };

            bot.start_walking(direction);
          })
          .await;
      }
    });
  }

  fn start_sprinting(direction_name: String) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY
          .get_bot(username, async |bot| {
            let direction = match direction_name.as_str() {
              "forward" => SprintDirection::Forward,
              "forward-left" => SprintDirection::ForwardLeft,
              "forward-right" => SprintDirection::ForwardRight,
              _ => return,
            };

            bot.start_sprinting(direction);
          })
          .await;
      }
    });
  }

  fn stop_move() {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY
          .get_bot(username, async |bot| {
            bot.stop_move();
          })
          .await;
      }
    });
  }

  pub fn execute(&mut self, script: String) {
    self.active.store(true, Ordering::Relaxed);

    let mut engine = Engine::new();

    let active_clone = self.active.clone();

    engine.on_progress(move |_| {
      if !active_clone.load(Ordering::Relaxed) {
        Some(rhai::Dynamic::UNIT)
      } else {
        None
      }
    });

    engine
      .register_fn("print", |str: &str| {
        println!("[ salarixi onion / script ]: {}", str);
      })
      .register_fn("log", |text: &str, class: &str| {
        send_log(text.to_string(), class)
      })
      .register_fn("message", |name: &str, content: &str| {
        send_message(name, content.to_string());
      })
      .register_fn("webhook", |content: &str| {
        if let Some(opts) = current_options() {
          send_webhook(opts.webhook.url, content.to_string());
        }
      });

    engine
      .register_fn("chat", |message: &str| {
        Self::chat(message.to_string());
      })
      .register_fn("set_jumping", |state: bool| {
        Self::set_jumping(state);
      })
      .register_fn("set_crouching", |state: bool| {
        Self::set_crouching(state);
      })
      .register_fn("set_direction", |y_rot: f32, x_rot: f32| {
        Self::set_direction(y_rot, x_rot)
      })
      .register_fn("set_velocity_y", |velocity_y: f64| {
        Self::set_velocity_y(velocity_y);
      })
      .register_fn("set_on_ground", |on_ground: bool| {
        Self::set_on_ground(on_ground)
      })
      .register_fn("start_walking", |direction: &str| {
        Self::start_walking(direction.to_string());
      })
      .register_fn("start_sprinting", |direction: &str| {
        Self::start_sprinting(direction.to_string());
      })
      .register_fn("stop_move", || {
        Self::stop_move();
      });

    match engine.eval::<()>(script.as_str()) {
      Ok(_) => {
        send_log("Скрипт выполнен".to_string(), "info");
        send_message("Скриптинг", "Скрипт выполнен".to_string());
      }
      Err(err) => {
        send_log(format!("Ошибка выполнения скрипта: {}", err), "error");
        send_message("Скриптинг", "Не удалось выполнить скрипт".to_string());
      }
    }

    self.active.store(false, Ordering::Relaxed);
  }

  pub fn stop(&mut self) {
    if self.active.load(Ordering::Relaxed) {
      self.active.store(false, Ordering::Relaxed);

      send_log("Скрипт принудительно остановлен".to_string(), "info");
      send_message("Скриптинг", "Скрипт принудительно остановлен".to_string());
    }
  }
}
