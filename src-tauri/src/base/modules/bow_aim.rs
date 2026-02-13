use azalea::prelude::*;
use azalea::protocol::packets::game::s_interact::InteractionHand;
use azalea::registry::builtin::ItemKind;
use azalea::Vec3;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::common::{
  get_entity_position, get_eye_position, get_inventory_menu, get_nearest_entity, release_use_item,
  start_use_item, take_item, EntityFilter,
};
use crate::tools::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BowAimModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BowAimOptions {
  pub target: String,
  pub nickname: Option<String>,
  pub delay: Option<u64>,
  pub max_distance: Option<f64>,
  pub state: bool,
}

impl BowAimModule {
  pub fn new() -> Self {
    Self
  }

  fn find_bow_in_inventory(&self, bot: &Client) -> Option<usize> {
    if let Some(menu) = get_inventory_menu(bot) {
      for (slot, item) in menu.slots().iter().enumerate() {
        if item.kind() == ItemKind::Bow {
          return Some(slot);
        }
      }
    }

    None
  }

  fn aiming(&self, bot: Client, filter: EntityFilter) {
    tokio::spawn(async move {
      let nickname = bot.username();

      if STATES.get_state(&nickname, "can_looking") {
        STATES.set_mutual_states(&nickname, "looking", true);

        loop {
          if !TASKS.get_task_activity(&nickname, "bow-aim") {
            break;
          }

          if let Some(entity) = get_nearest_entity(&bot, filter.clone()) {
            let target_pos = get_entity_position(&bot, entity);

            bot.look_at(Vec3::new(
              target_pos.x + randfloat(-0.11, 0.11),
              target_pos.y,
              target_pos.z + randfloat(-0.11, 0.11),
            ));
          }

          sleep(Duration::from_millis(50)).await;
        }
      }
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

        take_item(bot, slot, true).await;

        sleep(Duration::from_millis(50)).await;

        start_use_item(bot, InteractionHand::MainHand);

        sleep(Duration::from_millis(randuint(900, 1100))).await;

        if let Some(entity) = get_nearest_entity(bot, filter) {
          let target_pos = get_entity_position(bot, entity);

          let feet_pos = bot.position();

          let eye_pos = Vec3 {
            x: feet_pos.x,
            y: feet_pos.y + get_eye_position(bot, bot.entity),
            z: feet_pos.z,
          };

          let distance = eye_pos.distance_to(target_pos);

          bot.look_at(Vec3::new(
            target_pos.x + randfloat(-0.001158, 0.001158),
            target_pos.y + distance * 0.15,
            target_pos.z + randfloat(-0.001158, 0.001158),
          ));

          if distance > 50.0 {
            bot.jump();

            sleep(Duration::from_millis(100)).await;

            let target_pos = get_entity_position(bot, entity);

            let feet_pos = bot.position();

            let eye_pos = Vec3 {
              x: feet_pos.x,
              y: feet_pos.y + get_eye_position(bot, bot.entity),
              z: feet_pos.z,
            };

            let distance = eye_pos.distance_to(target_pos);

            bot.look_at(Vec3::new(
              target_pos.x + randfloat(-0.001158, 0.001158),
              target_pos.y + distance * 0.12,
              target_pos.z + randfloat(-0.001158, 0.001158),
            ));
          }

          sleep(Duration::from_millis(50)).await;
        }

        release_use_item(bot);

        STATES.set_mutual_states(&nickname, "interacting", false);
      }
    }
  }

  async fn shooting(&self, bot: &Client, options: &BowAimOptions) {
    let mut entity_filter = None;

    if options.target.as_str() == "custom" {
      if let Some(target_nickname) = &options.nickname {
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
      self.aiming(bot.clone(), filter.clone());

      loop {
        self.shoot(bot, filter.clone()).await;
        sleep(Duration::from_millis(options.delay.unwrap_or(50))).await;
      }
    }
  }

  pub async fn enable(&self, bot: &Client, options: &BowAimOptions) {
    self.shooting(bot, options).await;
  }

  pub fn stop(&self, bot: &Client) {
    let nickname = bot.username();

    kill_task(&nickname, "bow-aim");

    release_use_item(bot);

    STATES.set_mutual_states(&nickname, "looking", false);
    STATES.set_mutual_states(&nickname, "interacting", false);
  }
}
