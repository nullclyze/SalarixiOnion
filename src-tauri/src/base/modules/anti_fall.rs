use azalea::prelude::*;
use azalea::Vec3;
use azalea::core::position::BlockPos; 
use azalea::protocol::common::movements::MoveFlags;
use azalea::protocol::packets::game::ServerboundMovePlayerPos;
use serde::{Serialize, Deserialize};
use std::time::{Duration, Instant};

use crate::TASKS;
use crate::tools::*;
use crate::common::{get_block_state, get_bot_physics, set_bot_velocity_y, set_bot_on_ground};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiFallModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiFallOptions {
  pub mode: String,
  pub distance_to_ground: Option<i32>,
  pub fall_velocity: Option<f64>,
  pub delay: Option<usize>,
  pub state: bool
}

impl AntiFallModule {
  fn exist_blocks_below(bot: &Client, distance_to_ground: i32) -> bool {
    let pos = bot.position();

    for y in 0..=distance_to_ground {
      let block_pos = BlockPos::new(pos.x.floor() as i32, (pos.y.floor() as i32) - y, pos.z.floor() as i32);

      if let Some(state) = get_block_state(bot, block_pos) {
        if !state.is_air() {
          return true;
        }
      }
    }

    false
  }

  async fn hovering_anti_fall(bot: &Client, options: AntiFallOptions) {
    loop {
      let velocity_y = get_bot_physics(bot).velocity.y;

      if velocity_y < options.fall_velocity.unwrap_or(-0.5) {
        if Self::exist_blocks_below(bot, options.distance_to_ground.unwrap_or(4)) {
          let duration = Instant::now() + Duration::from_millis(randuint(300, 450));

          loop {
            if Instant::now() >= duration {
              break;
            }

            set_bot_velocity_y(bot, 0.001);
            set_bot_on_ground(bot, true);

            bot.wait_ticks(1).await;

            set_bot_on_ground(bot, false);
          }
        }
      }

      bot.wait_ticks(options.delay.unwrap_or(1)).await;
    }
  }

  async fn teleport_anti_fall(bot: &Client, options: AntiFallOptions) {
    loop {
      let velocity_y = get_bot_physics(bot).velocity.y;

      if velocity_y < options.fall_velocity.unwrap_or(-0.5) {
        if Self::exist_blocks_below(bot, options.distance_to_ground.unwrap_or(4)) {
          let pos = bot.position();
          let fake_pos = Vec3::new(pos.x, pos.y + randfloat(0.015, 0.022), pos.z);

          let physics = get_bot_physics(bot);

          let packet = ServerboundMovePlayerPos {
            pos: fake_pos,
            flags: MoveFlags {
              on_ground: physics.on_ground(),
              horizontal_collision: physics.horizontal_collision
            }
          };

          bot.write_packet(packet);
        }
      }

      bot.wait_ticks(options.delay.unwrap_or(1)).await;
    }
  }

  pub async fn enable(bot: &Client, options: AntiFallOptions) {
    match options.mode.as_str() {
      "hovering" => { Self::hovering_anti_fall(bot, options).await; },
      "teleport" => { Self::teleport_anti_fall(bot, options).await; },
      _ => {}
    }
  }

  pub fn stop(nickname: &String) {
    TASKS.get(nickname).unwrap().write().unwrap().stop_task("anti-fall");
  }
}