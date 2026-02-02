use azalea::entity::LocalEntity;
use azalea::entity::Position;
use azalea::prelude::*;
use azalea::Vec3;
use azalea::entity::Dead;
use azalea::entity::metadata::{Player, AbstractAnimal, AbstractMonster};
use azalea::player::GameProfileComponent;  
use azalea::registry::builtin::ItemKind;
use azalea::world::MinecraftEntityId;
use azalea::ecs::query::{With, Without};
use azalea::ecs::prelude::Entity;
use serde::{Serialize, Deserialize};
use std::time::Duration;
use tokio::time::sleep;

use crate::common::{get_entity_position, take_item, release_use_item};
use crate::tools::*;
use crate::base::*;


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
  fn find_bow_in_inventory(bot: &Client) -> Option<usize> {
    let menu = bot.menu();

    for (slot, item) in menu.slots().iter().enumerate() {
      if !item.is_empty() {
        if item.kind() == ItemKind::Bow {
          return Some(slot);
        }
      }
    }

    None
  }

  async fn shoot(bot: &Client, entity: Entity) {
    if let Some(slot) = Self::find_bow_in_inventory(bot) {
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
  }

  async fn aiming(bot: &Client, options: BowAimOptions) {
    let bot_id = if let Some(bot_id) = bot.get_entity_component::<MinecraftEntityId>(bot.entity) {
      bot_id
    } else {
      return;
    };

    let eye_pos = bot.eye_position();
    let nickname = bot.username();

    loop {
      if !STATES.get_state(&nickname, "is_eating") && !STATES.get_state(&nickname, "is_drinking") && !STATES.get_state(&nickname, "is_attackng") {
        let mut nearest_entity = None;

        match options.target.as_str() {
          "nearest-player" => {
            nearest_entity = bot.nearest_entity_by::<(&Position, &MinecraftEntityId), (With<Player>, Without<LocalEntity>, Without<Dead>)>(|data: (&Position, &MinecraftEntityId)| {
              eye_pos.distance_to(**data.0) <= options.max_distance.unwrap_or(70.0) && *data.1 != bot_id
            });
          },
          "nearest-monster" => {
            nearest_entity = bot.nearest_entity_by::<(&Position, &MinecraftEntityId), (With<AbstractMonster>, Without<LocalEntity>, Without<Dead>)>(|data: (&Position, &MinecraftEntityId)| {
              eye_pos.distance_to(**data.0) <= options.max_distance.unwrap_or(70.0) && *data.1 != bot_id
            });
          },
          "nearest-animal" => {
            nearest_entity = bot.nearest_entity_by::<(&Position, &MinecraftEntityId), (With<AbstractAnimal>, Without<LocalEntity>, Without<Dead>)>(|data: (&Position, &MinecraftEntityId)| {
              eye_pos.distance_to(**data.0) <= options.max_distance.unwrap_or(70.0) && *data.1 != bot_id
            });
          },
          "custom-goal" => {
            if let Some(nickname) = options.nickname.clone() {
              nearest_entity = bot.nearest_entity_by::<(&Position, &GameProfileComponent, &MinecraftEntityId), (With<Player>, Without<LocalEntity>, Without<Dead>)>(|data: (&Position, &GameProfileComponent, &MinecraftEntityId)| {
                eye_pos.distance_to(**data.0) <= options.max_distance.unwrap_or(70.0) && data.1.name == nickname && *data.2 != bot_id
              });
            }
          },
          _ => {}
        }

        if let Some(entity) = nearest_entity {
          let pos = get_entity_position(bot, entity);
          
          bot.look_at(Vec3::new(pos.x + randfloat(-0.3405, 0.3405), pos.y + randfloat(-0.3405, 0.3405), pos.z + randfloat(-0.3405, 0.3405)));
          
          Self::shoot(bot, entity).await;
        }
      }

      sleep(Duration::from_millis(options.delay.unwrap_or(50))).await; 
    }
  }

  pub async fn enable(bot: &Client, options: BowAimOptions) {
    Self::aiming(bot, options).await;
  }

  pub fn stop(bot: &Client) {
    TASKS.get(&bot.username()).unwrap().write().unwrap().kill_task("bow-aim");
    release_use_item(bot);
  }
}
