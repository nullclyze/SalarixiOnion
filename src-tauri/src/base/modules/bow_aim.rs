use std::time::Duration;

use azalea::entity::LocalEntity;
use azalea::prelude::*;
use azalea::Vec3;
use azalea::entity::Dead;
use azalea::entity::metadata::{Player, AbstractAnimal, AbstractMonster};
use azalea::core::position::BlockPos;
use azalea::player::GameProfileComponent;  
use azalea::protocol::packets::game::ServerboundPlayerAction;
use azalea::protocol::packets::game::s_player_action::Action;
use azalea::packet::game::SendGamePacketEvent;
use azalea::core::direction::Direction;
use azalea::registry::builtin::ItemKind;
use azalea::world::MinecraftEntityId;
use azalea::ecs::query::{With, Without};
use azalea::ecs::prelude::Entity;
use serde::{Serialize, Deserialize};
use tokio::time::sleep;

use crate::TASKS;
use crate::common::get_entity_position;
use crate::common::move_item_to_hotbar;
use crate::state::STATES;
use crate::tools::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BowAimModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BowAimOptions {
  pub target: String,
  pub nickname: Option<String>,
  pub delay: Option<u64>,
  pub state: bool
}

impl BowAimModule {
  fn find_bow_in_inventory(bot: &Client) -> Option<usize> {
    let menu = bot.menu();

    for slot in menu.player_slots_range() {
      if let Some(item) = menu.slot(slot) {
        if !item.is_empty() {
          if item.kind() == ItemKind::Bow {
            return Some(slot);
          }
        }
      }
    }

    None
  }

  fn release_use_item(bot: &Client) {
    bot.ecs.lock().trigger(SendGamePacketEvent::new(  
      bot.entity,  
      ServerboundPlayerAction {  
        action: Action::ReleaseUseItem,  
        pos: BlockPos::new(0, 0, 0),  
        direction: Direction::Down,  
        seq: 0
      }
    ));
  }

  async fn shoot(bot: &Client, entity: Entity) {
    if let Some(slot) = Self::find_bow_in_inventory(bot) {
      move_item_to_hotbar(bot, slot).await;

      bot.start_use_item();

      sleep(Duration::from_millis(randuint(3000, 3500))).await;

      let target_pos = get_entity_position(bot, entity);
      let distance = bot.eye_position().distance_to(target_pos);

      bot.look_at(Vec3::new(
        target_pos.x + randfloat(-0.002, 0.002),
        target_pos.y + distance * 0.15,
        target_pos.z + randfloat(-0.002, 0.002)
      ));

      if distance > 50.0 {
        bot.jump();

        sleep(Duration::from_millis(50)).await;

        let target_pos = get_entity_position(bot, entity);
        let distance = bot.eye_position().distance_to(target_pos);

        bot.look_at(Vec3::new(
          target_pos.x + randfloat(-0.002, 0.002),
          target_pos.y + distance * 0.1,
          target_pos.z + randfloat(-0.002, 0.002)
        ));
      }

      sleep(Duration::from_millis(50)).await;

      Self::release_use_item(bot);
    }
  }

  async fn aiming(bot: &Client, options: BowAimOptions) {
    let mut counter = 0;

    let bot_id = if let Some(bot_id) = bot.get_entity_component::<MinecraftEntityId>(bot.entity) {
      bot_id
    } else {
      return;
    };

    let nickname = bot.username();

    loop {
      if !STATES.get_plugin_activity(&nickname, "auto-eat") && !STATES.get_plugin_activity(&nickname, "auto-potion") {
        let mut nearest_entity = None;

        match options.target.as_str() {
          "nearest-player" => {
            nearest_entity = bot.nearest_entity_by::<&MinecraftEntityId, (With<Player>, Without<LocalEntity>, Without<Dead>)>(|id: &MinecraftEntityId| {
              *id != bot_id
            });
          },
          "nearest-monster" => {
            nearest_entity = bot.nearest_entity_by::<&MinecraftEntityId, (With<AbstractMonster>, Without<LocalEntity>, Without<Dead>)>(|id: &MinecraftEntityId| {
              *id != bot_id
            });
          },
          "nearest-animal" => {
            nearest_entity = bot.nearest_entity_by::<&MinecraftEntityId, (With<AbstractAnimal>, Without<LocalEntity>, Without<Dead>)>(|id: &MinecraftEntityId| {
              *id != bot_id
            });
          },
          "custom-goal" => {
            if let Some(nickname) = options.nickname.clone() {
              nearest_entity = bot.nearest_entity_by::<(&GameProfileComponent, &MinecraftEntityId), (With<Player>, Without<LocalEntity>, Without<Dead>)>(|data: (&GameProfileComponent, &MinecraftEntityId)| {
                data.0.name == nickname && *data.1 != bot_id
              });
            }
          },
          _ => {}
        }

        if let Some(entity) = nearest_entity {
          counter += 1;

          let pos = get_entity_position(bot, entity);
          
          bot.look_at(Vec3::new(pos.x + randfloat(-0.5, 0.5), pos.y + randfloat(-0.5, 0.5), pos.z + randfloat(-0.5, 0.5)));

          if counter >= 2 {
            Self::shoot(bot, entity).await;
            counter = 0;
          }
        }

        sleep(Duration::from_millis(options.delay.unwrap_or(50))).await;
      } 
    }
  }

  pub async fn enable(bot: &Client, options: BowAimOptions) {
    Self::aiming(bot, options).await;
  }

  pub fn stop(bot: &Client) {
    TASKS.get(&bot.username()).unwrap().write().unwrap().stop_task("bow-aim");
    Self::release_use_item(bot);
  }
}
