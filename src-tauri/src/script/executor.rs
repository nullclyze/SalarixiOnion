use azalea::{BlockPos, SprintDirection, Vec3, WalkDirection, bot::BotClientExt, prelude::PathfinderClientExt};
use once_cell::sync::Lazy;
use rhai::Engine;
use std::sync::{
  atomic::{AtomicBool, Ordering},
  Arc, RwLock,
};

use crate::{
  common::{EntityFilter, convert_hotbar_slot_to_inventory_slot, get_nearest_entity, go_to, inventory_drop_item, inventory_left_click, inventory_right_click, inventory_shift_click, look_at_entity}, core::{BOT_REGISTRY, PROFILES, active_bots_count, current_options}, emit::{send_log, send_message}, methods::SafeClientMethods, webhook::send_webhook
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

  fn set_velocity_y(velocity_y: f64) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY
          .get_bot(username, async |bot| {
            bot.set_velocity_y(velocity_y);
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
            bot.set_on_ground(on_ground);
          })
          .await;
      }
    });
  }

  fn jump() {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY
          .get_bot(username, async |bot| {
            bot.jump();
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

  fn start_pathfinder(x: i32, z: i32) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().into_keys() {
        go_to(username, x, z);
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

  fn stop_pathfinder() {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY
          .get_bot(username, async |bot| {
            bot.stop_pathfinding();
          })
          .await;
      }
    });
  }

  fn look_at(x: f64, y: f64, z: f64) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY
          .get_bot(username, async |bot| {
            bot.look_at(Vec3 { x, y, z });
          })
          .await;
      }
    });
  }

  fn look_at_block(x: i32, y: i32, z: i32) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY
          .get_bot(username, async |bot| {
            bot.look_at(BlockPos::new(x, y, z).center());
          })
          .await;
      }
    });
  }

  fn place_block(x: i32, y: i32, z: i32) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY
          .get_bot(username, async |bot| {
            bot.block_interact(BlockPos::new(x, y, z));
          })
          .await;
      }
    });
  }

  fn drop_selected_item(lock: bool) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY
          .get_bot(username, async |bot| {
            inventory_drop_item(bot, convert_hotbar_slot_to_inventory_slot(bot.get_selected_hotbar_slot()), lock);
          })
          .await;
      }
    });
  }

  fn drop_all_items(lock: bool) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY
          .get_bot(username, async |bot| {
            if let Some(menu) = bot.get_inventory_menu() {
              for (slot, _) in menu.slots().iter().enumerate() {
                inventory_drop_item(bot, slot, lock);
              }
            }
          })
          .await;
      }
    });
  }

  fn inv_drop_click(slot: usize, lock: bool) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY
          .get_bot(username, async |bot| {
            inventory_drop_item(bot, slot, lock);
          })
          .await;
      }
    });
  }

  fn inv_shift_click(slot: usize, lock: bool) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY
          .get_bot(username, async |bot| {
            inventory_shift_click(bot, slot, lock);
          })
          .await;
      }
    });
  }

  fn inv_left_click(slot: usize, lock: bool) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY
          .get_bot(username, async |bot| {
            inventory_left_click(bot, slot, lock);
          })
          .await;
      }
    });
  }

  fn inv_right_click(slot: usize, lock: bool) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY
          .get_bot(username, async |bot| {
            inventory_right_click(bot, slot, lock);
          })
          .await;
      }
    });
  }

  fn attack(target: String, distance: f64, look_at_target: bool) {
    tokio::spawn(async move {
      for username in PROFILES.get_all().keys() {
        BOT_REGISTRY
          .get_bot(username, async |bot| {
            if let Some(entity) = get_nearest_entity(bot, EntityFilter::new(bot, &target, distance)) {
              if look_at_target {
                look_at_entity(bot, entity, false);
              }

              bot.attack(entity);
            }
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
      .register_fn("active_bots_count", || {
        active_bots_count()
      })
      .register_fn("chat", |message: &str| {
        Self::chat(message.to_string());
      })
      .register_fn("jump", || {
        Self::jump();
      })
      .register_fn("set_jumping", |state: bool| {
        Self::set_jumping(state);
      })
      .register_fn("set_crouching", |state: bool| {
        Self::set_crouching(state);
      })
      .register_fn("set_velocity_y", |velocity_y: f64| {
        Self::set_velocity_y(velocity_y);
      })
      .register_fn("set_on_ground", |on_ground: bool| {
        Self::set_on_ground(on_ground);
      })
      .register_fn("start_walking", |direction: &str| {
        Self::start_walking(direction.to_string());
      })
      .register_fn("start_sprinting", |direction: &str| {
        Self::start_sprinting(direction.to_string());
      })
      .register_fn("start_pathfinder", |x: i64, z: i64| {
        Self::start_pathfinder(x as i32, z as i32);
      })
      .register_fn("stop_move", || {
        Self::stop_move();
      })
      .register_fn("stop_pathfinder", || {
        Self::stop_pathfinder();
      });

    engine
      .register_fn("place_block", |x: i64, y: i64, z: i64| {
        Self::place_block(x as i32, y as i32, z as i32);
      })
      .register_fn("look_at", |x: f64, y: f64, z: f64| {
        Self::look_at(x, y, z);
      })
      .register_fn("look_at_block", |x: i64, y: i64, z: i64| {
        Self::look_at_block(x as i32, y as i32, z as i32);
      });

    engine
      .register_fn("drop_selected_item", |lock: bool| {
        Self::drop_selected_item(lock);
      })
      .register_fn("drop_all_items", |lock: bool| {
        Self::drop_all_items(lock);
      })
      .register_fn("inv_drop_click", |slot: i64, lock: bool| {
        Self::inv_drop_click(slot as usize, lock);
      })
      .register_fn("inv_shift_click", |slot: i64, lock: bool| {
        Self::inv_shift_click(slot as usize, lock);
      })
      .register_fn("inv_left_click", |slot: i64, lock: bool| {
        Self::inv_left_click(slot as usize, lock);
      })
      .register_fn("inv_right_click", |slot: i64, lock: bool| {
        Self::inv_right_click(slot as usize, lock);
      });

    engine
      .register_fn("attack", |target: &str, distance: f64, look_at_target: bool| {
        Self::attack(target.to_string(), distance, look_at_target);
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
