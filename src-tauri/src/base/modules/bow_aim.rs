use azalea::prelude::*;
use azalea::Vec3;
use azalea::registry::builtin::ItemKind;
use azalea::ecs::prelude::Entity;
use serde::{Serialize, Deserialize};
use std::time::Duration;
use tokio::time::sleep;

use crate::common::get_inventory_menu;
use crate::tools::*;
use crate::base::*;
use crate::common::{EntityFilter, get_nearest_entity, get_entity_position, take_item, release_use_item};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BowAimModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BowAimOptions {
  pub target: String,
  pub nickname: Option<String>,
  pub delay: Option<u64>,
  pub max_distance: Option<f64>,
  pub state: bool
}

impl BowAimModule {
  pub fn new() -> Self {
    Self
  }

  fn find_bow_in_inventory(&self, bot: &Client) -> Option<usize> {
    if let Some(menu) = get_inventory_menu(bot) {
      for (slot, item) in menu.slots().iter().enumerate() {
        if !item.is_empty() {
          if item.kind() == ItemKind::Bow {
            return Some(slot);
          }
        }
      }
    }

    None
  }

  async fn shoot(&self, bot: &Client, entity: Entity) {
    let nickname = bot.username();

    if !STATES.get_state(&nickname, "is_eating") && STATES.get_state(&nickname, "is_drinking") && !STATES.get_state(&nickname, "is_attacking") && STATES.get_state(&nickname, "can_interacting") {
      STATES.set_state(&nickname, "can_eating", false);
      STATES.set_state(&nickname, "can_drinking", false);
      STATES.set_state(&nickname, "can_attacking", false);
      STATES.set_mutual_states(&nickname, "interacting", true);
      
      if let Some(slot) = self.find_bow_in_inventory(bot) {
        take_item(bot, slot).await;

        bot.start_use_item();

        sleep(Duration::from_millis(randuint(800, 1100))).await;

        let target_pos = get_entity_position(bot, entity);
        let distance = bot.eye_position().distance_to(target_pos);

        bot.look_at(Vec3::new(
          target_pos.x + randfloat(-0.001158, 0.001158),
          target_pos.y + distance * 0.15,
          target_pos.z + randfloat(-0.001158, 0.001158)
        ));

        if distance > 50.0 {
          bot.jump();

          sleep(Duration::from_millis(50)).await;

          let target_pos = get_entity_position(bot, entity);
          let distance = bot.eye_position().distance_to(target_pos);

          bot.look_at(Vec3::new(
            target_pos.x + randfloat(-0.001158, 0.001158),
            target_pos.y + distance * 0.1,
            target_pos.z + randfloat(-0.001158, 0.001158)
          ));
        }

        sleep(Duration::from_millis(50)).await;

        release_use_item(bot);
      }

      STATES.set_state(&nickname, "can_eating", true);
      STATES.set_state(&nickname, "can_drinking", true);
      STATES.set_state(&nickname, "can_attacking", true);
      STATES.set_mutual_states(&nickname, "interacting", false);
    }
  }

  async fn aiming(&self, bot: &Client, options: BowAimOptions) {
    let nickname = bot.username();

    if STATES.get_state(&nickname, "can_looking") {
      STATES.set_mutual_states(&nickname, "looking", true);

      loop {
        let mut nearest_entity = None;

        if options.target.as_str() == "custom" {
          if let Some(target_nickname) = &options.nickname {
            nearest_entity = get_nearest_entity(bot, EntityFilter::new(bot, target_nickname, options.max_distance.unwrap_or(70.0)));
          }
        } else {
          nearest_entity = get_nearest_entity(bot, EntityFilter::new(bot, &options.target, options.max_distance.unwrap_or(70.0)));
        }

        if let Some(entity) = nearest_entity {
          let pos = get_entity_position(bot, entity);
          
          bot.look_at(Vec3::new(pos.x + randfloat(-0.3405, 0.3405), pos.y + randfloat(-0.3405, 0.3405), pos.z + randfloat(-0.3405, 0.3405)));
          
          self.shoot(bot, entity).await;
        }
      }
    }
  }

  pub async fn enable(&self, bot: &Client, options: BowAimOptions) {
    self.aiming(bot, options).await;
  }

  pub fn stop(&self, bot: &Client) {
    let nickname = bot.username();

    kill_task(&nickname, "bow-aim");

    release_use_item(bot);

    STATES.set_mutual_states(&nickname, "looking", false);
    STATES.set_mutual_states(&nickname, "interacting", false);
  }
}
