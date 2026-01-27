use azalea::prelude::*;
use azalea::Vec3;
use azalea::WalkDirection;
use azalea::block::BlockState;
use azalea::core::position::BlockPos;
use azalea::auto_tool::best_tool_in_hotbar_for_block;
use serde::{Serialize, Deserialize};
use std::time::Duration;
use tokio::time::sleep;

use crate::TASKS;
use crate::tools::*;
use crate::common::get_block_state;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinerModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinerOptions {
  pub mode: String,
  pub tunnel: String,
  pub look: String,
  pub direction_x: Option<f32>,
  pub slot: Option<u8>,
  pub delay: Option<usize>,
  pub state: bool
}

impl MinerModule {
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

  fn can_reach_block(eye_pos: Vec3, block_pos: BlockPos) -> bool {
    if eye_pos.distance_to(Vec3::new(block_pos.x as f64,  block_pos.y as f64, block_pos.z as f64)) < 4.5 {
      return true;
    }

    false
  }

  async fn micro_offset(bot: &Client) {
    if randchance(0.8) {
      let walk_directions = vec![
        WalkDirection::Left, WalkDirection::Right, 
        WalkDirection::ForwardLeft, WalkDirection::ForwardRight,
        WalkDirection::BackwardLeft, WalkDirection::BackwardRight
      ];

      let walk_direction = randelem(walk_directions.as_ref());

      if let Some(dir) = walk_direction {
        bot.walk(*dir);
        bot.wait_ticks(randticks(3, 5)).await;
        bot.walk(WalkDirection::None);
      }
    }
  }

  async fn look_at_block(bot: &Client, block_pos: BlockPos, look: String) {
    let mut center = block_pos.center();

    if look.as_str() == "smooth" {
      let pre = Vec3::new(
        (block_pos.x as f64) + randfloat(-0.05, 0.05),
        (block_pos.y as f64) + randfloat(-0.05, 0.05),
        (block_pos.z as f64) + randfloat(-0.05, 0.05)
      );

      bot.look_at(pre);

      bot.wait_ticks(randticks(1, 3)).await;
    }

    if randchance(0.7) {
      center.x += randfloat(-0.03, 0.03);
    }

    if randchance(0.7) {
      center.y += randfloat(-0.03, 0.03);
    }

    if randchance(0.7) {
      center.z += randfloat(-0.03, 0.03);
    }

    bot.look_at(center);
  }

  async fn move_forward(bot: &Client, territory: Vec<BlockPos>) {
    for block_pos in territory {
      if let Some(state) = get_block_state(bot, block_pos) {
        if !state.is_air() && Self::is_breakable_block(state) {
          return;
        }
      }
    }

    bot.walk(WalkDirection::Forward);
    bot.wait_ticks(randticks(1, 3)).await;
    bot.walk(WalkDirection::None);
  }

  fn get_territory(pos: Vec3, tunnel: String) -> Vec<BlockPos> {
    let x = pos.x.floor() as i32;
    let y = pos.y.floor() as i32;
    let z = pos.z.floor() as i32;

    let territory;

    match tunnel.as_str() {
      "2x2x2" => {
        territory = vec![
          BlockPos::new(x + 1, y, z),
          BlockPos::new(x - 1, y, z),
          BlockPos::new(x, y, z + 1),
          BlockPos::new(x, y, z - 1)
        ];
      },
      "3x2x2" => {
        territory = vec![
          BlockPos::new(x + 1, y, z),
          BlockPos::new(x - 1, y, z),
          BlockPos::new(x, y, z + 1),
          BlockPos::new(x, y, z - 1)
        ];
      },
      "2x3x3" => {
        territory = vec![
          BlockPos::new(x + 1, y, z),
          BlockPos::new(x - 1, y, z),
          BlockPos::new(x, y, z + 1),
          BlockPos::new(x, y, z - 1),
          BlockPos::new(x + 2, y, z),
          BlockPos::new(x - 2, y, z),
          BlockPos::new(x, y, z + 2),
          BlockPos::new(x, y, z - 2)
        ];
      },
      _ => {
        territory = vec![
          BlockPos::new(x + 1, y, z),
          BlockPos::new(x - 1, y, z),
          BlockPos::new(x, y, z + 1),
          BlockPos::new(x, y, z - 1),
          BlockPos::new(x + 2, y, z),
          BlockPos::new(x - 2, y, z),
          BlockPos::new(x, y, z + 2),
          BlockPos::new(x, y, z - 2),
          BlockPos::new(x + 3, y, z),
          BlockPos::new(x - 3, y, z),
          BlockPos::new(x, y, z + 3),
          BlockPos::new(x, y, z - 3)
        ];
      }
    };

    territory
  }

  async fn default_mine(bot: &Client, options: MinerOptions) {
    bot.left_click_mine(true);
    bot.walk(WalkDirection::Forward);
    bot.set_direction(options.direction_x.unwrap_or(0.0), 40.0 + randfloat(-3.5, 3.5) as f32);
  }

  async fn extended_mine(bot: &Client, options: MinerOptions) {
    loop {
      let position = bot.position();

      let territory = Self::get_territory(position, options.tunnel.clone());

      for pos in territory.clone() {
        let heights = match options.tunnel.as_str() {
          "3x2x2" => vec![2, 1, 0],
          "3x3x3" => vec![2, 1, 0],
          _ => vec![1, 0]
        };

        for height in heights {
          let block_pos = BlockPos::new(pos.x, pos.y + height, pos.z);
          
          if let Some(state) = get_block_state(bot, block_pos) {
            if !state.is_air() && Self::is_breakable_block(state) && Self::can_reach_block(bot.eye_position(), block_pos) {
              let should_shift = randchance(0.2);

              if should_shift {
                bot.set_crouching(true);
              }
              
              Self::look_at_block(bot, block_pos, options.look.clone()).await;

              bot.wait_ticks(randticks(2, 4)).await;

              if let Some(slot) = options.slot {
                bot.set_selected_hotbar_slot(slot);
              } else {
                if let Some(menu) = &bot.get_inventory().menu() {
                  let best_tool = best_tool_in_hotbar_for_block(state, menu).index;
                  bot.set_selected_hotbar_slot(best_tool as u8);
                }
              }

              if should_shift {
                bot.set_crouching(false);
              }

              Self::micro_offset(bot).await;

              bot.start_mining(block_pos);

              loop {
                if let Some(s) = get_block_state(bot, block_pos) {
                  if state.is_air() || !Self::is_breakable_block(s) {
                    break;
                  }
                } else {
                  break;
                }

                sleep(Duration::from_millis(50)).await;
              }

              Self::micro_offset(bot).await;

              bot.wait_ticks(randticks(1, 2)).await;

              sleep(Duration::from_millis(randuint(50, 100))).await;

              if let Some(s) = get_block_state(bot, block_pos) {
                if !s.is_air() || Self::is_breakable_block(s) {
                  bot.walk(WalkDirection::Backward);
                  bot.wait_ticks(randticks(1, 2)).await;
                  bot.walk(WalkDirection::None);
                }
              }
            }
          }
        }
      }

      let direction = bot.direction();

      if randchance(0.5) {
        bot.set_direction(options.direction_x.unwrap_or(0.0) + randfloat(-2.5, 2.5) as f32, direction.1 + randfloat(-2.5, 2.5) as f32);
      } else {
        if randchance(0.3) {
          bot.set_direction(options.direction_x.unwrap_or(0.0) + randfloat(-1.3, 1.3) as f32, direction.1 + randfloat(-1.3, 1.3) as f32);
        } else {
          bot.set_direction(options.direction_x.unwrap_or(0.0), direction.1);
        }
      }

      sleep(Duration::from_millis(randuint(50, 100))).await;

      Self::move_forward(bot, territory).await;

      bot.wait_ticks(options.delay.unwrap_or(2)).await;
    }
  } 

  pub async fn enable(bot: &Client, options: MinerOptions) {
    match options.mode.as_str() {
      "default" => { Self::default_mine(bot, options).await; },
      "extended" => { Self::extended_mine(bot, options).await; }
      _ => {}
    }
  } 

  pub fn stop(bot: &Client) {
    TASKS.get(&bot.username()).unwrap().write().unwrap().stop_task("miner");
    bot.left_click_mine(false);
    bot.walk(WalkDirection::None);
    bot.set_crouching(false);
  }
}