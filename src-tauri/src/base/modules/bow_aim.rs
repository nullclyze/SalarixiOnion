use azalea::entity::Dead;
use azalea::entity::metadata::AbstractAnimal;
use azalea::entity::metadata::AbstractMonster;
use azalea::prelude::*;
use azalea::Vec3;
use azalea::core::position::BlockPos;
use azalea::entity::{metadata::Player, LocalEntity};
use azalea::player::GameProfileComponent;  
use azalea::protocol::packets::game::ServerboundPlayerAction;
use azalea::protocol::packets::game::s_player_action::Action;
use azalea::packet::game::SendGamePacketEvent;
use azalea::core::direction::Direction;
use azalea::registry::builtin::ItemKind;
use bevy_ecs::query::{With, Without};
use bevy_ecs::prelude::Entity;
use serde::{Serialize, Deserialize};

use crate::TASKS;
use crate::common::convert_inventory_slot_to_hotbar_slot;
use crate::common::find_empty_slot_in_hotbar;
use crate::common::get_entity_position;
use crate::tools::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BowAimModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BowAimOptions {
  pub target: String,
  pub nickname: Option<String>,
  pub delay: Option<usize>,
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
      let final_slot;

      if let Some(hotbar_slot) = convert_inventory_slot_to_hotbar_slot(slot) {
        final_slot = hotbar_slot;
      } else {
        let inventory = bot.get_inventory();

        if let Some(empty_slot) = find_empty_slot_in_hotbar(bot) {
          inventory.left_click(slot);
          bot.wait_ticks(randticks(1, 2)).await;
          inventory.left_click(empty_slot);

          final_slot = empty_slot;
        } else {
          let random_slot = randuint(36, 44) as usize;

          inventory.shift_click(random_slot);

          bot.wait_ticks(1).await;

          inventory.left_click(slot);
          bot.wait_ticks(randticks(1, 2)).await;
          inventory.left_click(random_slot);

          final_slot = convert_inventory_slot_to_hotbar_slot(random_slot).unwrap_or(0);
        }
      }

      if bot.selected_hotbar_slot() != final_slot {
        bot.set_selected_hotbar_slot(final_slot);
      }
      
      bot.start_use_item();
      
      bot.wait_ticks(randuint(15, 20) as usize).await;

      let target_pos = get_entity_position(bot, entity);
      let distance = bot.eye_position().distance_to(target_pos);

      bot.look_at(Vec3::new(
        target_pos.x + randfloat(-0.002, 0.002),
        target_pos.y + distance * 0.15,
        target_pos.z + randfloat(-0.002, 0.002)
      ));

      if distance > 50.0 {
        bot.jump();

        bot.wait_ticks(1).await;

        let target_pos = get_entity_position(bot, entity);
        let distance = bot.eye_position().distance_to(target_pos);

        bot.look_at(Vec3::new(
          target_pos.x + randfloat(-0.002, 0.002),
          target_pos.y + distance * 0.1,
          target_pos.z + randfloat(-0.002, 0.002)
        ));
      }

      bot.wait_ticks(1).await;

      Self::release_use_item(bot);

      bot.wait_ticks(1).await;
    }
  }

  async fn aiming(bot: &Client, options: BowAimOptions) {
    let mut counter = 0;

    loop {
      let mut nearest_entity = None;

      match options.target.as_str() {
        "nearest-player" => {
          nearest_entity = bot.nearest_entity_by::<(), (With<Player>, Without<LocalEntity>, Without<Dead>)>(|_: ()| true);
        },
        "nearest-monster" => {
          nearest_entity = bot.nearest_entity_by::<(), (With<AbstractMonster>, Without<LocalEntity>, Without<Dead>)>(|_: ()| true);
        },
        "nearest-animal" => {
          nearest_entity = bot.nearest_entity_by::<(), (With<AbstractAnimal>, Without<LocalEntity>, Without<Dead>)>(|_: ()| true);
        },
        "custom-goal" => {
          if let Some(nickname) = options.nickname.clone() {
            nearest_entity = bot.nearest_entity_by::<&GameProfileComponent, (With<Player>, Without<LocalEntity>, Without<Dead>)>(|profile: &GameProfileComponent| profile.name == nickname);
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

      bot.wait_ticks(options.delay.unwrap_or(1)).await;
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
