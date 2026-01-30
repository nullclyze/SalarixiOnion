use azalea::entity::metadata::Health;
use azalea::inventory::ItemStack;
use azalea::inventory::components::PotionContents;
use azalea::registry::builtin::Potion as PotionKind;
use azalea::{BlockPos, prelude::*};
use azalea::protocol::packets::game::{ServerboundPlayerAction, ServerboundUseItem};
use azalea::protocol::packets::game::s_interact::InteractionHand;
use azalea::registry::builtin::ItemKind;
use azalea::protocol::packets::game::s_player_action::Action;
use std::time::Duration;
use tokio::time::sleep;

use crate::base::get_flow_manager;
use crate::state::STATES;
use crate::tasks::TASKS;
use crate::tools::{randchance, randfloat, randticks};
use crate::common::{get_bot_physics, move_item_to_hotbar};


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
            if !STATES.get_plugin_activity(&nickname, "auto-eat") {
              let mut should_drink = true;

              if TASKS.get_task_activity(&nickname, "killaura") {
                should_drink = randchance(0.7);
              }

              if should_drink {
                STATES.set_plugin_activity(&nickname, "auto-potion", true);

                move_item_to_hotbar(bot, slot).await;
                bot.wait_ticks(randticks(1, 2)).await;
                Self::use_potion(bot, potion.kind).await;

                STATES.set_plugin_activity(&nickname, "auto-potion", false);
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
        let direction = bot.direction();

        bot.write_packet(ServerboundUseItem {
          hand: InteractionHand::MainHand,
          seq: 0,
          y_rot: direction.0,
          x_rot: direction.1
        });

        sleep(Duration::from_millis(3000)).await;

        bot.write_packet(ServerboundPlayerAction {
          action: Action::ReleaseUseItem,
          pos: BlockPos::new(0, 0, 0),
          direction: azalea::core::direction::Direction::Down,
          seq: 0
        });
      },
      "splash" => {
        let direction = bot.direction();

        bot.set_direction(direction.0 + randfloat(-5.5, 5.5) as f32, randfloat(87.0, 90.0) as f32);

        bot.wait_ticks(randticks(15, 20)).await;

        bot.write_packet(ServerboundUseItem {
          hand: InteractionHand::MainHand,
          seq: 0,
          y_rot: direction.0,
          x_rot: direction.1
        });

        bot.wait_ticks(randticks(4, 7)).await;

        bot.set_direction(direction.0 + randfloat(-2.5, 2.5) as f32, direction.1 + randfloat(-2.5, 2.5) as f32);
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
