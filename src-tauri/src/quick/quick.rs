use azalea::bot::BotClientExt;
use azalea::core::position::BlockPos;
use azalea::prelude::PathfinderClientExt;
use azalea::{Vec3, WalkDirection};
use core::time::Duration;
use once_cell::sync::Lazy;
use std::f32::consts::PI;
use std::sync::Arc;
use tokio::time::sleep;

use crate::common::{get_average_coordinates_of_bots, this_is_solid_block};
use crate::core::*;
use crate::extensions::{BotDefaultExt, BotInventoryExt, BotMovementExt, BotPhysicsExt, InvClick, go_to};
use crate::generators::{randfloat, randint, randuint};

pub static QUICK_TASK_MANAGER: Lazy<Arc<QuickTaskManager>> =
  Lazy::new(|| Arc::new(QuickTaskManager::new()));

/// Менеджер быстрых задач.
/// Позволяет распределять быстрые задачи ботов.
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
          .async_get_bot(&nickname, async |bot| match name.as_str() {
            "clear-inventory" => {
              if let Some(menu) = bot.get_inventory_menu() {
                for (slot, _) in menu.slots().iter().enumerate() {
                  bot.inventory_click(slot, InvClick::from(4), true);
                }
              }
            }
            "move-forward" => {
              bot.start_walking(WalkDirection::Forward);
              sleep(Duration::from_millis(200)).await;
              bot.stop_move();
            }
            "move-backward" => {
              bot.start_walking(WalkDirection::Backward);
              sleep(Duration::from_millis(200)).await;
              bot.stop_move();
            }
            "move-left" => {
              bot.start_walking(WalkDirection::Left);
              sleep(Duration::from_millis(200)).await;
              bot.stop_move();
            }
            "move-right" => {
              bot.start_walking(WalkDirection::Right);
              sleep(Duration::from_millis(200)).await;
              bot.stop_move();
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
                bot.set_velocity("y", randfloat(0.022 * i as f64, 0.031 * i as f64));
                sleep(Duration::from_millis(50)).await;
              }
            }
            "rise" => {
              if get_state(&nickname, "can_looking") && get_state(&nickname, "can_interacting") {
                let mut block_slot = None;

                if let Some(menu) = bot.get_inventory_menu() {
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
                  set_mutual_states(&nickname, "looking", true);
                  set_mutual_states(&nickname, "interacting", true);

                  bot.take_item(slot, true).await;

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

                  set_mutual_states(&nickname, "looking", false);
                  set_mutual_states(&nickname, "interacting", false);
                }
              }
            }
            "capsule" => {
              let position = bot.feet_pos();

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

                if let Some(menu) = bot.get_inventory_menu() {
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
                  if get_state(&nickname, "can_looking") && get_state(&nickname, "can_interacting")
                  {
                    set_mutual_states(&nickname, "looking", true);
                    set_mutual_states(&nickname, "interacting", true);

                    bot.take_item(slot, true).await;

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

                    set_mutual_states(&nickname, "looking", false);
                    set_mutual_states(&nickname, "interacting", false);

                    sleep(Duration::from_millis(randuint(100, 150))).await;
                  }
                }
              }
            }
            "unite" => {
              let mut positions = vec![];

              for username in PROFILES.get_all().keys() {
                let Some(b) = BOT_REGISTRY.get_bot(username) else {
                  continue;
                };

                positions.push(b.feet_pos());
              }

              let average_cords = get_average_coordinates_of_bots(&positions);

              go_to(
                bot.username(),
                average_cords.0 as i32,
                average_cords.2 as i32,
              );
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
                let Some(b) = BOT_REGISTRY.get_bot(username) else {
                  continue;
                };

                positions.push(b.feet_pos());
              }

              let average_cords = get_average_coordinates_of_bots(&positions);

              let angle = 2.0 * PI * (number as f32) / (PROFILES.map.read().unwrap().len() as f32);
              let x = average_cords.0 + positions.len() as f64 * 0.5 * angle.cos() as f64;
              let z = average_cords.2 + positions.len() as f64 * 0.5 * angle.sin() as f64;

              go_to(bot.username(), x as i32, z as i32);
            }
            "form-line" => {
              let mut positions = vec![];

              for username in PROFILES.get_all().keys() {
                let Some(b) = BOT_REGISTRY.get_bot(username) else {
                  continue;
                };

                positions.push(b.feet_pos());
              }

              let average_cords = get_average_coordinates_of_bots(&positions);

              let x = average_cords.0 + 1.0 * (number as f64 * 1.0);
              let z = average_cords.2 * (number as f64 * 1.0);

              go_to(bot.username(), x as i32, z as i32);
            }
            "pathfinder-stop" => {
              bot.stop_pathfinding();
            }
            _ => {}
          })
          .await;

        if name.as_str() == "quit" {
          let _ = BOT_REGISTRY.take_bot(&nickname).await;

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
