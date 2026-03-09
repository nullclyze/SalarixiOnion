use azalea::{prelude::*, SprintDirection};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

use crate::core::*;
use crate::extensions::{BotDefaultExt, BotMovementExt, EntityType, go_to};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StalkerModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StalkerOptions {
  pub mode: String,
  pub target_nickname: Option<String>,
  pub min_distance: Option<f64>,
  pub max_distance: Option<f64>,
  pub state: bool,
}

impl StalkerModule {
  pub fn new() -> Self {
    Self
  }

  async fn default_stalking(&self, bot: &Client, options: &StalkerOptions) {
    let username = bot.username();

    loop {
      let Some(target_nickname) = options.target_nickname.clone() else {
        return;
      };

      if let Some(target) = bot.find_nearest_entity(EntityType::Custom(target_nickname), options.max_distance.unwrap_or(100.0)) {
        let target_pos = bot.get_entity_position(target);
        let min_distance = options.min_distance.unwrap_or(6.0);

        if bot.feet_pos().distance_to(target_pos) > min_distance {
          bot.look_at(target_pos);
          bot.start_jumping();
          bot.start_sprinting(SprintDirection::Forward);
        } else {
          bot.stop_jumping();

          if get_state(&username, "is_sprinting") {
            bot.stop_move();
          }
        }
      } else {
        bot.stop_jumping();

        if get_state(&username, "is_sprinting") {
          bot.stop_move();
        }
      }

      sleep(Duration::from_millis(50)).await;
    }
  }

  async fn advanced_stalking(&self, bot: &Client, options: &StalkerOptions) {
    let Some(target_nickname) = options.target_nickname.clone() else {
      return;
    };

    loop {
      if let Some(target) = bot.find_nearest_entity(EntityType::Custom(target_nickname.clone()), options.max_distance.unwrap_or(100.0)) {
        let target_pos = bot.get_entity_position(target);
        let min_distance = options.min_distance.unwrap_or(6.0);

        if bot.feet_pos().distance_to(target_pos) > min_distance {
          if bot.is_goto_target_reached() {
            go_to(
              bot.username(),
              (target_pos.x - min_distance) as i32,
              (target_pos.z - min_distance) as i32,
            );
          }
        }
      }

      sleep(Duration::from_millis(200)).await;
    }
  }

  async fn stalking(&self, bot: &Client, options: &StalkerOptions) {
    let username = bot.username();

    set_mutual_states(&username, "looking", true);

    match options.mode.as_str() {
      "default" => {
        self.default_stalking(bot, options).await;
      }
      "advanced" => {
        self.advanced_stalking(bot, options).await;
      }
      _ => {}
    }
  }

  pub async fn enable(&self, username: &str, options: &StalkerOptions) {
    BOT_REGISTRY
      .async_get_bot(username, async |bot| {
        self.stalking(bot, options).await;
      })
      .await;
  }

  pub fn stop(&self, username: &str) {
    kill_task(username, "stalker");

    if let Some(bot) = BOT_REGISTRY.get_bot(username) {
      bot.stop_move();
      bot.stop_jumping();
    }

    set_mutual_states(&username, "looking", false);
  }
}
