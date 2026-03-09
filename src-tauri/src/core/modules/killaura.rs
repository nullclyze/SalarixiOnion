use azalea::ecs::entity::Entity;
use azalea::registry::builtin::ItemKind;
use azalea::{prelude::*, SprintDirection};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

use crate::core::*;
use crate::generators::*;
use crate::extensions::{BotDefaultExt, BotInventoryExt, BotMovementExt, BotRotationExt, entity_type_from};

#[derive(Debug)]
struct Weapon {
  slot: Option<usize>,
  priority: u8,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KillauraModule;

#[derive(Debug, Serialize, Deserialize)]
pub struct KillauraOptions {
  pub behavior: String,
  pub settings: String,
  pub target: String,
  pub target_nickname: Option<String>,
  pub weapon: String,
  pub weapon_slot: Option<u8>,
  pub attack_distance: Option<f64>,
  pub delay: Option<u64>,
  pub chase_distance: Option<f64>,
  pub min_distance_to_target: Option<f64>,
  pub use_auto_weapon: bool,
  pub use_dodging: bool,
  pub use_chase: bool,
  pub use_critical: bool,
  pub state: bool,
}

#[derive(Debug)]
struct KillauraConfig {
  target: String,
  weapon_slot: Option<u8>,
  attack_distance: f64,
  delay: u64,
  chase_distance: f64,
  min_distance_to_target: f64,
}

impl KillauraModule {
  pub fn new() -> Self {
    Self
  }

  fn create_config(&self, options: &KillauraOptions) -> KillauraConfig {
    if options.settings.as_str() == "adaptive" {
      KillauraConfig {
        target: options.target.clone(),
        weapon_slot: None,
        attack_distance: 3.1,
        delay: 500,
        chase_distance: options.chase_distance.unwrap_or(10.0),
        min_distance_to_target: options.min_distance_to_target.unwrap_or(3.0),
      }
    } else {
      KillauraConfig {
        target: options.target.clone(),
        weapon_slot: options.weapon_slot,
        attack_distance: options.attack_distance.unwrap_or(3.1),
        delay: options.delay.unwrap_or(500),
        chase_distance: options.chase_distance.unwrap_or(10.0),
        min_distance_to_target: options.min_distance_to_target.unwrap_or(3.0),
      }
    }
  }

  fn find_nearest_entity(
    bot: &Client,
    target: String,
    target_nickname: Option<String>,
    max_distance: f64,
  ) -> Option<Entity> {
    let target = if target == "custom" {
      target_nickname
    } else {
      Some(target)
    };

    let Some(target) = target else {
      return None;
    };

    bot.find_nearest_entity(entity_type_from(target), max_distance)
  }

  async fn auto_weapon(&self, bot: &Client, weapon: &String) {
    let mut weapons = vec![];

    if let Some(menu) = bot.get_inventory_menu() {
      for (slot, item) in menu.slots().iter().enumerate() {
        if !item.is_empty() {
          match weapon.as_str() {
            "sword" => match item.kind() {
              ItemKind::WoodenSword => {
                weapons.push(Weapon {
                  slot: Some(slot),
                  priority: 0,
                });
              }
              ItemKind::GoldenSword => {
                weapons.push(Weapon {
                  slot: Some(slot),
                  priority: 1,
                });
              }
              ItemKind::StoneSword => {
                weapons.push(Weapon {
                  slot: Some(slot),
                  priority: 2,
                });
              }
              ItemKind::CopperSword => {
                weapons.push(Weapon {
                  slot: Some(slot),
                  priority: 3,
                });
              }
              ItemKind::IronSword => {
                weapons.push(Weapon {
                  slot: Some(slot),
                  priority: 4,
                });
              }
              ItemKind::DiamondSword => {
                weapons.push(Weapon {
                  slot: Some(slot),
                  priority: 5,
                });
              }
              ItemKind::NetheriteSword => {
                weapons.push(Weapon {
                  slot: Some(slot),
                  priority: 6,
                });
              }
              _ => {}
            },
            "axe" => match item.kind() {
              ItemKind::WoodenAxe => {
                weapons.push(Weapon {
                  slot: Some(slot),
                  priority: 0,
                });
              }
              ItemKind::GoldenAxe => {
                weapons.push(Weapon {
                  slot: Some(slot),
                  priority: 1,
                });
              }
              ItemKind::StoneAxe => {
                weapons.push(Weapon {
                  slot: Some(slot),
                  priority: 2,
                });
              }
              ItemKind::CopperAxe => {
                weapons.push(Weapon {
                  slot: Some(slot),
                  priority: 3,
                });
              }
              ItemKind::IronAxe => {
                weapons.push(Weapon {
                  slot: Some(slot),
                  priority: 4,
                });
              }
              ItemKind::DiamondAxe => {
                weapons.push(Weapon {
                  slot: Some(slot),
                  priority: 5,
                });
              }
              ItemKind::NetheriteAxe => {
                weapons.push(Weapon {
                  slot: Some(slot),
                  priority: 6,
                });
              }
              _ => {}
            },
            _ => {}
          }
        }
      }
    }

    let mut best_weapon = Weapon {
      slot: None,
      priority: 0,
    };

    for w in weapons {
      if w.priority > best_weapon.priority {
        best_weapon = w;
      }
    }

    if let Some(slot) = best_weapon.slot {
      bot.take_item(slot, true).await;
    }
  }

  fn dodging(&self, username: String) {
    tokio::spawn(async move {
      loop {
        let bot_available = BOT_REGISTRY
          .async_get_bot(&username, async |bot| {
            if !TASKS.get_task_activity(&username, "killaura") {
              bot.stop_crouching();
              return;
            }

            bot.start_crouching();
            sleep(Duration::from_millis(randuint(150, 250))).await;
            bot.stop_crouching();
          })
          .await
          .is_some();

        if !bot_available || !TASKS.get_task_activity(&username, "killaura") {
          break;
        }

        sleep(Duration::from_millis(50)).await;
      }
    });
  }

  fn chase(
    &self,
    username: String,
    target: String,
    target_nickname: Option<String>,
    distance: f64,
    min_distance_to_target: f64,
  ) {
    tokio::spawn(async move {
      loop {
        if let Some(bot) = BOT_REGISTRY.get_bot(&username) {
          if !TASKS.get_task_activity(&username, "killaura") {
            bot.stop_move();
            return;
          }

          if let Some(entity) = Self::find_nearest_entity(&bot, target.clone(), target_nickname.clone(), distance)
          {
            let eye_pos = bot.eye_pos();

            if eye_pos.distance_to(bot.get_entity_position(entity)) > min_distance_to_target {
              bot.start_jumping();
              bot.start_sprinting(SprintDirection::Forward);

              bot.look_at_entity(entity, true);
            } else {
              bot.stop_jumping();

              if get_state(&username, "is_sprinting") {
                bot.stop_move();
              }
            }
          } else {
            bot.stop_jumping();

            if get_state(&username, "is_sprinting") {
              bot.stop_move();
            }
          }
        }

        if !TASKS.get_task_activity(&username, "killaura") {
          break;
        }

        sleep(Duration::from_millis(50)).await;
      }
    });
  }

  fn aiming(
    &self,
    username: String,
    target: String,
    target_nickname: Option<String>,
    distance: f64,
  ) {
    tokio::spawn(async move {
      set_mutual_states(&username, "looking", true);

      loop {
        if let Some(bot) = BOT_REGISTRY.get_bot(&username) {
          if !TASKS.get_task_activity(&username, "killaura") {
            set_mutual_states(&username, "looking", false);
            return;
          }

          if let Some(entity) = Self::find_nearest_entity(&bot, target.clone(), target_nickname.clone(), distance)
          {
            bot.look_at_entity(entity, false);
          }
        }

        if !TASKS.get_task_activity(&username, "killaura") {
          set_mutual_states(&username, "looking", false);
          break;
        }

        sleep(Duration::from_millis(50)).await;
      }
    });
  }

  async fn moderate_killaura(&self, bot: &Client, options: &KillauraOptions) {
    let config = self.create_config(options);

    self.aiming(
      bot.username(),
      config.target.clone(),
      options.target_nickname.clone(),
      config.attack_distance,
    );

    if options.use_chase {
      self.chase(
        bot.username(),
        config.target.clone(),
        options.target_nickname.clone(),
        config.chase_distance,
        config.min_distance_to_target,
      );
    }

    if options.use_dodging {
      self.dodging(bot.username());
    }

    let nickname = bot.username();

    loop {
      if get_state(&nickname, "can_attacking")
        && !get_state(&nickname, "is_interacting")
        && !get_state(&nickname, "is_eating")
        && !get_state(&nickname, "is_drinking")
      {
        if let Some(entity) = Self::find_nearest_entity(
          bot,
          options.target.clone(),
          options.target_nickname.clone(),
          config.attack_distance,
        ) {
          set_mutual_states(&nickname, "attacking", true);

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

            if let Some(e) = Self::find_nearest_entity(
              bot,
              options.target.clone(),
              options.target_nickname.clone(),
              config.attack_distance,
            ) {
              bot.attack(e);
            }
          } else {
            bot.attack(entity);
          }

          set_mutual_states(&nickname, "attacking", false);
        }
      }

      sleep(Duration::from_millis(config.delay)).await;
    }
  }

  async fn aggressive_killaura(&self, bot: &Client, options: &KillauraOptions) {
    let config = self.create_config(options);

    if options.use_chase {
      self.chase(
        bot.username(),
        config.target.clone(),
        options.target_nickname.clone(),
        config.chase_distance,
        config.min_distance_to_target,
      );
    }

    if options.use_dodging {
      self.dodging(bot.username());
    }

    let nickname = bot.username();

    loop {
      if get_state(&nickname, "can_attacking")
        && !get_state(&nickname, "is_interacting")
        && !get_state(&nickname, "is_eating")
        && !get_state(&nickname, "is_drinking")
      {
        if let Some(entity) = Self::find_nearest_entity(
          bot,
          options.target.clone(),
          options.target_nickname.clone(),
          config.attack_distance,
        ) {
          set_mutual_states(&nickname, "attacking", true);

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

            if let Some(e) = Self::find_nearest_entity(
              bot,
              options.target.clone(),
              options.target_nickname.clone(),
              config.attack_distance,
            ) {
              bot.attack(e);
            }
          } else {
            bot.attack(entity);
          }

          set_mutual_states(&nickname, "attacking", false);
        }
      }

      sleep(Duration::from_millis(config.delay)).await;
    }
  }

  pub async fn enable(&self, username: &str, options: &KillauraOptions) {
    BOT_REGISTRY
      .async_get_bot(username, async |bot| match options.behavior.as_str() {
        "moderate" => {
          self.moderate_killaura(bot, options).await;
        }
        "aggressive" => {
          self.aggressive_killaura(bot, options).await;
        }
        _ => {}
      })
      .await;
  }

  pub async fn stop(&self, username: &str) {
    kill_task(username, "killaura");

    if let Some(bot) = BOT_REGISTRY.get_bot(username) {
      bot.stop_move();
      bot.stop_crouching();
      bot.stop_jumping();
    }

    set_mutual_states(username, "looking", false);
    set_mutual_states(username, "attacking", false);
  }
}
