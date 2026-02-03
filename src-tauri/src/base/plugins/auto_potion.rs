use azalea::entity::metadata::Health;
use azalea::inventory::ItemStack;
use azalea::inventory::components::PotionContents;
use azalea::registry::builtin::Potion as PotionKind;
use azalea::prelude::*;
use azalea::protocol::packets::game::s_interact::InteractionHand;
use azalea::registry::builtin::ItemKind;
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::common::{start_use_item, stop_bot_sprinting};
use crate::tools::*;
use crate::common::{get_bot_physics, take_item, release_use_item};


#[derive(Clone)]
struct Potion {
  kind: String,
  slot: Option<usize>,
  name: PotionKind
}

pub struct AutoPotionPlugin;

impl AutoPotionPlugin {
  pub fn enable(bot: Client) {
    tokio::spawn(async move {
      loop {
        if let Some(arc) = get_flow_manager() {
          if !arc.read().active {
            break;
          }
        }

        Self::drink(&bot).await;

        sleep(Duration::from_millis(50)).await;
      }
    });
  } 

  async fn drink(bot: &Client) {
    let health = if let Some(health) = bot.get_component::<Health>() {
      health
    } else {
      return;
    };

    let nickname = bot.username();

    if health.0 < 20.0 {
      let potions = Self::find_potion_in_inventory(bot);

      if potions.len() > 0 {
        if let Some(potion) = Self::get_best_potion(bot, potions) {
          if let Some(slot) = potion.slot {
            if health.0 < 10.0 && !STATES.get_state(&nickname, "is_eating") {
              STATES.set_state(&nickname, "can_eating", false);
            } else {
              STATES.set_state(&nickname, "can_eating", true);
            }

            if STATES.get_state(&nickname, "can_drinking") && !STATES.get_state(&nickname, "is_eating") && !STATES.get_state(&nickname, "is_sprinting") {
              let mut should_drink = true;

              if TASKS.get_task_activity(&nickname, "killaura") {
                should_drink = !randchance(health.0 as f64 / 20.0);
              }

              if should_drink {
                if STATES.get_state(&nickname, "is_sprinting") || STATES.get_state(&nickname, "can_sprinting") {
                  stop_bot_sprinting(bot).await;
                  sleep(Duration::from_millis(randuint(50, 100))).await;
                }

                STATES.set_state(&nickname, "can_eating", false);
                STATES.set_state(&nickname, "is_drinking", true);

                take_item(bot, slot).await;
                sleep(Duration::from_millis(50)).await;
                Self::use_potion(bot, potion.kind).await;
                sleep(Duration::from_millis(50)).await;

                STATES.set_state(&nickname, "can_eating", true);
                STATES.set_state(&nickname, "can_sprinting", true);
                STATES.set_state(&nickname, "is_drinking", false);
              }
            }
          }
        }
      }
    }
  }

  async fn use_potion(bot: &Client, kind: String) {
    match kind.as_str() {
      "default" => {
        start_use_item(bot, InteractionHand::MainHand);
        sleep(Duration::from_millis(2700)).await;
        release_use_item(bot);
      },
      "splash" => {
        let nickname = bot.username();

        STATES.set_state(&nickname, "can_looking", false);
        STATES.set_state(&nickname, "is_looking", true);

        let direction = bot.direction();

        bot.set_direction(direction.0 + randfloat(-5.5, 5.5) as f32, randfloat(87.0, 90.0) as f32);

        sleep(Duration::from_millis(randuint(400, 600))).await;

        start_use_item(bot, InteractionHand::MainHand);

        sleep(Duration::from_millis(randuint(30, 500))).await;

        bot.set_direction(direction.0 + randfloat(-2.5, 2.5) as f32, direction.1 + randfloat(-2.5, 2.5) as f32);
      
        STATES.set_state(&nickname, "is_looking", false);
        STATES.set_state(&nickname, "can_looking", true);
      },
      _ => {}
    }
  }

  fn get_best_potion(bot: &Client, potions: Vec<Potion>) -> Option<Potion> {
    let health = if let Some(health) = bot.get_component::<Health>() {
      health
    } else {
      return None;
    };

    let physics = get_bot_physics(bot);

    let is_in_lava = physics.is_in_lava();
    let is_in_water = physics.is_in_water();
    let on_ground = physics.on_ground();
    let velocity_y = physics.velocity.y;

    let mut best_potion = None;

    for p in potions {
      if best_potion.clone().unwrap_or(Potion { kind: "deafult".to_string(), slot: Some(0), name: PotionKind::Awkward }).kind.as_str() != "splash" {
        if is_in_lava {
          match p.name {
            PotionKind::FireResistance => {
              best_potion = Some(p.clone());
            },
            PotionKind::LongFireResistance => {
              best_potion = Some(p.clone());
            },
            _ => {}
          }
        }

        if best_potion.is_none() && is_in_water {
          match p.name {
            PotionKind::WaterBreathing => {
              best_potion = Some(p.clone());
            },
            PotionKind::LongWaterBreathing => {
              best_potion = Some(p.clone());
            },
            _ => {}
          }
        }

        if best_potion.is_none() && !on_ground && velocity_y < -0.5 {
          match p.name {
            PotionKind::SlowFalling => {
              best_potion = Some(p.clone());
            },
            PotionKind::LongSlowFalling => {
              best_potion = Some(p.clone());
            },
            _ => {}
          }
        }

        if best_potion.is_none() && health.0 <= 8.0 {
          match p.name {
            PotionKind::TurtleMaster => {
              best_potion = Some(p.clone());
            },
            PotionKind::LongTurtleMaster => {
              best_potion = Some(p.clone());
            },
            PotionKind::StrongTurtleMaster => {
              best_potion = Some(p.clone());
            },
            _ => {}
          }
        }

        if best_potion.is_none() && health.0 <= 15.0 {
          match p.name {
            PotionKind::Regeneration => {
              best_potion = Some(p);
              break;
            },
            PotionKind::LongRegeneration => {
              best_potion = Some(p);
              break;
            },
            PotionKind::StrongRegeneration => {
              best_potion = Some(p);
              break;
            },
            PotionKind::Healing => {
              best_potion = Some(p);
              break;
            },
            PotionKind::StrongHealing => {
              best_potion = Some(p);
              break;
            },
            _ => {}
          }
        }

        if p.kind.as_str() == "splash" {
          break;
        }
      }
    }

    best_potion
  }

  fn find_potion_in_inventory(bot: &Client) -> Vec<Potion> {
    let mut potion_list = vec![];

    for (slot, item) in bot.menu().slots().iter().enumerate() {
      if let Some(potion) = Self::is_potion(Some(slot), item) {
        potion_list.push(potion);
      }
    }

    potion_list
  }

  fn is_potion(slot: Option<usize>, item: &ItemStack) -> Option<Potion> {
    match item.kind() {
      ItemKind::Potion => {
        if let Some(contents) = item.get_component::<PotionContents>() {
          if let Some(potion) = contents.potion {
            return Some(Potion { 
              kind: "default".to_string(),
              slot: slot, 
              name: potion
            });
          }
        }
      },
      ItemKind::SplashPotion => {
        if let Some(contents) = item.get_component::<PotionContents>() {
          if let Some(potion) = contents.potion {
            return Some(Potion { 
              kind: "splash".to_string(),
              slot: slot, 
              name: potion
            });
          }
        }
      },
      _ => {}
    }

    None
  }
}
