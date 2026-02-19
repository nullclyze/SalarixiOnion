use azalea::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::common::*;
use crate::generators::randuint;
use crate::methods::SafeClientMethods;

pub struct AutoLookPlugin;

impl AutoLookPlugin {
  pub fn new() -> Self {
    Self
  }

  pub fn enable(&'static self, username: String) {
    tokio::spawn(async move {
      loop {
        if let Some(arc) = get_flow_manager() {
          if !arc.read().active {
            break;
          }
        }

        let _ = BOT_REGISTRY
          .get_bot(&username, async |bot| {
            if !bot.workable() {
              return;
            }

            self.look(&bot).await;
          })
          .await;

        sleep(Duration::from_millis(50)).await;
      }
    });
  }

  async fn look(&self, bot: &Client) {
    if STATES.get_state(&bot.username(), "can_looking") {
      if bot.is_goto_target_reached() {
        if let Some(entity) = get_nearest_entity(bot, EntityFilter::new(bot, "any", 14.0)) {
          look_at_entity(bot, entity, true);
          sleep(Duration::from_millis(randuint(50, 100))).await;
        }
      }
    }
  }
}
