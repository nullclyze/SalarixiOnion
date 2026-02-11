use azalea::prelude::*;
use azalea::WalkDirection;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::common::{go, go_to};
use crate::tools::*;

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

  pub async fn enable(&self, bot: &Client, options: &MovementOptions) {
    match options.mode.as_str() {
      "default" => match options.use_impulsiveness {
        true => {
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
        }
        false => {
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
      },
      "pathfinder" => {
        if let Some(x) = options.x {
          if let Some(z) = options.z {
            if !options.use_sync {
              sleep(Duration::from_millis(randuint(500, 2000))).await;
            }

            go_to(bot.clone(), x, z);
          }
        }
      }
      _ => {}
    }
  }

  pub fn stop(&self, bot: &Client) {
    let nickname = bot.username();

    kill_task(&nickname, "movement");

    bot.walk(WalkDirection::None);
    bot.stop_pathfinding();

    STATES.set_mutual_states(&nickname, "walking", false);
    STATES.set_mutual_states(&nickname, "sprinting", false);
  }
}
