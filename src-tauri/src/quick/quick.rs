use azalea::{Vec3, WalkDirection};
use azalea::bot::BotClientExt;
use azalea::prelude::PathfinderClientExt;
use azalea::inventory::operations::ThrowClick;  
use azalea::core::position::BlockPos;
use std::sync::Arc;
use once_cell::sync::Lazy;
use std::f32::consts::PI; 
use core::time::Duration;
use tokio::time::sleep;

use crate::base::{STATES, get_flow_manager};
use crate::tools::{randfloat, randint, randuint};
use crate::common::{get_average_coordinates_of_bots, go, go_to, get_inventory, set_bot_velocity_y, swing_arm, take_item, this_is_solid_block};


pub static QUICK_TASK_MANAGER: Lazy<Arc<QuickTaskManager>> = Lazy::new(|| { Arc::new(QuickTaskManager::new()) });

// Структура QuickTaskManager
pub struct QuickTaskManager;

impl QuickTaskManager {
  pub fn new() -> Self {
    Self
  }

  pub fn execute(&self, name: String) {
    if let Some(arc) = get_flow_manager() {
      let fm = arc.write();
      
      if fm.bots.len() > 0 {
        for (number, (nickname, bot)) in fm.bots.clone().into_iter().enumerate() {
          match name.as_str() {
            "clear-inventory" => {
              tokio::spawn(async move {
                if let Some(inventory) = get_inventory(&bot) {
                  for slot in 0..=48 {  
                    if let Some(menu) = inventory.menu() {  
                      if let Some(s) = menu.slot(slot) {
                        if !s.is_empty() {  
                          inventory.click(ThrowClick::All { slot: slot as u16 });  
                        }  
                      }
                    }  
                  }

                  STATES.set_mutual_states(&nickname, "walking", false);
                  STATES.set_mutual_states(&nickname, "sprinting", false);
                }
              });
            },
            "move-forward" => {
              tokio::spawn(async move {
                go(&bot, WalkDirection::Forward);
                sleep(Duration::from_millis(100)).await;
                bot.walk(WalkDirection::None);
                STATES.set_mutual_states(&nickname, "walking", false);
              });
            },
            "move-backward" => {
              tokio::spawn(async move {
                go(&bot, WalkDirection::Backward);
                sleep(Duration::from_millis(100)).await;
                bot.walk(WalkDirection::None);
                STATES.set_mutual_states(&nickname, "walking", false);
              });
            },
            "move-left" => {
              tokio::spawn(async move {
                go(&bot, WalkDirection::Left);
                sleep(Duration::from_millis(100)).await;
                bot.walk(WalkDirection::None);
                STATES.set_mutual_states(&nickname, "walking", false);
              });
            },
            "move-right" => {
              tokio::spawn(async move {
                go(&bot, WalkDirection::Right);
                sleep(Duration::from_millis(100)).await;
                bot.walk(WalkDirection::None);
                STATES.set_mutual_states(&nickname, "walking", false);
              });
            },
            "jump" => {
              bot.jump();
            },
            "shift" => {
              tokio::spawn(async move {
                bot.set_crouching(true);
                sleep(Duration::from_millis(200)).await;
                bot.set_crouching(false);
              });
            },
            "quit" => {
              bot.disconnect();
            },
            "fly" => {
              tokio::spawn(async move {
                for i in 0..randint(3,5) {
                  set_bot_velocity_y(&bot, randfloat(0.022 * i as f64, 0.031 * i as f64));
                  sleep(Duration::from_millis(50)).await;
                }
              });
            },
            "rise" => {
              tokio::spawn(async move {
                if STATES.get_state(&nickname, "can_looking") && STATES.get_state(&nickname, "can_interacting") {
                  let mut block_slot = None;

                  for (slot, item) in bot.menu().slots().iter().enumerate() {
                    if !item.is_empty() {
                      if this_is_solid_block(item.kind()) {
                        block_slot = Some(slot);
                        break;
                      }
                    }
                  }

                  if let Some(slot) = block_slot {
                    STATES.set_state(&nickname, "can_walking", false);
                    STATES.set_state(&nickname, "can_sprinting", false);
                    STATES.set_mutual_states(&nickname, "looking", true);
                    STATES.set_mutual_states(&nickname, "interacting", true);

                    take_item(&bot, slot).await;

                    let original_direction_1 = bot.direction();  
      
                    bot.set_direction(original_direction_1.0 + randfloat(-5.0, 5.0) as f32, randfloat(40.0, 58.0) as f32);  
                    sleep(Duration::from_millis(randuint(50, 100))).await;
                    bot.jump();    

                    let original_direction_2 = bot.direction();  

                    bot.set_direction(original_direction_2.0 + randfloat(-5.0, 5.0) as f32, randfloat(86.0, 90.0) as f32);  
                    sleep(Duration::from_millis(randuint(250, 350))).await;

                    swing_arm(&bot); 

                    bot.start_use_item();

                    sleep(Duration::from_millis(randuint(150, 250))).await;
                        
                    bot.set_direction(original_direction_1.0, original_direction_1.1); 

                    STATES.set_state(&nickname, "can_walking", true);
                    STATES.set_state(&nickname, "can_sprinting", true);
                    STATES.set_mutual_states(&nickname, "looking", false);
                    STATES.set_mutual_states(&nickname, "interacting", false);
                  }
                }
              });
            },
            "capsule" => {
              tokio::spawn(async move {
                let position = bot.position();

                let block_positions = vec![
                  BlockPos { x: (position.x - 1.0).floor() as i32, y: position.y.floor() as i32, z: position.z.floor() as i32 },
                  BlockPos { x: (position.x + 1.0).floor() as i32, y: position.y.floor() as i32, z: position.z.floor() as i32 },
                  BlockPos { x: position.x.floor() as i32, y: position.y.floor() as i32, z: (position.z -  1.0).floor() as i32 },
                  BlockPos { x: position.x.floor() as i32, y: position.y.floor() as i32, z: (position.z + 1.0).floor() as i32 },
                  BlockPos { x: (position.x - 1.0).floor() as i32, y: (position.y + 1.0).floor() as i32, z: position.z.floor() as i32 },
                  BlockPos { x: (position.x + 1.0).floor() as i32, y: (position.y + 1.0).floor() as i32, z: position.z.floor() as i32 },
                  BlockPos { x: position.x.floor() as i32, y: (position.y + 1.0).floor() as i32, z: (position.z -  1.0).floor() as i32 },
                  BlockPos { x: position.x.floor() as i32, y: (position.y + 1.0).floor() as i32, z: (position.z + 1.0).floor() as i32 },
                  BlockPos { x: (position.x - 1.0).floor() as i32, y: (position.y + 2.0).floor() as i32, z: position.z.floor() as i32 },
                  BlockPos { x: position.x.floor() as i32, y: (position.y + 2.0).floor() as i32, z: position.z.floor() as i32 }
                ];

                let mut count = 0;

                for pos in block_positions {
                  let mut block_slot = None;

                  for (slot, item) in bot.menu().slots().iter().enumerate() {
                    if !item.is_empty() {
                      if this_is_solid_block(item.kind()) {
                        block_slot = Some(slot);
                        break;
                      }
                    }
                  }

                  if let Some(slot) = block_slot {
                    if STATES.get_state(&nickname, "can_looking") && STATES.get_state(&nickname, "can_interacting") {
                      STATES.set_state(&nickname, "can_walking", false);
                      STATES.set_state(&nickname, "can_sprinting", false);
                      STATES.set_mutual_states(&nickname, "looking", true);
                      STATES.set_mutual_states(&nickname, "interacting", true);

                      take_item(&bot, slot).await;

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

                      swing_arm(&bot);

                      bot.start_use_item();  

                      if count == 10 {
                        bot.set_crouching(false);
                      }

                      STATES.set_state(&nickname, "can_walking", true);
                      STATES.set_state(&nickname, "can_sprinting", true);
                      STATES.set_mutual_states(&nickname, "looking", false);
                      STATES.set_mutual_states(&nickname, "interacting", false);
                      
                      sleep(Duration::from_millis(randuint(100, 150))).await;
                    }
                  }
                }
              });
            },
            "unite" => {
              let mut positions = vec![];

              for (_, bot) in fm.bots.iter() {
                positions.push(bot.position());
              }

              let average_cords = get_average_coordinates_of_bots(positions);

              go_to(bot, average_cords.0 as i32, average_cords.2 as i32);
            },
            "turn" => {
              let direction = bot.direction();
              bot.set_direction(direction.0 - 90.0, direction.1);
            },
            "zero" => {
              bot.set_direction(0.0, 0.0);
            },
            "form-circle" => {
              let mut positions = vec![];

              for (_, bot) in fm.bots.iter() {
                positions.push(bot.position());
              }

              let average_cords = get_average_coordinates_of_bots(positions);
              
              let angle = 2.0 * PI * (number as f32) / (fm.bots.len() as f32);  
              let x = average_cords.0 + 6.0 * angle.cos() as f64;  
              let z = average_cords.2 + 6.0 * angle.sin() as f64;  

              go_to(bot, x as i32, z as i32);
            },
            "form-line" => { 
              let mut positions = vec![];

              for (_, bot) in fm.bots.iter() {
                positions.push(bot.position());
              }

              let average_cords = get_average_coordinates_of_bots(positions);

              let x = average_cords.0 + 1.0 * (number as f64 * 1.0);  
              let z = average_cords.2 + 0.0 * (number as f64 * 1.0);  
                      
              go_to(bot, x as i32, z as i32);
            },
            "pathfinder-stop" => {
              bot.stop_pathfinding();
            },
            _ => {}
          }
        }
      }
    }
  }
}