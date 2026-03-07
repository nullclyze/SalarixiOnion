use azalea::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

use crate::core::*;
use crate::generators::randuint;
use crate::extensions::{BotDefaultExt, BotRotationExt, EntityType};

pub struct AutoLookPlugin;

impl AutoLookPlugin {
  pub fn new() -> Self {
    Self
  }

  pub fn enable(&'static self, username: String) {
    let nickname = username.clone();

    let task = tokio::spawn(async move {
      loop {
        if !process_is_active() {
          break;
        }

        let _ = BOT_REGISTRY
          .async_get_bot(&nickname, async |bot| {
            if !bot.workable() {
              return;
            }

            self.look(&bot).await;
          })
          .await;

        sleep(Duration::from_millis(50)).await;
      }
    });

    PLUGIN_MANAGER.push_task(&username, "auto-look", task);
  }

  async fn look(&self, bot: &Client) {
    let username = bot.name();

    if STATES.get_state(&username, "can_looking") && bot.is_goto_target_reached() {
      if let Some(entity) = bot.find_nearest_entity(EntityType::Any, 14.0) {
        bot.look_at_entity(entity, true);
        sleep(Duration::from_millis(randuint(50, 100))).await;
      }
    }
  }
}
