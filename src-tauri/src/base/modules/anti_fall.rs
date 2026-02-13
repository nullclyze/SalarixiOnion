use azalea::core::position::BlockPos;
use azalea::prelude::*;
use azalea::protocol::common::movements::MoveFlags;
use azalea::protocol::packets::game::s_interact::InteractionHand;
use azalea::protocol::packets::game::ServerboundMovePlayerPos;
use azalea::protocol::packets::game::ServerboundUseItem;
use azalea::registry::builtin::ItemKind;
use azalea::Vec3;
use serde::{Deserialize, Serialize};
use std::time::{Duration, Instant};
use tokio::time::sleep;

use crate::base::*;
use crate::common::take_item;
use crate::common::{get_block_state, get_bot_physics, set_bot_on_ground, set_bot_velocity_y};
use crate::tools::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiFallModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiFallOptions {
  pub mode: String,
  pub distance_to_ground: Option<i32>,
  pub fall_velocity: Option<f64>,
  pub delay: Option<u64>,
  pub state: bool,
}

impl AntiFallModule {
  pub fn new() -> Self {
    Self
  }

  fn exist_blocks_below(&self, bot: &Client, distance_to_ground: i32) -> bool {
    let pos = bot.position();

    for y in 0..=distance_to_ground {
      let block_pos = BlockPos::new(
        pos.x.floor() as i32,
        (pos.y.floor() as i32) - y,
        pos.z.floor() as i32,
      );

      if let Some(state) = get_block_state(bot, block_pos) {
        if !state.is_air() {
          return true;
        }
      }
    }

    false
  }

  fn exist_water_below(&self, bot: &Client, distance_to_ground: i32) -> bool {
    let pos = bot.position();

    for y in 0..=distance_to_ground {
      let block_pos = BlockPos::new(
        pos.x.floor() as i32,
        (pos.y.floor() as i32) - y,
        pos.z.floor() as i32,
      );

      if let Some(state) = get_block_state(bot, block_pos) {
        if !state.is_air() {
          if state.id() == 86 {
            return true;
          }
        }
      }
    }

    false
  }

  async fn take_water_bucket(&self, bot: &Client) -> bool {
    let menu = bot.menu();

    for (slot, item) in menu.slots().iter().enumerate() {
      if !item.is_empty() {
        if item.kind() == ItemKind::WaterBucket {
          take_item(bot, slot, true).await;
          return true;
        }
      }
    }

    false
  }

  async fn hovering_anti_fall(&self, bot: &Client, options: &AntiFallOptions) {
    loop {
      let velocity_y = if let Some(physics) = get_bot_physics(bot) {
        physics.velocity.y
      } else {
        0.0
      };

      if velocity_y < options.fall_velocity.unwrap_or(-0.5) {
        if self.exist_blocks_below(bot, options.distance_to_ground.unwrap_or(4)) {
          let duration = Instant::now() + Duration::from_millis(randuint(300, 450));

          loop {
            if Instant::now() >= duration {
              break;
            }

            set_bot_velocity_y(bot, 0.001);
            set_bot_on_ground(bot, true);

            sleep(Duration::from_millis(50)).await;

            set_bot_on_ground(bot, false);
          }
        }
      }

      sleep(Duration::from_millis(options.delay.unwrap_or(50))).await;
    }
  }

  async fn teleport_anti_fall(&self, bot: &Client, options: &AntiFallOptions) {
    loop {
      if let Some(physics) = get_bot_physics(bot) {
        let velocity_y = physics.velocity.y;

        if velocity_y < options.fall_velocity.unwrap_or(-0.5) {
          if self.exist_blocks_below(bot, options.distance_to_ground.unwrap_or(4)) {
            let pos = bot.position();
            let fake_pos = Vec3::new(pos.x, pos.y + randfloat(0.015, 0.022), pos.z);

            bot.write_packet(ServerboundMovePlayerPos {
              pos: fake_pos,
              flags: MoveFlags {
                on_ground: physics.on_ground(),
                horizontal_collision: physics.horizontal_collision,
              },
            });
          }
        }
      }

      sleep(Duration::from_millis(options.delay.unwrap_or(50))).await;
    }
  }

  async fn water_drop_anti_fall(&self, bot: &Client, options: &AntiFallOptions) {
    let distance_to_ground = options.distance_to_ground.unwrap_or(4);

    loop {
      let velocity_y = if let Some(physics) = get_bot_physics(bot) {
        physics.velocity.y
      } else {
        0.0
      };

      if velocity_y < options.fall_velocity.unwrap_or(-0.5) {
        if self.exist_blocks_below(bot, distance_to_ground) {
          let y_rot = bot.direction().0;

          bot.set_direction(y_rot, randfloat(85.0, 89.0) as f32);

          if self.take_water_bucket(bot).await {
            if !self.exist_water_below(bot, distance_to_ground) {
              sleep(Duration::from_millis(50)).await;

              let direction = bot.direction();

              bot.write_packet(ServerboundUseItem {
                hand: InteractionHand::MainHand,
                seq: 0,
                y_rot: direction.0,
                x_rot: direction.1,
              });
            }
          }

          sleep(Duration::from_millis(50)).await;

          if self.exist_water_below(bot, distance_to_ground) {
            let direction = bot.direction();

            bot.write_packet(ServerboundUseItem {
              hand: InteractionHand::MainHand,
              seq: 0,
              y_rot: direction.0,
              x_rot: direction.1,
            });
          }
        }
      }

      sleep(Duration::from_millis(options.delay.unwrap_or(50))).await;
    }
  }

  pub async fn enable(&self, bot: &Client, options: &AntiFallOptions) {
    match options.mode.as_str() {
      "hovering" => {
        self.hovering_anti_fall(bot, options).await;
      }
      "teleport" => {
        self.teleport_anti_fall(bot, options).await;
      }
      "water-drop" => {
        self.water_drop_anti_fall(bot, options).await;
      }
      _ => {}
    }
  }

  pub fn stop(&self, nickname: &String) {
    kill_task(nickname, "anti-fall");
  }
}
