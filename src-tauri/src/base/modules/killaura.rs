use azalea::prelude::*;
use azalea::ecs::prelude::*;
use azalea::Vec3;
use azalea::entity::{Dead, LocalEntity, Position, metadata::{Player, AbstractAnimal, AbstractMonster}};
use azalea::registry::builtin::ItemKind;
use azalea::world::MinecraftEntityId;
use serde::{Serialize, Deserialize};

use crate::TASKS;
use crate::tools::*;
use crate::common::convert_inventory_slot_to_hotbar_slot;


#[derive(Debug)]
struct Weapon {
  slot: Option<u8>,
  priority: u8
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KillauraModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KillauraOptions {
  pub mode: String,
  pub target: String,
  pub distance: Option<f64>,
  pub delay: Option<usize>,
  pub state: bool
}

impl KillauraModule {
  fn find_nearest_entity(bot: &Client, target: String, distance: Option<f64>) -> Option<Entity> {
    let mut nearest_entity = None;

    let bot_position = bot.eye_position();

    match target.as_str() {
      "player" => { 
        nearest_entity = bot.nearest_entity_by::<&Position, (With<Player>, Without<LocalEntity>, Without<Dead>)>(|position: &Position| {
          bot_position.distance_to(**position) <= distance.unwrap_or(3.1)
        });
      },
      "monster" => {
        nearest_entity = bot.nearest_entity_by::<&Position, (With<AbstractMonster>, Without<LocalEntity>, Without<Dead>)>(|position: &Position| {
          bot_position.distance_to(**position) <= distance.unwrap_or(3.1)
        });
      },
      "animal" => {
        nearest_entity = bot.nearest_entity_by::<&Position, (With<AbstractAnimal>, Without<LocalEntity>, Without<Dead>)>(|position: &Position| {
          bot_position.distance_to(**position) <= distance.unwrap_or(3.1)
        });
      },
      _ => {}
    }

    nearest_entity
  }

  fn get_entity_position(bot: &Client, entity: Entity) -> Vec3 {
    let mut ecs = bot.ecs.lock(); 
    let pos = ecs.get_mut::<Position>(entity).unwrap().clone();

    Vec3::new(pos.x, pos.y, pos.z)
  }

  fn auto_weapon(bot: &Client) {
    let mut weapons = vec![];

    if let Some(inventory) = bot.open_inventory() {
      if let Some(menu) = inventory.menu() {
        for slot in menu.hotbar_slots_range() {
          if let Some(item) = menu.slot(slot) {
            if !item.is_empty() {
              match item.kind() {
                ItemKind::WoodenSword => { weapons.push(Weapon { slot: Some(slot as u8), priority: 0 }); },
                ItemKind::GoldenSword => { weapons.push(Weapon { slot: Some(slot as u8), priority: 1 }); },
                ItemKind::StoneSword => { weapons.push(Weapon { slot: Some(slot as u8), priority: 2 }); },
                ItemKind::CopperSword => { weapons.push(Weapon { slot: Some(slot as u8), priority: 3 }); },
                ItemKind::IronSword => { weapons.push(Weapon { slot: Some(slot as u8), priority: 4 }); },
                ItemKind::DiamondSword => { weapons.push(Weapon { slot: Some(slot as u8), priority: 5 }); },
                ItemKind::NetheriteSword => { weapons.push(Weapon { slot: Some(slot as u8), priority: 6 }); },
                _ => {}
              }
            }
          }
        }
      }
    }

    let mut best_weapon = Weapon { slot: None, priority: 0 };

    for weapon in weapons {
      if weapon.priority > best_weapon.priority {
        best_weapon = weapon;
      }
    }

    if let Some(slot) = best_weapon.slot {
      if bot.selected_hotbar_slot() != slot {
        if let Some(s) = convert_inventory_slot_to_hotbar_slot(slot as u16) {
          bot.set_selected_hotbar_slot(s);
        }
      }
    }
  }

  async fn moderate_killaura(bot: &Client, options: KillauraOptions) {
    loop {
      Self::auto_weapon(bot);

      let nearest_entity = Self::find_nearest_entity(bot, options.target.clone(), options.distance);

      if let Some(entity) = nearest_entity {
        if let Some(bot_id) = bot.get_entity_component::<MinecraftEntityId>(bot.entity) {
          if let Some(entity_id) = bot.get_entity_component::<MinecraftEntityId>(entity) {
            if bot_id != entity_id {
              bot.look_at(Self::get_entity_position(bot, entity));
              bot.wait_ticks(randuint(3, 5) as usize).await;
              bot.attack(entity);
            }
          }
        }
      }

      bot.wait_ticks(options.delay.unwrap_or(1)).await;
    }
  }

  async fn aggressive_killaura(bot: &Client, options: KillauraOptions) {
    loop {
      Self::auto_weapon(bot);

      let nearest_entity = Self::find_nearest_entity(bot, options.target.clone(), options.distance);

      if let Some(entity) = nearest_entity {
        if let Some(bot_id) = bot.get_entity_component::<MinecraftEntityId>(bot.entity) {
          if let Some(entity_id) = bot.get_entity_component::<MinecraftEntityId>(entity) {
            if bot_id != entity_id {
              bot.wait_ticks(randuint(3, 4) as usize).await;
              bot.attack(entity);
            }
          }
        }
      }

      bot.wait_ticks(options.delay.unwrap_or(1)).await;
    }
  }

  pub async fn enable(bot: &Client, options: KillauraOptions) {
    match options.mode.as_str() {
      "moderate" => { Self::moderate_killaura(bot, options).await; },
      "aggressive" => { Self::aggressive_killaura(bot, options).await; },
      _ => {}
    }
  }

  pub fn stop(nickname: &String) {
    TASKS.get(nickname).unwrap().write().unwrap().stop_task("killaura");
  }
}