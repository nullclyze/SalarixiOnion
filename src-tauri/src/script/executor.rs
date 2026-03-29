use azalea::{
  bot::BotClientExt, prelude::PathfinderClientExt,
  protocol::packets::game::s_interact::InteractionHand, BlockPos, SprintDirection, Vec3,
  WalkDirection,
};
use once_cell::sync::Lazy;
use rhai::{Dynamic, Engine, EvalAltResult, Map};
use std::sync::{
  atomic::{AtomicBool, Ordering},
  Arc, RwLock,
};

use crate::core::{active_bots_count, current_options, BOT_REGISTRY, PROFILES};
use crate::emit::{send_log, send_message};
use crate::extensions::InvClick;
use crate::extensions::{
  entity_type_from, go_to, BotDefaultExt, BotInteractExt, BotInventoryExt, BotMovementExt,
  BotPhysicsExt, BotRotationExt,
};
use crate::webhook::*;
use crate::{
  common::convert_hotbar_slot_to_inventory_slot,
  core::{get_state, set_state},
};

pub static SCRIPT_EXECUTOR: Lazy<ScriptExecutor> = Lazy::new(|| ScriptExecutor::new());

static EXECUTOR_STATE: Lazy<Arc<RwLock<AtomicBool>>> =
  Lazy::new(|| Arc::new(RwLock::new(AtomicBool::new(false))));

pub struct ScriptExecutor;

impl ScriptExecutor {
  pub fn new() -> Self {
    Self
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
          let selected_slot = convert_hotbar_slot_to_inventory_slot(bot.get_selected_slot());
          bot.inventory_click(selected_slot, InvClick::from(4), lock);
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        let selected_slot = convert_hotbar_slot_to_inventory_slot(bot.get_selected_slot());
        bot.inventory_click(selected_slot, InvClick::from(4), lock);
      }
    }
  }

  fn drop_all_items(username: &str, lock: bool) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          if let Some(menu) = bot.get_inventory_menu() {
            for (slot, _) in menu.slots().iter().enumerate() {
              bot.inventory_click(slot, InvClick::from(4), lock);
            }
          }
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        if let Some(menu) = bot.get_inventory_menu() {
          for (slot, _) in menu.slots().iter().enumerate() {
            bot.inventory_click(slot, InvClick::from(4), lock);
          }
        }
      }
    }
  }

  fn inv_click(username: &str, slot: usize, click_mode: u8, lock: bool) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          bot.inventory_click(slot, InvClick::from(click_mode), lock);
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        bot.inventory_click(slot, InvClick::from(click_mode), lock);
      }
    }
  }

  fn inv_click_on(username: &str, name: &str, click_mode: u8, lock: bool) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          bot.inventory_click_on(name, InvClick::from(click_mode), lock);
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        bot.inventory_click_on(name, InvClick::from(click_mode), lock);
      }
    }
  }

  fn set_selected_slot(username: &str, slot: u8) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          bot.set_selected_hotbar_slot(slot);
        }
      });
    } else {
      if let Some(bot) = BOT_REGISTRY.get_bot(username) {
        bot.set_selected_hotbar_slot(slot);
      }
    }
  }

  fn attack(username: &str, target: String, distance: f64, look_at_target: bool) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        if let Some(bot) = BOT_REGISTRY.get_bot(name) {
          if let Some(entity) = bot.find_nearest_entity(entity_type_from(target.clone()), distance)
          {
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

  fn start_use_item(username: &str, hand: &str) {
    let interaction_hand = if hand == "offhand" {
      InteractionHand::OffHand
    } else {
      InteractionHand::MainHand
    };

    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        BOT_REGISTRY.get_bot(name).map(|bot| {
          bot.start_use_held_item(interaction_hand);
        });
      });
    } else {
      BOT_REGISTRY.get_bot(username).map(|bot| {
        bot.start_use_held_item(interaction_hand);
      });
    }
  }

  fn release_use_item(username: &str) {
    if username == "*" {
      PROFILES.get_all().keys().for_each(|name| {
        BOT_REGISTRY.get_bot(name).map(|bot| {
          bot.release_use_held_item();
        });
      });
    } else {
      BOT_REGISTRY.get_bot(username).map(|bot| {
        bot.release_use_held_item();
      });
    }
  }

  fn parse_username(su: &str, mu: &str) -> String {
    if su == "#" {
      mu.to_string()
    } else {
      su.to_string()
    }
  }

  fn create_dynamic_vec(x: f64, y: f64, z: f64) -> Dynamic {
    let mut map = Map::new();
    map.insert("x".into(), Dynamic::from_float(x));
    map.insert("y".into(), Dynamic::from_float(y));
    map.insert("z".into(), Dynamic::from_float(z));
    Dynamic::from_map(map)
  }

  pub fn execute(&self, su: String, script: String) {
    EXECUTOR_STATE
      .read()
      .unwrap()
      .store(true, Ordering::Relaxed);

    let mut engine = Engine::new();

    engine.on_progress(move |_| {
      if !EXECUTOR_STATE.read().unwrap().load(Ordering::Relaxed) {
        Some(Dynamic::UNIT)
      } else {
        None
      }
    });

    engine
      .register_fn("println", |str: &str| {
        println!("[ salarixi onion / script ]: {}", str);
      })
      .register_fn("log", |text: &str, class: &str| {
        send_log(text.to_string(), class);
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
      .register_fn("get_bot_id", {
        let su = su.clone();
        move |mu: &str| {
          BOT_REGISTRY
            .get_bot(&Self::parse_username(&su, mu))
            .map(|b| b.id().0 as i64).unwrap_or(0)
        }
      })
      .register_fn("get_bot_ping", {
        let su = su.clone();
        move |mu: &str| {
          BOT_REGISTRY
            .get_bot(&Self::parse_username(&su, mu))
            .map(|b| b.ping() as i64).unwrap_or(0)
        }
      })
      .register_fn("get_bot_feet_pos", {
        let su = su.clone();
        move |mu: &str| {
          BOT_REGISTRY
            .get_bot(&Self::parse_username(&su, mu))
            .map(|b| {
              let pos = b.feet_pos();
              Self::create_dynamic_vec(pos.x, pos.y, pos.z)
            }).unwrap_or(Self::create_dynamic_vec(0.0, 0.0, 0.0))
        }
      })
      .register_fn("get_bot_eye_pos", {
        let su = su.clone();
        move |mu: &str| {
          BOT_REGISTRY
            .get_bot(&Self::parse_username(&su, mu))
            .map(|b| {
              let pos = b.eye_pos();
              Self::create_dynamic_vec(pos.x, pos.y, pos.z)
            }).unwrap_or(Self::create_dynamic_vec(0.0, 0.0, 0.0))
        }
      })
      .register_fn("get_bot_health", {
        let su = su.clone();
        move |mu: &str| {
          BOT_REGISTRY
            .get_bot(&Self::parse_username(&su, mu))
            .map(|b| b.get_health() as f64).unwrap_or(0.0)
        }
      })
      .register_fn("get_bot_satiety", {
        let su = su.clone();
        move |mu: &str| {
          BOT_REGISTRY
            .get_bot(&Self::parse_username(&su, mu))
            .map(|b| b.get_satiety() as i64).unwrap_or(0)
        }
      })
      .register_fn("bot_is_workable", {
        let su = su.clone();
        move |mu: &str| {
          BOT_REGISTRY
            .get_bot(&Self::parse_username(&su, mu))
            .map(|b| b.workable()).unwrap_or(false)
        }
      });

    engine
      .register_fn("chat", {
        let su = su.clone();
        move |mu: &str, message: &str| {
          Self::chat(&Self::parse_username(&su, mu), message.to_string())
        }
      })
      .register_fn("jump", {
        let su = su.clone();
        move |mu: &str| Self::jump(&Self::parse_username(&su, mu))
      })
      .register_fn("swing_arm", {
        let su = su.clone();
        move |mu: &str| Self::swing_arm(&Self::parse_username(&su, mu))
      })
      .register_fn("set_jumping", {
        let su = su.clone();
        move |mu: &str, state: bool| Self::set_jumping(&Self::parse_username(&su, mu), state)
      })
      .register_fn("set_crouching", {
        let su = su.clone();
        move |mu: &str, state: bool| Self::set_crouching(&Self::parse_username(&su, mu), state)
      })
      .register_fn("set_velocity", {
        let su = su.clone();
        move |mu: &str, axis: &str, velocity: f64| {
          Self::set_velocity(&Self::parse_username(&su, mu), axis, velocity)
        }
      })
      .register_fn("set_on_ground", {
        let su = su.clone();
        move |mu: &str, on_ground: bool| {
          Self::set_on_ground(&Self::parse_username(&su, mu), on_ground)
        }
      })
      .register_fn("set_old_position", {
        let su = su.clone();
        move |mu: &str, x: f64, y: f64, z: f64| {
          Self::set_old_position(&Self::parse_username(&su, mu), x, y, z)
        }
      })
      .register_fn("set_no_jump_delay", {
        let su = su.clone();
        move |mu: &str, delay: i64| {
          Self::set_no_jump_delay(&Self::parse_username(&su, mu), delay as u32)
        }
      })
      .register_fn("set_was_touching_water", {
        let su = su.clone();
        move |mu: &str, state: bool| {
          Self::set_was_touching_water(&Self::parse_username(&su, mu), state)
        }
      })
      .register_fn("set_has_impulse", {
        let su = su.clone();
        move |mu: &str, state: bool| Self::set_has_impulse(&Self::parse_username(&su, mu), state)
      })
      .register_fn("start_walking", {
        let su = su.clone();
        move |mu: &str, direction: &str| {
          Self::start_walking(&Self::parse_username(&su, mu), direction.to_string())
        }
      })
      .register_fn("start_sprinting", {
        let su = su.clone();
        move |mu: &str, direction: &str| {
          Self::start_sprinting(&Self::parse_username(&su, mu), direction)
        }
      })
      .register_fn("start_pathfinder", {
        let su = su.clone();
        move |mu: &str, x: i64, z: i64| {
          Self::start_pathfinder(&Self::parse_username(&su, mu), x as i32, z as i32)
        }
      })
      .register_fn("stop_move", {
        let su = su.clone();
        move |mu: &str| Self::stop_move(&Self::parse_username(&su, mu))
      })
      .register_fn("stop_pathfinder", {
        let su = su.clone();
        move |mu: &str| Self::stop_pathfinder(&Self::parse_username(&su, mu))
      });

    engine
      .register_fn("place_block", {
        let su = su.clone();
        move |mu: &str, x: i64, y: i64, z: i64| {
          Self::place_block(&Self::parse_username(&su, mu), x as i32, y as i32, z as i32)
        }
      })
      .register_fn("look_at", {
        let su = su.clone();
        move |mu: &str, x: f64, y: f64, z: f64| {
          Self::look_at(&Self::parse_username(&su, mu), x, y, z)
        }
      })
      .register_fn("look_at_block", {
        let su = su.clone();
        move |mu: &str, x: i64, y: i64, z: i64| {
          Self::look_at_block(&Self::parse_username(&su, mu), x as i32, y as i32, z as i32)
        }
      });

    engine
      .register_fn("drop_selected_item", {
        let su = su.clone();
        move |mu: &str, lock: bool| Self::drop_selected_item(&Self::parse_username(&su, mu), lock)
      })
      .register_fn("drop_all_items", {
        let su = su.clone();
        move |mu: &str, lock: bool| Self::drop_all_items(&Self::parse_username(&su, mu), lock)
      })
      .register_fn("inv_click", {
        let su = su.clone();
        move |mu: &str, slot: i64, mode: i64, lock: bool| {
          Self::inv_click(
            &Self::parse_username(&su, mu),
            slot as usize,
            mode as u8,
            lock,
          )
        }
      })
      .register_fn("inv_click_on", {
        let su = su.clone();
        move |mu: &str, name: &str, mode: i64, lock: bool| {
          Self::inv_click_on(&Self::parse_username(&su, mu), name, mode as u8, lock)
        }
      })
      .register_fn("set_selected_slot", {
        let su = su.clone();
        move |mu: &str, slot: i64| {
          Self::set_selected_slot(&Self::parse_username(&su, mu), slot as u8)
        }
      });

    engine
      .register_fn("attack", {
        let su = su.clone();
        move |mu: &str, target: &str, distance: f64, look_at_target: bool| {
          Self::attack(
            &Self::parse_username(&su, mu),
            target.to_string(),
            distance,
            look_at_target,
          );
        }
      })
      .register_fn("start_use_item", {
        let su = su.clone();
        move |mu: &str, hand: &str| Self::start_use_item(&Self::parse_username(&su, mu), hand)
      })
      .register_fn("release_use_item", {
        let su = su.clone();
        move |mu: &str| Self::release_use_item(&Self::parse_username(&su, mu))
      });

    engine
      .register_fn("set_state", {
        let su = su.clone();
        move |mu: &str, field: &str, value: bool| {
          set_state(&Self::parse_username(&su, mu), field, value)
        }
      })
      .register_fn("get_state", {
        let su = su.clone();
        move |mu: &str, field: &str| get_state(&Self::parse_username(&su, mu), field)
      });

    match engine.eval::<()>(script.as_str()) {
      Ok(_) => {
        send_log("Скрипт выполнен".to_string(), "info");
        send_message("Скриптинг", "Скрипт выполнен".to_string());
      }
      Err(err) => match err.as_ref() {
        &EvalAltResult::ErrorTerminated(_, _) => {
          send_log("Скрипт принудительно остановлен".to_string(), "info");
          send_message("Скриптинг", "Скрипт принудительно остановлен".to_string());
        }
        _ => {
          send_log(format!("Ошибка выполнения скрипта: {}", err), "error");
          send_message("Скриптинг", "Не удалось выполнить скрипт".to_string());
        }
      },
    }

    EXECUTOR_STATE
      .read()
      .unwrap()
      .store(false, Ordering::Relaxed);
  }

  pub fn stop(&self) {
    if EXECUTOR_STATE.read().unwrap().load(Ordering::Relaxed) {
      EXECUTOR_STATE
        .read()
        .unwrap()
        .store(false, Ordering::Relaxed);
    }
  }
}
