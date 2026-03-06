use azalea::prelude::*;
use azalea::protocol::packets::game::s_interact::InteractionHand;
use azalea::registry::builtin::ItemKind;
use azalea::Vec3;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

use crate::common::{get_nearest_entity, EntityFilter};
use crate::core::*;
use crate::extensions::BotInteractExt;
use crate::extensions::{BotDefaultExt, BotInventoryExt};
use crate::generators::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BowAimModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BowAimOptions {
  pub target: String,
  pub target_nickname: Option<String>,
  pub delay: Option<u64>,
  pub max_distance: Option<f64>,
  pub state: bool,
}

impl BowAimModule {
  pub fn new() -> Self {
    Self
  }

  fn find_bow_in_inventory(&self, bot: &Client) -> Option<usize> {
    if let Some(menu) = bot.get_inventory_menu() {
      for (slot, item) in menu.slots().iter().enumerate() {
        if item.kind() == ItemKind::Bow {
          return Some(slot);
        }
      }
    }

    None
  }

  fn aiming(&self, username: String, filter: EntityFilter) {
    tokio::spawn(async move {
      BOT_REGISTRY
        .async_get_bot(&username, async |bot| {
          let nickname = bot.username();

          if STATES.get_state(&nickname, "can_looking") {
            STATES.set_mutual_states(&nickname, "looking", true);

            loop {
              if !TASKS.get_task_activity(&nickname, "bow-aim") {
                break;
              }

              if let Some(entity) = get_nearest_entity(bot, filter.clone()) {
                let target_pos = bot.get_entity_position(entity);

                bot.look_at(Vec3::new(
                  target_pos.x,
                  target_pos.y + bot.get_entity_eye_height(entity),
                  target_pos.z,
                ));
              }

              sleep(Duration::from_millis(50)).await;
            }
          }
        })
        .await;
    });
  }

  async fn shoot(&self, bot: &Client, filter: EntityFilter) {
    let nickname = bot.username();

    if STATES.get_state(&nickname, "can_interacting")
      && !STATES.get_state(&nickname, "is_eating")
      && !STATES.get_state(&nickname, "is_drinking")
      && !STATES.get_state(&nickname, "is_attacking")
    {
      if let Some(slot) = self.find_bow_in_inventory(bot) {
        STATES.set_mutual_states(&nickname, "interacting", true);

        bot.take_item(slot, true).await;

        sleep(Duration::from_millis(50)).await;

        bot.start_use_held_item(InteractionHand::MainHand);

        sleep(Duration::from_millis(randuint(900, 1100))).await;

        if let Some(entity) = get_nearest_entity(bot, filter) {
          let target_pos = bot.get_entity_position(entity);

          let distance = bot.eye_pos().distance_to(target_pos);

          bot.look_at(Vec3::new(
            target_pos.x,
            target_pos.y + distance * 0.16,
            target_pos.z,
          ));

          if distance > 50.0 {
            bot.jump();

            sleep(Duration::from_millis(100)).await;

            let target_pos = bot.get_entity_position(entity);

            let distance = bot.eye_pos().distance_to(target_pos);

            bot.look_at(Vec3::new(
              target_pos.x,
              target_pos.y + distance * 0.12,
              target_pos.z,
            ));
          }

          sleep(Duration::from_millis(50)).await;
        }

        bot.release_use_held_item();

        STATES.set_mutual_states(&nickname, "interacting", false);
      }
    }
  }

  async fn shooting(&self, bot: &Client, options: &BowAimOptions) {
    let mut entity_filter = None;

    if options.target.as_str() == "custom" {
      if let Some(target_nickname) = &options.target_nickname {
        entity_filter = Some(EntityFilter::new(
          bot,
          target_nickname,
          options.max_distance.unwrap_or(70.0),
        ));
      }
    } else {
      entity_filter = Some(EntityFilter::new(
        bot,
        &options.target,
        options.max_distance.unwrap_or(70.0),
      ));
    }

    if let Some(filter) = entity_filter {
      self.aiming(bot.username(), filter.clone());

      loop {
        self.shoot(bot, filter.clone()).await;
        sleep(Duration::from_millis(options.delay.unwrap_or(50))).await;
      }
    }
  }

  pub async fn enable(&self, username: &str, options: &BowAimOptions) {
    BOT_REGISTRY
      .async_get_bot(username, async |bot| {
        self.shooting(bot, options).await;
      })
      .await;
  }

  pub async fn stop(&self, username: &str) {
    kill_task(username, "bow-aim");

    if let Some(bot) = BOT_REGISTRY.get_bot(username) {
      bot.release_use_held_item();
    }

    STATES.set_mutual_states(username, "looking", false);
    STATES.set_mutual_states(username, "interacting", false);
  }
}
