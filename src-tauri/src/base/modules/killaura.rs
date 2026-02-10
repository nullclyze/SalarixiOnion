use azalea::{SprintDirection, WalkDirection, prelude::*};
use azalea::Vec3;
use azalea::registry::builtin::ItemKind;
use serde::{Serialize, Deserialize};
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::tools::*;
use crate::common::{EntityFilter, get_entity_position, get_eye_position, get_inventory_menu, get_nearest_entity, run, stop_bot_move, take_item};


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
  pub weapon_slot: Option<u8>,
  pub distance: Option<f64>,
  pub delay: Option<u64>,
  pub chase_distance: Option<f64>,
  pub min_distance_to_target: Option<f64>,
  pub use_auto_weapon: bool,
  pub use_dodging: bool,
  pub use_chase: bool,
  pub use_critical: bool,
  pub state: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct KillauraConfig {
  target: String,
  weapon_slot: Option<u8>,
  distance: f64,
  delay: u64,
  chase_distance: f64,
  min_distance_to_target: f64
}

impl KillauraModule {
  pub fn new() -> Self {
    Self
  }

  fn create_adaptive_config(&self, target: String, chase_distance: Option<f64>, min_distance_to_target: Option<f64>) -> KillauraConfig {
    KillauraConfig {
      target: target,
      weapon_slot: None,
      distance: 3.1,
      delay: 500,
      chase_distance: chase_distance.unwrap_or(10.0),
      min_distance_to_target: min_distance_to_target.unwrap_or(3.0)
    }
  }

  async fn auto_weapon(&self, bot: &Client, weapon: &String) {
    let mut weapons = vec![];

    if let Some(menu) = get_inventory_menu(bot) {
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
    }

    let mut best_weapon = Weapon { slot: None, priority: 0 };

    for w in weapons {
      if w.priority > best_weapon.priority {
        best_weapon = w;
      }
    }

    if let Some(slot) = best_weapon.slot {
      take_item(bot, slot).await;
    }
  }

  fn dodging(&self, bot: Client) {
    tokio::spawn(async move {
      loop {
        if !TASKS.get_task_activity(&bot.username(), "killaura") {
          bot.walk(WalkDirection::None);
          break;
        }

        if !bot.crouching() {
          bot.set_crouching(true);
          sleep(Duration::from_millis(randuint(150, 250))).await;
          bot.set_crouching(false);
        }

        sleep(Duration::from_millis(50)).await;
      }
    });
  }

  fn chase(&'static self, bot: Client, target: String, distance: f64, min_distance_to_target: f64) {
    tokio::spawn(async move {
      let nickname = bot.username();

      loop {
        if !TASKS.get_task_activity(&nickname, "killaura") {
          stop_bot_move(&bot);
          break;
        }

        if let Some(entity) = get_nearest_entity(&bot, EntityFilter::new(&bot, &target, distance)) {
          if get_eye_position(&bot).distance_to(get_entity_position(&bot, entity)) > min_distance_to_target {
            if !bot.jumping() {
              bot.set_jumping(true);
            }

            run(&bot, SprintDirection::Forward);

            if !STATES.get_state(&nickname, "is_looking") {
              let entity_pos = get_entity_position(&bot, entity);

              bot.look_at(Vec3::new(
                entity_pos.x,
                entity_pos.y + randfloat(0.4, 0.6),
                entity_pos.z
              ));
            }
          } else {
            bot.set_jumping(false);

            if !STATES.get_state(&nickname, "is_walking") {
              stop_bot_move(&bot);
            }
          }
        } else {
          bot.set_jumping(false);
          
          if !STATES.get_state(&nickname, "is_walking") {
            stop_bot_move(&bot);
          }
        }

        sleep(Duration::from_millis(50)).await;
      }
    });
  }

  fn aiming(&'static self, bot: Client, target: String, distance: f64) {
    tokio::spawn(async move {
      let nickname = bot.username();

      if STATES.get_state(&nickname, "can_looking") {
        STATES.set_mutual_states(&nickname, "looking", true);

        loop {
          if !TASKS.get_task_activity(&nickname, "killaura") {
            STATES.set_mutual_states(&nickname, "looking", false);
            break;
          }

          if let Some(entity) = get_nearest_entity(&bot, EntityFilter::new(&bot, &target, distance)) {
            let entity_pos = get_entity_position(&bot, entity);

            bot.look_at(Vec3::new(
              entity_pos.x,
              entity_pos.y + randfloat(0.4, 0.6),
              entity_pos.z
            ));
          }

          sleep(Duration::from_millis(50)).await;
        }
      }
    });
  }

  async fn moderate_killaura(&'static self, bot: &Client, options: &KillauraOptions) {
    let config = if options.settings.as_str() == "adaptive" {
      self.create_adaptive_config(options.target.clone(), options.chase_distance, options.min_distance_to_target)
    } else {
      KillauraConfig { 
        target: options.target.clone(), 
        weapon_slot: options.weapon_slot, 
        distance: options.distance.unwrap_or(3.1), 
        delay: options.delay.unwrap_or(300),
        chase_distance: options.chase_distance.unwrap_or(10.0),
        min_distance_to_target: options.min_distance_to_target.unwrap_or(3.0)
      }
    };

    self.aiming(bot.clone(), config.target.clone(), config.chase_distance);

    if options.use_chase {
      self.chase(bot.clone(), config.target.clone(), config.chase_distance, config.min_distance_to_target);
    }

    if options.use_dodging {
      self.dodging(bot.clone());
    }

    let nickname = bot.username();

    loop {
      if STATES.get_state(&nickname, "can_attacking") && !STATES.get_state(&nickname, "is_interacting") && !STATES.get_state(&nickname, "is_eating") && !STATES.get_state(&nickname, "is_drinking") {
        if let Some(entity) = get_nearest_entity(&bot, EntityFilter::new(&bot, &config.target, config.distance)) {
          STATES.set_mutual_states(&nickname, "attacking", true);

          if options.use_auto_weapon {
            self.auto_weapon(bot, &options.weapon).await;
          } else {
            if let Some(slot) = config.weapon_slot {
              if slot <= 8 {
                bot.set_selected_hotbar_slot(slot);
              } else {
                self.auto_weapon(bot, &options.weapon).await;
              }
            } else {
              self.auto_weapon(bot, &options.weapon).await;
            }
          }

          if options.use_critical {
            bot.jump();

            sleep(Duration::from_millis(randuint(500, 600))).await;

            if let Some(e) = get_nearest_entity(&bot, EntityFilter::new(&bot, &config.target, config.distance)) {
              bot.attack(e);
            }
          } else {
            bot.attack(entity);
          }

          STATES.set_mutual_states(&nickname, "attacking", false);
        }
      }
      
      sleep(Duration::from_millis(config.delay)).await;
    }
  }

  async fn aggressive_killaura(&'static self, bot: &Client, options: &KillauraOptions) {
    let config = if options.settings.as_str() == "adaptive" {
      self.create_adaptive_config(options.target.clone(), options.chase_distance, options.min_distance_to_target)
    } else {
      KillauraConfig { 
        target: options.target.clone(), 
        weapon_slot: options.weapon_slot, 
        distance: options.distance.unwrap_or(3.1), 
        delay: options.delay.unwrap_or(150),
        chase_distance: options.chase_distance.unwrap_or(10.0),
        min_distance_to_target: options.min_distance_to_target.unwrap_or(3.0)
      }
    };

    if options.use_chase {
      self.chase(bot.clone(), config.target.clone(), config.chase_distance, config.min_distance_to_target);
    }

    if options.use_dodging {
      self.dodging(bot.clone());
    }

    let nickname = bot.username();

    loop {
      if STATES.get_state(&nickname, "can_attacking") && !STATES.get_state(&nickname, "is_interacting") && !STATES.get_state(&nickname, "is_eating") && !STATES.get_state(&nickname, "is_drinking") {
        if let Some(entity) = get_nearest_entity(&bot, EntityFilter::new(&bot, &config.target, config.distance)) {
          STATES.set_mutual_states(&nickname, "attacking", true);
          
          if options.use_auto_weapon {
            self.auto_weapon(bot, &options.weapon).await;
          } else {
            if let Some(slot) = config.weapon_slot {
              if slot <= 8 {
                bot.set_selected_hotbar_slot(slot);
              } else {
                self.auto_weapon(bot, &options.weapon).await;
              }
            } else {
              self.auto_weapon(bot, &options.weapon).await;
            }
          }

          if options.use_critical {
            bot.jump();
            sleep(Duration::from_millis(randuint(500, 600))).await;

            if let Some(e) = get_nearest_entity(&bot, EntityFilter::new(&bot, &config.target, config.distance)) {
              bot.attack(e);
            }
          } else {
            bot.attack(entity);
          }

          STATES.set_mutual_states(&nickname, "attacking", false);
        }
      }

      sleep(Duration::from_millis(config.delay)).await;
    }
  }

  pub async fn enable(&'static self, bot: &Client, options: &KillauraOptions) {
    match options.mode.as_str() {
      "moderate" => { self.moderate_killaura(bot, options).await; },
      "aggressive" => { self.aggressive_killaura(bot, options).await; },
      _ => {}
    }
  }

  pub fn stop(&self, bot: &Client) {
    let nickname = bot.username();

    kill_task(&nickname, "killaura");

    bot.walk(WalkDirection::None);

    STATES.set_mutual_states(&nickname, "looking", false);
    STATES.set_mutual_states(&nickname, "attacking", false);
    STATES.set_mutual_states(&nickname, "walking", false);
    STATES.set_mutual_states(&nickname, "sprinting", false);
  }
}