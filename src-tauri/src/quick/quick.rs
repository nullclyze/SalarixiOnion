use azalea::bot::BotClientExt;
use azalea::core::position::BlockPos;
use azalea::prelude::PathfinderClientExt;
use azalea::{Vec3, WalkDirection};
use core::time::Duration;
use once_cell::sync::Lazy;
use std::f32::consts::PI;
use std::sync::Arc;
use tokio::time::sleep;

use crate::base::{BOT_REGISTRY, PROFILES, STATES, TASKS};
use crate::common::{
  get_average_coordinates_of_bots, get_inventory_menu, go, go_to, inventory_drop_item,
  set_bot_velocity_y, take_item, this_is_solid_block, SafeClientImpls,
};
use crate::tools::{randfloat, randint, randuint};

pub static QUICK_TASK_MANAGER: Lazy<Arc<QuickTaskManager>> =
  Lazy::new(|| Arc::new(QuickTaskManager::new()));

// Структура QuickTaskManager
pub struct QuickTaskManager;

impl QuickTaskManager {
  pub fn new() -> Self {
    Self
  }

  pub fn execute(&self, name: String) {
    for (number, nickname) in PROFILES.get_all().into_keys().enumerate() {
      let name = name.clone();

      tokio::spawn(async move {
        BOT_REGISTRY
          .get_bot(&nickname, async |bot| match name.as_str() {
            "clear-inventory" => {
              if let Some(menu) = get_inventory_menu(bot) {
                for (slot, item) in menu.slots().iter().enumerate() {
                  if !item.is_empty() {
                    inventory_drop_item(&bot, slot, true);
                  }
                }
              }
            }
            "move-forward" => {
              go(&bot, WalkDirection::Forward);
              sleep(Duration::from_millis(200)).await;
              bot.walk(WalkDirection::None);
              STATES.set_mutual_states(&nickname, "walking", false);
            }
            "move-backward" => {
              go(&bot, WalkDirection::Backward);
              sleep(Duration::from_millis(200)).await;
              bot.walk(WalkDirection::None);
              STATES.set_mutual_states(&nickname, "walking", false);
            }
            "move-left" => {
              go(&bot, WalkDirection::Left);
              sleep(Duration::from_millis(200)).await;
              bot.walk(WalkDirection::None);
              STATES.set_mutual_states(&nickname, "walking", false);
            }
            "move-right" => {
              go(&bot, WalkDirection::Right);
              sleep(Duration::from_millis(200)).await;
              bot.walk(WalkDirection::None);
              STATES.set_mutual_states(&nickname, "walking", false);
            }
            "jump" => {
              bot.jump();
            }
            "shift" => {
              bot.set_crouching(true);
              sleep(Duration::from_millis(200)).await;
              bot.set_crouching(false);
            }
            "quit" => {
              bot.disconnect();
            }
            "fly" => {
              for i in 0..randint(3, 5) {
                set_bot_velocity_y(&bot, randfloat(0.022 * i as f64, 0.031 * i as f64));
                sleep(Duration::from_millis(50)).await;
              }
            }
            "rise" => {
              if STATES.get_state(&nickname, "can_looking")
                && STATES.get_state(&nickname, "can_interacting")
              {
                let mut block_slot = None;

                if let Some(menu) = get_inventory_menu(&bot) {
                  for (slot, item) in menu.slots().iter().enumerate() {
                    if !item.is_empty() {
                      if this_is_solid_block(item.kind()) {
                        block_slot = Some(slot);
                        break;
                      }
                    }
                  }
                }

                if let Some(slot) = block_slot {
                  STATES.set_mutual_states(&nickname, "looking", true);
                  STATES.set_mutual_states(&nickname, "interacting", true);

                  take_item(&bot, slot, true).await;

                  let original_direction_1 = bot.direction();

                  bot.set_direction(
                    original_direction_1.0 + randfloat(-5.0, 5.0) as f32,
                    randfloat(40.0, 58.0) as f32,
                  );
                  sleep(Duration::from_millis(randuint(50, 100))).await;
                  bot.jump();

                  let original_direction_2 = bot.direction();

                  bot.set_direction(
                    original_direction_2.0 + randfloat(-5.0, 5.0) as f32,
                    randfloat(86.0, 90.0) as f32,
                  );
                  sleep(Duration::from_millis(randuint(250, 350))).await;

                  bot.swing_arm();

                  bot.start_use_item();

                  sleep(Duration::from_millis(randuint(150, 250))).await;

                  bot.set_direction(original_direction_1.0, original_direction_1.1);

                  STATES.set_mutual_states(&nickname, "looking", false);
                  STATES.set_mutual_states(&nickname, "interacting", false);
                }
              }
            }
            "capsule" => {
              let position = bot.position();

              let block_positions = vec![
                BlockPos {
                  x: (position.x - 1.0).floor() as i32,
                  y: position.y.floor() as i32,
                  z: position.z.floor() as i32,
                },
                BlockPos {
                  x: (position.x + 1.0).floor() as i32,
                  y: position.y.floor() as i32,
                  z: position.z.floor() as i32,
                },
                BlockPos {
                  x: position.x.floor() as i32,
                  y: position.y.floor() as i32,
                  z: (position.z - 1.0).floor() as i32,
                },
                BlockPos {
                  x: position.x.floor() as i32,
                  y: position.y.floor() as i32,
                  z: (position.z + 1.0).floor() as i32,
                },
                BlockPos {
                  x: (position.x - 1.0).floor() as i32,
                  y: (position.y + 1.0).floor() as i32,
                  z: position.z.floor() as i32,
                },
                BlockPos {
                  x: (position.x + 1.0).floor() as i32,
                  y: (position.y + 1.0).floor() as i32,
                  z: position.z.floor() as i32,
                },
                BlockPos {
                  x: position.x.floor() as i32,
                  y: (position.y + 1.0).floor() as i32,
                  z: (position.z - 1.0).floor() as i32,
                },
                BlockPos {
                  x: position.x.floor() as i32,
                  y: (position.y + 1.0).floor() as i32,
                  z: (position.z + 1.0).floor() as i32,
                },
                BlockPos {
                  x: (position.x - 1.0).floor() as i32,
                  y: (position.y + 2.0).floor() as i32,
                  z: position.z.floor() as i32,
                },
                BlockPos {
                  x: position.x.floor() as i32,
                  y: (position.y + 2.0).floor() as i32,
                  z: position.z.floor() as i32,
                },
              ];

              let mut count = 0;

              for pos in block_positions {
                let mut block_slot = None;

                if let Some(menu) = get_inventory_menu(&bot) {
                  for (slot, item) in menu.slots().iter().enumerate() {
                    if !item.is_empty() {
                      if this_is_solid_block(item.kind()) {
                        block_slot = Some(slot);
                        break;
                      }
                    }
                  }
                }

                if let Some(slot) = block_slot {
                  if STATES.get_state(&nickname, "can_looking")
                    && STATES.get_state(&nickname, "can_interacting")
                  {
                    STATES.set_mutual_states(&nickname, "looking", true);
                    STATES.set_mutual_states(&nickname, "interacting", true);

                    take_item(&bot, slot, true).await;

                    count = count + 1;

                    if count == 10 {
                      sleep(Duration::from_millis(randuint(150, 200))).await;
                    }

                    if count == 9 {
                      bot.jump();
                      sleep(Duration::from_millis(50)).await;
                      bot.set_crouching(true);
                      sleep(Duration::from_millis(randuint(100, 150))).await;
                    }

                    bot.look_at(Vec3::new(pos.x as f64, pos.y as f64, pos.z as f64));

                    sleep(Duration::from_millis(randuint(50, 100))).await;

                    bot.swing_arm();

                    bot.start_use_item();

                    if count == 10 {
                      bot.set_crouching(false);
                    }

                    STATES.set_mutual_states(&nickname, "looking", false);
                    STATES.set_mutual_states(&nickname, "interacting", false);

                    sleep(Duration::from_millis(randuint(100, 150))).await;
                  }
                }
              }
            }
            "unite" => {
              let mut positions = vec![];

              for username in PROFILES.get_all().keys() {
                BOT_REGISTRY
                  .get_bot(username, async |b| {
                    positions.push(b.position());
                  })
                  .await;
              }

              let average_cords = get_average_coordinates_of_bots(&positions);

              go_to(bot.clone(), average_cords.0 as i32, average_cords.2 as i32);
            }
            "turn" => {
              let direction = bot.direction();
              bot.set_direction(direction.0 - 90.0, direction.1);
            }
            "zero" => {
              bot.set_direction(0.0, 0.0);
            }
            "form-circle" => {
              let mut positions = vec![];

              for username in PROFILES.get_all().keys() {
                BOT_REGISTRY
                  .get_bot(username, async |b| {
                    positions.push(b.position());
                  })
                  .await;
              }

              let average_cords = get_average_coordinates_of_bots(&positions);

              let angle = 2.0 * PI * (number as f32) / (PROFILES.map.read().unwrap().len() as f32);
              let x = average_cords.0 + positions.len() as f64 * 0.5 * angle.cos() as f64;
              let z = average_cords.2 + positions.len() as f64 * 0.5 * angle.sin() as f64;

              go_to(bot.clone(), x as i32, z as i32);
            }
            "form-line" => {
              let mut positions = vec![];

              for username in PROFILES.get_all().keys() {
                BOT_REGISTRY
                  .get_bot(username, async |b| {
                    positions.push(b.position());
                  })
                  .await;
              }

              let average_cords = get_average_coordinates_of_bots(&positions);

              let x = average_cords.0 + 1.0 * (number as f64 * 1.0);
              let z = average_cords.2 * (number as f64 * 1.0);

              go_to(bot.clone(), x as i32, z as i32);
            }
            "pathfinder-stop" => {
              bot.stop_pathfinding();
            }
            _ => {}
          })
          .await;

        if name.as_str() == "quit" {
          let _ = BOT_REGISTRY.remove_bot(&nickname).await;

          if let Some(tasks) = TASKS.get(&nickname) {
            tasks.write().unwrap().kill_all_tasks();
          }
          
          STATES.reset(&nickname);
          TASKS.remove(&nickname);
          PROFILES.set_str(&nickname, "status", "Оффлайн");
        }
      });
    }
  }
}
