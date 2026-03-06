use azalea::prelude::*;
use azalea::WalkDirection;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

use crate::core::*;
use crate::extensions::{go_to, BotMovementExt};
use crate::generators::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MovementOptions {
  pub mode: String,
  pub direction: String,
  pub use_sync: bool,
  pub use_impulsiveness: bool,
  pub x: Option<i32>,
  pub z: Option<i32>,
  pub state: bool,
}

impl MovementModule {
  pub fn new() -> Self {
    Self
  }

  async fn default_move(&self, bot: &Client, options: &MovementOptions) {
    if options.use_impulsiveness {
      if !options.use_sync {
        sleep(Duration::from_millis(randuint(500, 2000))).await;
      }

      loop {
        let direction = match options.direction.as_str() {
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

        bot.start_walking(direction);

        if options.use_sync {
          sleep(Duration::from_millis(1200)).await;
        } else {
          sleep(Duration::from_millis(randuint(800, 1800))).await;
        }

        bot.stop_move();
      }
    } else {
      if !options.use_sync {
        sleep(Duration::from_millis(randuint(500, 2000))).await;
      }

      let direction = match options.direction.as_str() {
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

      bot.start_walking(direction);
    }
  }

  async fn pathfinder_move(&self, bot: &Client, options: &MovementOptions) {
    if let Some(x) = options.x {
      if let Some(z) = options.z {
        if !options.use_sync {
          sleep(Duration::from_millis(randuint(500, 2000))).await;
        }

        go_to(bot.username(), x, z);
      }
    }
  }

  pub async fn enable(&self, username: &str, options: &MovementOptions) {
    BOT_REGISTRY
      .async_get_bot(username, async |bot| match options.mode.as_str() {
        "default" => {
          self.default_move(bot, options).await;
        }
        "pathfinder" => {
          self.pathfinder_move(bot, options).await;
        }
        _ => {}
      })
      .await;
  }

  pub async fn stop(&self, username: &str) {
    kill_task(username, "movement");

    if let Some(bot) = BOT_REGISTRY.get_bot(username) {
      bot.stop_pathfinding();
      bot.stop_move();
    }
  }
}
