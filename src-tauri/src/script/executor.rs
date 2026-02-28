use once_cell::sync::Lazy;
use rhai::Engine;
use std::thread;
use std::sync::{Arc, RwLock, atomic::{AtomicBool, Ordering}};

use crate::{common::{set_bot_on_ground, set_bot_velocity_y}, core::{BOT_REGISTRY, PROFILES, current_options}, emit::{send_log, send_message}, generators::randfloat, methods::SafeClientMethods, webhook::send_webhook};

pub static SCRIPT_EXECUTOR: Lazy<Arc<RwLock<ScriptExecutor>>> =
  Lazy::new(|| Arc::new(RwLock::new(ScriptExecutor::new())));

pub struct ScriptExecutor {
  pub active: Arc<AtomicBool>
}

impl ScriptExecutor {
  pub fn new() -> Self {
    Self { 
      active: Arc::new(AtomicBool::new(false))
    }
  }

  fn set_direction(y_rot: f32, x_rot: f32) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY.get_bot(username, async |bot| {
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
        }).await;
      }
    });
  }

  fn set_velocity_y(velocity_y: f64) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY.get_bot(username, async |bot| {
          set_bot_velocity_y(bot, velocity_y);
        }).await;
      }
    });
  }

  fn set_on_ground(on_ground: bool) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY.get_bot(username, async |bot| {
          set_bot_on_ground(bot, on_ground);
        }).await;
      }
    });
  }

  fn set_jumping(state: bool) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY.get_bot(username, async |bot| {
          if state {
            bot.start_jumping();
          } else {
            bot.stop_jumping();
          }
        }).await;
      }
    });
  }

  fn set_crouching(state: bool) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY.get_bot(username, async |bot| {
          if state {
            bot.start_crouching();
          } else {
            bot.stop_crouching();
          }
        }).await;
      }
    });
  }

  fn chat(message: String) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY.get_bot(username, async |bot| {
          bot.chat(&message);
        }).await;
      }
    });
  }

  pub fn execute(&mut self, script: String) {
    let flag = self.active.clone();

    thread::spawn(move || {
      let mut engine = Engine::new();

      let flag_clone = flag.clone();

      engine.on_progress(move |_| {
        if !flag_clone.load(Ordering::Relaxed) {
          Some(rhai::Dynamic::UNIT)
        } else {
          None
        }
      });

      engine
        .register_fn("print", |str: &str| { println!("[ salarixi onion / script ]: {}", str); })
        .register_fn("log", |text: &str, class: &str| { send_log(text.to_string(), class) })
        .register_fn("message", |name: &str, content: &str| { send_message(name, content.to_string()); } )
        .register_fn("webhook", |content: &str| {
          if let Some(opts) = current_options() {
            send_webhook(opts.webhook.url, content.to_string());
          }
        });

      engine
        .register_fn("chat", |message: &str| { Self::chat(message.to_string()); })
        .register_fn("set_jumping", |state: bool| { Self::set_jumping(state); })
        .register_fn("set_crouching", |state: bool| { Self::set_crouching(state); })
        .register_fn("set_direction", |y_rot: f32, x_rot: f32| Self::set_direction(y_rot, x_rot))
        .register_fn("set_velocity_y", |velocity_y: f64| { Self::set_velocity_y(velocity_y); })
        .register_fn("set_on_ground", |on_ground: bool| Self::set_on_ground(on_ground));

      
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

      flag.store(false, Ordering::Relaxed);
    });

    self.active.store(true, Ordering::Relaxed);
  }

  pub fn stop(&mut self) {
    if self.active.load(Ordering::Relaxed) {
      self.active.store(false, Ordering::Relaxed);

      send_log("Скрипт принудительно остановлен".to_string(), "info");
      send_message("Скриптинг", "Скрипт принудительно остановлен".to_string());
    }
  }
}
