use azalea::entity::LocalEntity;
use azalea::{SprintDirection, WalkDirection, prelude::*};
use azalea::ecs::prelude::*;
use azalea::Vec3;
use azalea::entity::{Dead, Position, metadata::{Player, AbstractAnimal, AbstractMonster}};
use azalea::registry::builtin::ItemKind;
use azalea::world::MinecraftEntityId;
use serde::{Serialize, Deserialize};
use std::time::Duration;
use tokio::time::sleep;

use crate::TASKS;
use crate::state::STATES;
use crate::tools::*;
use crate::common::{get_entity_position, move_item_to_hotbar, run};


#[derive(Debug)]
struct Weapon {
  slot: Option<usize>,
  priority: u8
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KillauraModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KillauraOptions {
  pub mode: String,
  pub settings: String,
  pub target: String,
  pub weapon: String,
  pub slot: Option<u8>,
  pub distance: Option<f64>,
  pub delay: Option<u64>,
  pub chase_distance: Option<f64>,
  pub min_distance_to_target: Option<f64>,
  pub use_dodging: bool,
  pub use_chase: bool,
  pub use_critical: bool,
  pub state: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KillauraConfig {
  target: String,
  slot: Option<u8>,
  distance: f64,
  delay: u64,
  chase_distance: f64,
  min_distance_to_target: f64
}

impl KillauraModule {
  fn create_adaptive_config(target: String) -> KillauraConfig {
    KillauraConfig {
      target: target,
      slot: None,
      distance: 3.1,
      delay: 300,
      chase_distance: 10.0,
      min_distance_to_target: 3.0
    }
  }

  fn find_nearest_entity(bot: &Client, target: String, distance: f64) -> Option<Entity> {
    let mut nearest_entity = None;

    let eye_pos = bot.eye_position();

    let bot_id = if let Some(bot_id) = bot.get_entity_component::<MinecraftEntityId>(bot.entity) {
      bot_id
    } else {
      return None;
    };

    match target.as_str() {
      "player" => { 
        nearest_entity = bot.nearest_entity_by::<(&Position, &MinecraftEntityId), (With<Player>, Without<LocalEntity>, Without<Dead>)>(|data: (&Position, &MinecraftEntityId)| {
          eye_pos.distance_to(**data.0) <= distance && *data.1 != bot_id
        });
      },
      "monster" => {
        nearest_entity = bot.nearest_entity_by::<(&Position, &MinecraftEntityId), (With<AbstractMonster>, Without<LocalEntity>, Without<Dead>)>(|data: (&Position, &MinecraftEntityId)| {
          eye_pos.distance_to(**data.0) <= distance && *data.1 != bot_id
        });
      },
      "animal" => {
        nearest_entity = bot.nearest_entity_by::<(&Position, &MinecraftEntityId), (With<AbstractAnimal>, Without<LocalEntity>, Without<Dead>)>(|data: (&Position, &MinecraftEntityId)| {
          eye_pos.distance_to(**data.0) <= distance && *data.1 != bot_id
        });
      },
      _ => {}
    }

    nearest_entity
  }

  async fn auto_weapon(bot: &Client, weapon: &String) {
    let mut weapons = vec![];

    let menu = bot.menu();

    for (slot, item) in menu.slots().iter().enumerate() {
      if !item.is_empty() {
        match weapon.as_str() {
          "sword" => {
            match item.kind() {
              ItemKind::WoodenSword => { weapons.push(Weapon { slot: Some(slot), priority: 0 }); },
              ItemKind::GoldenSword => { weapons.push(Weapon { slot: Some(slot), priority: 1 }); },
              ItemKind::StoneSword => { weapons.push(Weapon { slot: Some(slot), priority: 2 }); },
              ItemKind::CopperSword => { weapons.push(Weapon { slot: Some(slot), priority: 3 }); },
              ItemKind::IronSword => { weapons.push(Weapon { slot: Some(slot), priority: 4 }); },
              ItemKind::DiamondSword => { weapons.push(Weapon { slot: Some(slot), priority: 5 }); },
              ItemKind::NetheriteSword => { weapons.push(Weapon { slot: Some(slot), priority: 6 }); },
              _ => {}
            }
          },
          "axe" => {
            match item.kind() {
              ItemKind::WoodenAxe => { weapons.push(Weapon { slot: Some(slot), priority: 0 }); },
              ItemKind::GoldenAxe => { weapons.push(Weapon { slot: Some(slot), priority: 1 }); },
              ItemKind::StoneAxe => { weapons.push(Weapon { slot: Some(slot), priority: 2 }); },
              ItemKind::CopperAxe => { weapons.push(Weapon { slot: Some(slot), priority: 3 }); },
              ItemKind::IronAxe => { weapons.push(Weapon { slot: Some(slot), priority: 4 }); },
              ItemKind::DiamondAxe => { weapons.push(Weapon { slot: Some(slot), priority: 5 }); },
              ItemKind::NetheriteAxe => { weapons.push(Weapon { slot: Some(slot), priority: 6 }); },
              _ => {}
            }
          },
          _ => {}
        }
      }
    }

    let mut best_weapon = Weapon { slot: None, priority: 0 };

    for w in weapons {
      if w.priority > best_weapon.priority {
        best_weapon = w;
      }
    }

    if let Some(slot) = best_weapon.slot {
      move_item_to_hotbar(bot, slot).await;
    }
  }

  fn chase(bot: Client, target: String, distance: f64, min_distance_to_target: f64) {
    tokio::spawn(async move {
      loop {
        if !TASKS.get_task_activity(&bot.username(), "killaura") {
          bot.walk(WalkDirection::None);
          break;
        }

        let nearest_entity = Self::find_nearest_entity(&bot, target.clone(), distance);

        if let Some(entity) = nearest_entity {
          let eye_pos = bot.eye_position();

          if eye_pos.distance_to(get_entity_position(&bot, entity)) > min_distance_to_target {
            run(&bot, SprintDirection::Forward);

            let entity_pos = get_entity_position(&bot, entity);

            bot.look_at(Vec3::new(
              entity_pos.x,
              entity_pos.y + randfloat(1.0, 1.5),
              entity_pos.z
            ));
          } else {
            bot.walk(WalkDirection::None);
          }
        } else {
          bot.walk(WalkDirection::None);
        }

        sleep(Duration::from_millis(50)).await;
      }
    });
  }

  async fn moderate_killaura(bot: &Client, options: KillauraOptions) {
    let config = if options.settings.as_str() == "adaptive" {
      Self::create_adaptive_config(options.target.clone())
    } else {
      KillauraConfig { 
        target: options.target.clone(), 
        slot: options.slot, 
        distance: options.distance.unwrap_or(3.1), 
        delay: options.delay.unwrap_or(300),
        chase_distance: options.chase_distance.unwrap_or(10.0),
        min_distance_to_target: options.min_distance_to_target.unwrap_or(3.0)
      }
    };

    if options.use_chase {
      Self::chase(bot.clone(), config.target.clone(), config.chase_distance, config.min_distance_to_target);
    }

    let nickname = bot.username();

    loop {
      if !STATES.get_plugin_activity(&bot.username(), "auto-eat") && !STATES.get_plugin_activity(&bot.username(), "auto-potion") {
        if let Some(entity) = Self::find_nearest_entity(bot, config.target.clone(), config.distance) {
          STATES.set(&nickname, "attacks", "true".to_string());

          if options.settings.as_str() == "adaptive" {
            Self::auto_weapon(bot, &options.weapon).await;
          } else {
            if let Some(slot) = config.slot {
              if slot <= 8 {
                bot.set_selected_hotbar_slot(slot);
              }
            } else {
              Self::auto_weapon(bot, &options.weapon).await;
            }
          }

          if options.use_critical {
            bot.jump();
            sleep(Duration::from_millis(randuint(50, 100))).await;
          }

          let entity_pos = get_entity_position(&bot, entity);

          bot.look_at(Vec3::new(
            entity_pos.x,
            entity_pos.y + randfloat(1.0, 1.5),
            entity_pos.z
          ));

          sleep(Duration::from_millis(randuint(100, 200))).await;

          if let Some(e) = Self::find_nearest_entity(bot, config.target.clone(), config.distance) {
            let entity_pos = get_entity_position(&bot, entity);

            bot.look_at(Vec3::new(
              entity_pos.x,
              entity_pos.y + randfloat(1.0, 1.5),
              entity_pos.z
            ));

            bot.attack(e);
          }

          STATES.set(&nickname, "attacks", "false".to_string());
        }
      }
      
      sleep(Duration::from_millis(config.delay)).await;
    }
  }

  async fn aggressive_killaura(bot: &Client, options: KillauraOptions) {
    let config = if options.settings.as_str() == "adaptive" {
      Self::create_adaptive_config(options.target.clone())
    } else {
      KillauraConfig { 
        target: options.target.clone(), 
        slot: options.slot, 
        distance: options.distance.unwrap_or(3.1), 
        delay: options.delay.unwrap_or(150),
        chase_distance: options.chase_distance.unwrap_or(10.0),
        min_distance_to_target: options.min_distance_to_target.unwrap_or(3.0)
      }
    };

    if options.use_chase {
      Self::chase(bot.clone(), config.target.clone(), config.chase_distance, config.min_distance_to_target);
    }

    let nickname = bot.username();

    loop {
      if !STATES.get_plugin_activity(&bot.username(), "auto-eat") && !STATES.get_plugin_activity(&bot.username(), "auto-potion") {
        if let Some(_) = Self::find_nearest_entity(bot, config.target.clone(), config.distance) {
          STATES.set(&nickname, "attacks", "true".to_string());
          
          if options.settings.as_str() == "adaptive" {
            Self::auto_weapon(bot, &options.weapon).await;
          } else {
            if let Some(slot) = config.slot {
              if slot <= 8 {
                bot.set_selected_hotbar_slot(slot);
              }
            } else {
              Self::auto_weapon(bot, &options.weapon).await;
            }
          }

          if options.use_critical {
            bot.jump();
            sleep(Duration::from_millis(randuint(50, 150))).await;
          }
          
          if let Some(e) = Self::find_nearest_entity(bot, config.target.clone(), config.distance) {
            bot.attack(e);
          }
        }

        STATES.set(&nickname, "attacks", "false".to_string());
      }

      sleep(Duration::from_millis(config.delay)).await;
    }
  }

  pub async fn enable(bot: &Client, options: KillauraOptions) {
    match options.mode.as_str() {
      "moderate" => { Self::moderate_killaura(bot, options).await; },
      "aggressive" => { Self::aggressive_killaura(bot, options).await; },
      _ => {}
    }
  }

  pub fn stop(bot: &Client) {
    TASKS.get(&bot.username()).unwrap().write().unwrap().stop_task("killaura");
    bot.walk(WalkDirection::None);
  }
}