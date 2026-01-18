use azalea::prelude::*;
use azalea::Vec3;
use azalea::WalkDirection;
use azalea::auto_tool::AutoToolClientExt;
use azalea::block::BlockState;
use azalea::core::position::BlockPos;
use azalea::pathfinder::goals::XZGoal;  
use azalea::pathfinder::PathfinderOpts;
use azalea::pathfinder::astar::PathfinderTimeout;
use azalea::pathfinder::moves;
use azalea::auto_tool::best_tool_in_hotbar_for_block;
use serde::{Serialize, Deserialize};
use std::time::Duration;
use tokio::time::sleep;

use crate::TASKS;
use crate::tools::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinerModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinerOptions {
  pub mode: String,
  pub block: String,
  pub direction_x: Option<f32>,
  pub slot: Option<u8>,
  pub delay: Option<usize>,
  pub state: bool
}

impl MinerModule {
  fn get_block_state(bot: &Client, block_pos: BlockPos) -> Option<BlockState> {
    let world_clone = bot.world().clone();
    let world = world_clone.write();

    if let Some(state) = world.get_block_state(block_pos) {
      return Some(state);
    }

    None
  }

  fn is_breakable_block(state: BlockState) -> bool {
    let unbreakable_blocks = vec![
      86, 88, 87, 89, 94, 0,
      110, 104, 106, 102, 108
    ];

    for id in unbreakable_blocks {
      if state.id() == id {
        return false;
      }
    }

    true
  }

  fn check_block(block: String, block_id: u16) -> bool {
    let mut id_list = vec![];

    match block.as_str() {
      "stone" => {
        id_list = vec![1, 27722, 21629];
      },
      "wood" => {
        id_list = vec![
          140, 160, 141, 155,
          151, 20760, 153, 148,
          142, 136, 20745, 164,
          154, 146, 162, 137
        ];
      },
      "ore" => {
        id_list = vec![
          25111, 131, 133, 29377, 
          29376, 5106, 5107, 29375,
          6681, 563, 564, 11110, 
          9372, 6683, 130, 129,
          134, 132, 135, 9373
        ];
      },
      "sand" => {
        id_list = vec![118];
      },
      "sugarcane" => {
        id_list = vec![6748];
      },
      _ => {}
    }

    for id in id_list {
      if block_id == id {
        return true;
      }
    }

    false
  }

  fn find_block(bot: &Client, block: String) -> Option<BlockPos> {
    let center = bot.position();

    let cords_x = vec![-5..5, -25..25, -50..50];
    let cords_y = vec![0..0, 0..50, -50..50];
    let cords_z = vec![-5..5, -25..25, -50..50];

    let mut positions = vec![];
    
    for x_range in cords_x.clone() {
      for y_range in cords_y.clone() {
        for z_range in cords_z.clone() {
          for x in x_range.clone() {  
            for y in y_range.clone() {
              for z in z_range.clone() {  
                let block_pos = BlockPos::new(  
                  center.x as i32 + x,  
                  center.y as i32 + y,  
                  center.z as i32 + z
                );  
                            
                if let Some(state) = Self::get_block_state(bot, block_pos) {  
                  if !state.is_air() && Self::is_breakable_block(state) {
                    if Self::check_block(block.clone(), state.id()) {
                      positions.push(block_pos);
                    }
                  }
                }
              }  
            }
          }
        }
      }
    }

    let mut nearest = None;

    for pos in positions {
      if nearest.is_none() {
        nearest = Some(pos);
      } else {
        if let Some(n) = nearest {
          if (center.x.floor() as i32) - pos.x < (center.x.floor() as i32) - n.x {
            if (center.y.floor() as i32) - pos.y < (center.y.floor() as i32) - n.y {
              if (center.z.floor() as i32) - pos.z < (center.z.floor() as i32) - n.z {
                nearest = Some(pos);
              }
            }
          }
        }
      }
    }

    nearest
  } 

  fn check_adjacent_block(bot: &Client, block_pos: BlockPos) -> bool {
    let adjacent_block = BlockPos::new(block_pos.x, block_pos.y + 1, block_pos.z);

    if let Some(state) = Self::get_block_state(bot, adjacent_block) {
      if !state.is_air() && Self::is_breakable_block(state) {
        return false;
      }
    }

    true
  }

  fn can_reach_block(eye_pos: Vec3, block_pos: BlockPos) -> bool {
    if eye_pos.distance_to(Vec3::new(block_pos.x as f64,  block_pos.y as f64, block_pos.z as f64)) < 4.5 {
      return true;
    }

    false
  }

  async fn look_at_block(bot: &Client, block_pos: BlockPos) {
    let correct_position = Vec3::new(
      block_pos.x as f64,
      block_pos.y as f64,
      block_pos.z as f64
    );

    bot.look_at(correct_position);
  }

  async fn move_forward(bot: &Client) {
    let position = bot.position();

    let x = position.x.floor() as i32;
    let y = position.y.floor() as i32;
    let z = position.z.floor() as i32;

    let nearest_blocks = [
      BlockPos::new(x + 1, y, z),
      BlockPos::new(x - 1, y, z),
      BlockPos::new(x, y, z + 1),
      BlockPos::new(x, y, z - 1)
    ];

    let mut existing_blocks = 0;

    for block_pos in nearest_blocks {
      if let Some(state) = Self::get_block_state(bot, block_pos) {
        if !state.is_air() && Self::is_breakable_block(state) {
          existing_blocks += 1;
        }
      }
    }

    if existing_blocks == 0 {
      bot.walk(WalkDirection::Forward);
      sleep(Duration::from_millis(randuint(50, 80))).await;
      bot.walk(WalkDirection::None);
    }
  }

  async fn manual_mine(bot: &Client, options: MinerOptions) {
    loop {
      let position = bot.position();

      let x = position.x.floor() as i32;
      let y = position.y.floor() as i32;
      let z = position.z.floor() as i32;

      let nearest_blocks = [
        BlockPos::new(x, y, z),
        BlockPos::new(x + 1, y, z),
        BlockPos::new(x - 1, y, z),
        BlockPos::new(x, y, z + 1),
        BlockPos::new(x, y, z - 1)
      ];

      for pos in nearest_blocks {
        for height in [2, 1, 0] {
          let block_pos = BlockPos::new(pos.x, pos.y + height, pos.z);
          
          if let Some(state) = Self::get_block_state(bot, block_pos) {
            if !state.is_air() && Self::is_breakable_block(state) && Self::can_reach_block(bot.eye_position(), block_pos) {
              Self::look_at_block(bot, block_pos).await;

              sleep(Duration::from_millis(randuint(150, 230))).await;

              if let Some(slot) = options.slot {
                bot.set_selected_hotbar_slot(slot);
                bot.mine(block_pos).await;
              } else {
                if let Some(menu) = &bot.get_inventory().menu() {
                  let best_tool = best_tool_in_hotbar_for_block(state, menu).index;
                  bot.set_selected_hotbar_slot(best_tool as u8);

                  bot.mine(block_pos).await;
                }
              }

              sleep(Duration::from_millis(randuint(50, 150))).await;

              if let Some(s) = Self::get_block_state(bot, block_pos) {
                if !s.is_air() {
                  bot.walk(WalkDirection::Backward);
                  sleep(Duration::from_millis(randuint(50, 80))).await;
                  bot.walk(WalkDirection::None);
                }
              }
            }
          }
        }
      }

      let direction = bot.direction();
      bot.set_direction(options.direction_x.unwrap_or(0.0), direction.1);

      sleep(Duration::from_millis(randuint(50, 150))).await;

      Self::move_forward(bot).await;

      bot.wait_ticks(options.delay.unwrap_or(2)).await;
    }
  }

  async fn auto_mine(bot: &Client, options: MinerOptions) {
    loop {
      let pos = Self::find_block(bot, options.block.clone());

      if let Some(block_pos) = pos {
        bot.goto_with_opts(
          XZGoal { x: block_pos.x - 1, z: block_pos.z - 1 },  
          PathfinderOpts::new()  
            .min_timeout(PathfinderTimeout::Time(Duration::from_millis(300)))  
            .max_timeout(PathfinderTimeout::Time(Duration::from_millis(1000)))  
            .allow_mining(false)  
            .successors_fn(moves::basic::basic_move)
        ).await;

        bot.wait_ticks(randuint(1, 2) as usize).await;

        Self::look_at_block(bot, block_pos).await;

        bot.wait_ticks(randuint(1, 2) as usize).await;

        if let Some(slot) = options.slot {
          bot.set_selected_hotbar_slot(slot);
          bot.mine(block_pos).await;
        } else {
          if let Some(state) = Self::get_block_state(bot, block_pos) {
            if let Some(menu) = bot.get_inventory().menu() {
              let best_tool = best_tool_in_hotbar_for_block(state, &menu).index;
              bot.set_selected_hotbar_slot(best_tool as u8);
                
              bot.mine(block_pos).await;
            }
          }
        }

        if Self::check_adjacent_block(bot, block_pos) {
          bot.goto_with_opts(
            XZGoal { x: block_pos.x, z: block_pos.z },  
            PathfinderOpts::new()  
              .min_timeout(PathfinderTimeout::Time(Duration::from_millis(300)))  
              .max_timeout(PathfinderTimeout::Time(Duration::from_millis(1000)))  
              .allow_mining(false)  
              .successors_fn(moves::basic::basic_move)
          ).await;
        } else {
          if let Some(slot) = options.slot {
            bot.set_selected_hotbar_slot(slot);
            bot.mine(BlockPos::new(block_pos.x, block_pos.y + 1, block_pos.z)).await;
          } else {
            bot.mine_with_auto_tool(BlockPos::new(block_pos.x, block_pos.y + 1, block_pos.z)).await;
          }

          bot.goto_with_opts(
            XZGoal { x: block_pos.x, z: block_pos.z },  
            PathfinderOpts::new()  
              .min_timeout(PathfinderTimeout::Time(Duration::from_millis(300)))  
              .max_timeout(PathfinderTimeout::Time(Duration::from_millis(1000)))  
              .allow_mining(false)  
              .successors_fn(moves::basic::basic_move)
          ).await;
        }
      }

      bot.wait_ticks(options.delay.unwrap_or(10)).await;
    }
  } 

  pub async fn enable(bot: &Client, options: MinerOptions) {
    match options.mode.as_str() {
      "manual" => { Self::manual_mine(bot, options).await; },
      "auto" => { Self::auto_mine(bot, options).await; },
      _ => {}
    }
  } 

  pub fn stop(bot: &Client) {
    TASKS.get(&bot.username()).unwrap().write().unwrap().stop_task("miner");
    bot.stop_pathfinding();
    bot.walk(WalkDirection::None);
  }
}