use azalea::Vec3;
use azalea::bot::BotClientExt;
use azalea::prelude::{ContainerClientExt, PathfinderClientExt};
use azalea::inventory::operations::ThrowClick;  
use azalea::pathfinder::goals::XZGoal;  
use azalea::pathfinder::PathfinderOpts;
use azalea::pathfinder::astar::PathfinderTimeout;
use azalea::pathfinder::moves;
use azalea::entity::Physics;
use azalea::interact::SwingArmEvent;
use azalea::core::position::BlockPos;
use core::time;
use std::f32::consts::PI; 

use crate::base::get_flow_manager;
use crate::tools::{randfloat, randuint};


// Структура QuickTaskManager
pub struct QuickTaskManager;

impl QuickTaskManager {
  pub async fn execute(name: String) {
    if let Some(arc) = get_flow_manager() {
      let fm = arc.write();
      
      if fm.bots_count > 0 {
        match name.as_str() {
          "clear-inventory" => {
            for bot in fm.bots.clone().into_values() {
              let inventory = bot.get_inventory();  

              for slot in 0..=48 {  
                if let Some(menu) = inventory.menu() {  
                  if let Some(s) = menu.slot(slot) {
                    if !s.is_empty() {  
                      inventory.click(ThrowClick::All { slot: slot as u16 });  
                    }  
                  }
                }  
              }
            }
          },
          "move-forward" => {
            for bot in fm.bots.clone().into_values() {
              tokio::task::spawn(async move {
                bot.walk(azalea::WalkDirection::Forward);
                tokio::time::sleep(time::Duration::from_millis(100)).await;
                bot.walk(azalea::WalkDirection::None);
              });
            }
          },
          "move-backward" => {
            for bot in fm.bots.clone().into_values() {
              tokio::task::spawn(async move {
                bot.walk(azalea::WalkDirection::Backward);
                tokio::time::sleep(time::Duration::from_millis(100)).await;
                bot.walk(azalea::WalkDirection::None);
              });
            }
          },
          "move-left" => {
            for bot in fm.bots.clone().into_values() {
              tokio::task::spawn(async move {
                bot.walk(azalea::WalkDirection::Left);
                tokio::time::sleep(time::Duration::from_millis(100)).await;
                bot.walk(azalea::WalkDirection::None);
              });
            }
          },
          "move-right" => {
            for bot in fm.bots.clone().into_values() {
              tokio::task::spawn(async move {
                bot.walk(azalea::WalkDirection::Right);
                tokio::time::sleep(time::Duration::from_millis(100)).await;
                bot.walk(azalea::WalkDirection::None);
              });
            }
          },
          "jump" => {
            for bot in fm.bots.clone().into_values() {
              bot.jump();
            }
          },
          "shift" => {
            for bot in fm.bots.clone().into_values() {
              tokio::task::spawn(async move {
                bot.set_crouching(true);
                tokio::time::sleep(time::Duration::from_millis(200)).await;
                bot.set_crouching(false);
              });
            }
          },
          "quit" => {
            for bot in fm.bots.clone().into_values() {
              bot.disconnect();
            }
          },
          "fly" => {
            for bot in fm.bots.clone().into_values() {
              let mut ecs = bot.ecs.lock();  
              let mut physics = ecs.get_mut::<Physics>(bot.entity).unwrap();  
                  
              physics.velocity.y = randfloat(0.22, 0.31);  
            }
          },
          "rise" => {
            for bot in fm.bots.clone().into_values() {
              tokio::task::spawn(async move {
                let original_direction_1 = bot.direction();  
      
                bot.set_direction(original_direction_1.0 + randfloat(-5.0, 5.0) as f32, randfloat(40.0, 58.0) as f32);  
                bot.wait_ticks(randuint(1, 2) as usize).await;
                bot.jump();    

                let original_direction_2 = bot.direction();  

                bot.set_direction(original_direction_2.0 + randfloat(-5.0, 5.0) as f32, randfloat(86.0, 90.0) as f32);  
                bot.wait_ticks(randuint(5, 7) as usize).await;

                bot.ecs.lock().trigger(SwingArmEvent { entity: bot.entity });  

                bot.start_use_item();

                bot.wait_ticks(randuint(3, 5) as usize).await;
                    
                bot.set_direction(original_direction_1.0, original_direction_1.1);  
              });
            }
          },
          "capsule" => {
            for bot in fm.bots.clone().into_values() {
              tokio::task::spawn(async move {
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
                  count = count + 1;

                  if count == 10 {
                    bot.wait_ticks(randuint(4, 5) as usize).await;
                  }

                  if count == 9 {
                    bot.jump();
                    bot.wait_ticks(1).await;
                    bot.set_crouching(true);
                    bot.wait_ticks(randuint(2, 3) as usize).await;                  
                  }

                  bot.look_at(Vec3::new(pos.x as f64, pos.y as f64, pos.z as f64));

                  bot.wait_ticks(randuint(1, 2) as usize).await;

                  bot.ecs.lock().trigger(SwingArmEvent { entity: bot.entity });

                  bot.block_interact(pos);  

                  if count == 10 {
                    bot.set_crouching(false);
                  }
                  
                  bot.wait_ticks(randuint(2, 3) as usize).await;
                }
              });
            }
          },
          "unite" => {
            let mut positions = vec![];

            for bot in fm.bots.clone().into_values() {
              positions.push(bot.position());
            }

            let average_cords = Self::get_average_coordinates(positions);

            for bot in fm.bots.clone().into_values() {
              tokio::task::spawn(async move {
                bot.start_goto_with_opts(
                  XZGoal { x: average_cords.0 as i32, z: average_cords.2 as i32 },  
                  PathfinderOpts::new()  
                    .min_timeout(PathfinderTimeout::Time(time::Duration::from_millis(300)))  
                    .max_timeout(PathfinderTimeout::Time(time::Duration::from_millis(1000)))  
                    .allow_mining(false)  
                    .successors_fn(moves::basic::basic_move)  
                );
              });
            }
          },
          "turn" => {
            for bot in fm.bots.clone().into_values() {
              let direction = bot.direction();
              bot.set_direction(direction.0 - 90.0, direction.1);
            }
          },
          "zero" => {
            for bot in fm.bots.clone().into_values() {
              bot.set_direction(0.0, 0.0);
            }
          },
          "form-circle" => {
            let mut positions = vec![];
            let mut bots = vec![];

            for bot in fm.bots.clone().into_values() {
              positions.push(bot.position());
              bots.push(bot);
            }

            let average_cords = Self::get_average_coordinates(positions);
            
            for (i, bot) in bots.iter().enumerate() {  
              let angle = 2.0 * PI * (i as f32) / (bots.len() as f32);  
              let x = average_cords.0 + 6.0 * angle.cos() as f64;  
              let z = average_cords.2 + 6.0 * angle.sin() as f64;  

              bot.start_goto_with_opts(
                XZGoal { x: x as i32, z: z as i32 },  
                PathfinderOpts::new()  
                  .min_timeout(PathfinderTimeout::Time(time::Duration::from_millis(300)))  
                  .max_timeout(PathfinderTimeout::Time(time::Duration::from_millis(1000)))  
                  .allow_mining(false)  
                  .successors_fn(moves::basic::basic_move) 
              );
            }  
          },
          "form-line" => { 
            let mut positions = vec![];
            let mut bots = vec![];

            for bot in fm.bots.clone().into_values() {
              positions.push(bot.position());
              bots.push(bot);
            }

            let average_cords = Self::get_average_coordinates(positions);

            for (i, bot) in bots.iter().enumerate() {  
              let x = average_cords.0 + 1.0 * (i as f64 * 1.0);  
              let z = average_cords.2 + 0.0 * (i as f64 * 1.0);  
                      
              bot.start_goto_with_opts(
                XZGoal { x: x as i32, z: z as i32 },  
                PathfinderOpts::new()  
                  .min_timeout(PathfinderTimeout::Time(time::Duration::from_millis(300)))  
                  .max_timeout(PathfinderTimeout::Time(time::Duration::from_millis(1000)))  
                  .allow_mining(false)  
                  .successors_fn(moves::basic::basic_move)  
              );
            }  
          },
          "pathfinder-stop" => {
            for bot in fm.bots.clone().into_values() {
              bot.stop_pathfinding();
            }
          },
          _ => {}
        }
      }
    }
  }

  fn get_average_coordinates(positions: Vec<Vec3>) -> (f64, f64, f64) {
    let mut x_coords = vec![];
    let mut y_coords = vec![];
    let mut z_coords = vec![];

    for pos in positions {
      x_coords.push(pos.x);
      y_coords.push(pos.x);
      z_coords.push(pos.z);
    }

    let mut x_global = 0.0;
    let mut y_global = 0.0;
    let mut z_global = 0.0;

    for coordinate in x_coords.clone().into_iter() {
      x_global = x_global + coordinate;
    }

    for coordinate in y_coords.clone().into_iter() {
      y_global = y_global + coordinate;
    }

    for coordinate in z_coords.clone().into_iter() {
      z_global = z_global + coordinate;
    }

    let x_average = x_global / x_coords.len() as f64;
    let y_average = y_global / y_coords.len() as f64;
    let z_average = z_global / z_coords.len() as f64;

    (x_average, y_average, z_average)
  }
}