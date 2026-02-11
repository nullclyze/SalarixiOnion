use azalea::entity::Position;
use azalea::prelude::*;
use azalea::Vec3;
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::common::get_nearest_entity;
use crate::common::EntityFilter;
use crate::tools::*;

pub struct AutoLookPlugin;

impl AutoLookPlugin {
  pub fn new() -> Self {
    Self
  }

  pub fn enable(&'static self, bot: Client) {
    tokio::spawn(async move {
      loop {
        if let Some(arc) = get_flow_manager() {
          if !arc.read().active {
            break;
          }
        }

        self.look(&bot).await;

        sleep(Duration::from_millis(50)).await;
      }
    });
  }

  async fn look(&self, bot: &Client) {
    if STATES.get_state(&bot.username(), "can_looking") {
      if bot.is_goto_target_reached() {
        if let Some(entity) = get_nearest_entity(bot, EntityFilter::new(bot, "any", 14.0)) {
          if let Some(entity_pos) = bot.get_entity_component::<Position>(entity) {
            let pos = Vec3::new(
              entity_pos.x + randfloat(-0.1, 0.1),
              entity_pos.y + randfloat(-0.1, 0.1),
              entity_pos.z + randfloat(-0.1, 0.1),
            );

            bot.look_at(pos);

            sleep(Duration::from_millis(randuint(50, 100))).await;
          }
        }
      }
    }
  }
}
