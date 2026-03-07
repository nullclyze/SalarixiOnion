use azalea::{
  bot::BotClientExt, prelude::PathfinderClientExt, BlockPos, SprintDirection, Vec3, WalkDirection,
};
use once_cell::sync::Lazy;
use rhai::Engine;
use std::sync::{
  atomic::{AtomicBool, Ordering},
  Arc, RwLock,
};

use crate::common::convert_hotbar_slot_to_inventory_slot;
use crate::core::{active_bots_count, current_options, BOT_REGISTRY, PROFILES};
use crate::emit::{send_log, send_message};
use crate::webhook::*;
use crate::extensions::{
  go_to, BotDefaultExt, BotInventoryExt, BotMovementExt, BotPhysicsExt, BotRotationExt, entity_type_from
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

  fn chat(username: &str, message: String) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          bot.chat(&message);
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        bot.chat(&message);
      }
    }
  }

  fn jump(username: &str) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          bot.jump();
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        bot.jump();
      }
    }
  }

  fn swing_arm(username: &str) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          bot.swing_arm();
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        bot.swing_arm();
      }
    }
  }

  fn set_jumping(username: &str, state: bool) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          bot.set_jumping(state);
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        bot.set_jumping(state);
      }
    }
  }

  fn set_crouching(username: &str, state: bool) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          bot.set_crouching(state);
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        bot.set_crouching(state);
      }
    }
  }

  fn set_velocity(username: &str, axis: &str, velocity: f64) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          bot.set_velocity(axis, velocity);
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        bot.set_velocity(axis, velocity);
      }
    }
  }

  fn set_on_ground(username: &str, on_ground: bool) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          bot.set_on_ground(on_ground);
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        bot.set_on_ground(on_ground);
      }
    }
  }

  fn set_old_position(username: &str, x: f64, y: f64, z: f64) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          bot.set_old_position(x, y, z);
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        bot.set_old_position(x, y, z);
      }
    }
  }

  fn set_no_jump_delay(username: &str, delay: u32) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          bot.set_no_jump_delay(delay);
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        bot.set_no_jump_delay(delay);
      }
    }
  }

  fn set_was_touching_water(username: &str, state: bool) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          bot.set_was_touching_water(state);
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        bot.set_was_touching_water(state);
      }
    }
  }

  fn set_has_impulse(username: &str, state: bool) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          bot.set_has_impulse(state);
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        bot.set_has_impulse(state);
      }
    }
  }

  fn start_walking(username: &str, direction_name: String) {
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

    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          bot.start_walking(direction);
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        bot.start_walking(direction);
      }
    }
  }

  fn start_sprinting(username: &str, direction_name: &str) {
    let direction = match direction_name {
      "forward" => SprintDirection::Forward,
      "forward-left" => SprintDirection::ForwardLeft,
      "forward-right" => SprintDirection::ForwardRight,
      _ => return,
    };

    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          bot.start_sprinting(direction);
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        bot.start_sprinting(direction);
      }
    }
  }

  fn start_pathfinder(username: &str, x: i32, z: i32) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        go_to(name.to_string(), x, z);
      });
    } else {
      go_to(username.to_string(), x, z);
    }
  }

  fn stop_move(username: &str) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          bot.stop_move();
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        bot.stop_move();
      }
    }
  }

  fn stop_pathfinder(username: &str) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          bot.stop_pathfinding();
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        bot.stop_pathfinding();
      }
    }
  }

  fn look_at(username: &str, x: f64, y: f64, z: f64) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          bot.look_at(Vec3 { x, y, z });
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        bot.look_at(Vec3 { x, y, z });
      }
    }
  }

  fn look_at_block(username: &str, x: i32, y: i32, z: i32) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          bot.look_at(BlockPos::new(x, y, z).center());
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        bot.look_at(BlockPos::new(x, y, z).center());
      }
    }
  }

  fn place_block(username: &str, x: i32, y: i32, z: i32) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          bot.block_interact(BlockPos::new(x, y, z));
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        bot.block_interact(BlockPos::new(x, y, z));
      }
    }
  }

  fn drop_selected_item(username: &str, lock: bool) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          let selected_slot = convert_hotbar_slot_to_inventory_slot(bot.get_selected_hotbar_slot());
          bot.inventory_drop_item(selected_slot, lock);
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        let selected_slot = convert_hotbar_slot_to_inventory_slot(bot.get_selected_hotbar_slot());
        bot.inventory_drop_item(selected_slot, lock);
      }
    }
  }

  fn drop_all_items(username: &str, lock: bool) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          if let Some(menu) = bot.get_inventory_menu() {
            for (slot, _) in menu.slots().iter().enumerate() {
              bot.inventory_drop_item(slot, lock);
            }
          }
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        if let Some(menu) = bot.get_inventory_menu() {
          for (slot, _) in menu.slots().iter().enumerate() {
            bot.inventory_drop_item(slot, lock);
          }
        }
      }
    }
  }

  fn inv_drop_click(username: &str, slot: usize, lock: bool) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          bot.inventory_drop_item(slot, lock);
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        bot.inventory_drop_item(slot, lock);
      }
    }
  }

  fn inv_shift_click(username: &str, slot: usize, lock: bool) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          bot.inventory_shift_click(slot, lock);
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        bot.inventory_shift_click(slot, lock);
      }
    }
  }

  fn inv_left_click(username: &str, slot: usize, lock: bool) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          bot.inventory_left_click(slot, lock);
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        bot.inventory_left_click(slot, lock);
      }
    }
  }

  fn inv_right_click(username: &str, slot: usize, lock: bool) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          bot.inventory_right_click(slot, lock);
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        bot.inventory_right_click(slot, lock);
      }
    }
  }

  fn attack(username: &str, target: String, distance: f64, look_at_target: bool) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          if let Some(entity) = bot.find_nearest_entity(entity_type_from(target.clone()), distance) {
            if look_at_target {
              bot.look_at_entity(entity, false);
            }

            bot.attack(entity);
          }
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        if let Some(entity) = bot.find_nearest_entity(entity_type_from(target), distance) {
          if look_at_target {
            bot.look_at_entity(entity, false);
          }

          bot.attack(entity);
        }
      }
    }
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

    engine.register_fn("active_bots_count", || active_bots_count());

    engine
      .register_fn("get_bot_id", |username: &str| {
        BOT_REGISTRY.get_bot(username).map(|b| b.id().0 as i64)
      })
      .register_fn("get_bot_ping", |username: &str| {
        BOT_REGISTRY.get_bot(username).map(|b| b.ping())
      })
      .register_fn("get_bot_feet_pos", |username: &str| {
        BOT_REGISTRY.get_bot(username).map(|b| {
          let pos = b.feet_pos();
          (pos.x, pos.y, pos.z)
        })
      })
      .register_fn("get_bot_eye_pos", |username: &str| {
        BOT_REGISTRY.get_bot(username).map(|b| {
          let pos = b.eye_pos();
          (pos.x, pos.y, pos.z)
        })
      })
      .register_fn("get_bot_health", |username: &str| {
        BOT_REGISTRY.get_bot(username).map(|b| b.get_health())
      })
      .register_fn("get_bot_satiety", |username: &str| {
        BOT_REGISTRY.get_bot(username).map(|b| b.get_satiety())
      })
      .register_fn("bot_is_workable", |username: &str| {
        BOT_REGISTRY.get_bot(username).map(|b| b.workable())
      });

    engine
      .register_fn("chat", |username: &str, message: &str| {
        Self::chat(username, message.to_string())
      })
      .register_fn("jump", |username: &str| Self::jump(username))
      .register_fn("swing_arm", |username: &str| Self::swing_arm(username))
      .register_fn("set_jumping", |username: &str, state: bool| {
        Self::set_jumping(username, state)
      })
      .register_fn("set_crouching", |username: &str, state: bool| {
        Self::set_crouching(username, state)
      })
      .register_fn("set_velocity", |username: &str, axis: &str, velocity: f64| {
        Self::set_velocity(username, axis, velocity)
      })
      .register_fn("set_on_ground", |username: &str, on_ground: bool| {
        Self::set_on_ground(username, on_ground)
      })
      .register_fn("set_old_position", |username: &str, x: f64, y: f64, z: f64| {
        Self::set_old_position(username, x, y, z)
      })
      .register_fn("set_no_jump_delay", |username: &str, delay: i64| {
        Self::set_no_jump_delay(username, delay as u32)
      })
      .register_fn("set_was_touching_water", |username: &str, state: bool| {
        Self::set_was_touching_water(username, state)
      })
      .register_fn("set_has_impulse", |username: &str, state: bool| {
        Self::set_has_impulse(username, state)
      })
      .register_fn("start_walking", |username: &str, direction: &str| {
        Self::start_walking(username, direction.to_string())
      })
      .register_fn("start_sprinting", |username: &str, direction: &str| {
        Self::start_sprinting(username, direction)
      })
      .register_fn("start_pathfinder", |username: &str, x: i64, z: i64| {
        Self::start_pathfinder(username, x as i32, z as i32)
      })
      .register_fn("stop_move", |username: &str| Self::stop_move(username))
      .register_fn("stop_pathfinder", |username: &str| {
        Self::stop_pathfinder(username)
      });

    engine
      .register_fn("place_block", |username: &str, x: i64, y: i64, z: i64| {
        Self::place_block(username, x as i32, y as i32, z as i32)
      })
      .register_fn("look_at", |username: &str, x: f64, y: f64, z: f64| {
        Self::look_at(username, x, y, z)
      })
      .register_fn("look_at_block", |username: &str, x: i64, y: i64, z: i64| {
        Self::look_at_block(username, x as i32, y as i32, z as i32)
      });

    engine
      .register_fn("drop_selected_item", |username: &str, lock: bool| {
        Self::drop_selected_item(username, lock)
      })
      .register_fn("drop_all_items", |username: &str, lock: bool| {
        Self::drop_all_items(username, lock)
      })
      .register_fn("inv_drop_click", |username: &str, slot: i64, lock: bool| {
        Self::inv_drop_click(username, slot as usize, lock)
      })
      .register_fn(
        "inv_shift_click",
        |username: &str, slot: i64, lock: bool| {
          Self::inv_shift_click(username, slot as usize, lock)
        },
      )
      .register_fn("inv_left_click", |username: &str, slot: i64, lock: bool| {
        Self::inv_left_click(username, slot as usize, lock)
      })
      .register_fn(
        "inv_right_click",
        |username: &str, slot: i64, lock: bool| {
          Self::inv_right_click(username, slot as usize, lock)
        },
      );

    engine.register_fn(
      "attack",
      |username: &str, target: &str, distance: f64, look_at_target: bool| {
        Self::attack(username, target.to_string(), distance, look_at_target);
      },
    );

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
