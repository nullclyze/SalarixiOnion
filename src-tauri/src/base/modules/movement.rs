use azalea::prelude::*;
use azalea::WalkDirection;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::common::{go, go_to};
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
        match options.direction.as_str() {
          "forward" => {
            go(bot, WalkDirection::Forward);
          }
          "backward" => {
            go(bot, WalkDirection::Backward);
          }
          "left" => {
            go(bot, WalkDirection::Left);
          }
          "right" => {
            go(bot, WalkDirection::Right);
          }
          _ => {}
        }

        if options.use_sync {
          sleep(Duration::from_millis(1200)).await;
        } else {
          sleep(Duration::from_millis(randuint(800, 1800))).await;
        }

        bot.walk(WalkDirection::None);

        STATES.set_mutual_states(&bot.username(), "walking", false);
      }
    } else {
      if !options.use_sync {
        sleep(Duration::from_millis(randuint(500, 2000))).await;
      }

      match options.direction.as_str() {
        "forward" => {
          go(bot, WalkDirection::Forward);
        }
        "backward" => {
          go(bot, WalkDirection::Backward);
        }
        "left" => {
          go(bot, WalkDirection::Left);
        }
        "right" => {
          go(bot, WalkDirection::Right);
        }
        _ => {}
      }
    }
  }

  async fn pathfinder_move(&self, bot: &Client, options: &MovementOptions) {
    if let Some(x) = options.x {
      if let Some(z) = options.z {
        if !options.use_sync {
          sleep(Duration::from_millis(randuint(500, 2000))).await;
        }

        go_to(bot.clone(), x, z);
      }
    }
  }

  pub async fn enable(&self, username: &str, options: &MovementOptions) {
    BOT_REGISTRY
      .get_bot(username, async |bot| match options.mode.as_str() {
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

    BOT_REGISTRY
      .get_bot(username, async |bot| {
        bot.walk(WalkDirection::None);
        bot.stop_pathfinding();

        STATES.set_mutual_states(username, "walking", false);
        STATES.set_mutual_states(username, "sprinting", false);
      })
      .await;
  }
}
