use azalea::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

use crate::common::*;
use crate::core::*;
use crate::extensions::{BotDefaultExt, BotRotationExt};
use crate::generators::randuint;

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
    if STATES.get_state(&bot.name(), "can_looking") && bot.is_goto_target_reached() {
      if let Some(entity) = get_nearest_entity(bot, EntityFilter::new(bot, "any", 14.0)) {
        bot.look_at_entity(entity, true);
        sleep(Duration::from_millis(randuint(50, 100))).await;
      }
    }
  }
}
