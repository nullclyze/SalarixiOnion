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
use crate::tools::{randfloat, randticks, randuint};
use crate::common::{convert_inventory_slot_to_hotbar_slot, find_empty_slot_in_hotbar, get_bot_physics};


#[derive(Clone)]
struct Potion {
  kind: String,
  slot: Option<u16>,
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

    if health.0 < 20.0 {
      let potions = Self::find_potion_in_inventory(bot);

      if potions.len() > 0 {
        let inventory = bot.get_inventory();

        if let Some(potion) = Self::get_best_potion(bot, potions) {
          if let Some(slot) = potion.slot {
            if !STATES.get_plugin_activity(&bot.username(), "auto-eat") {
              STATES.set_plugin_activity(&bot.username(), "auto-potion", true);

              if let Some(hotbar_slot) = convert_inventory_slot_to_hotbar_slot(slot as usize) {
                if bot.selected_hotbar_slot() != hotbar_slot {
                  bot.set_selected_hotbar_slot(hotbar_slot);
                }

                Self::use_potion(bot, potion.kind).await;
              } else {
                if let Some(empty_slot) = find_empty_slot_in_hotbar(bot) {
                  inventory.left_click(slot);
                  bot.wait_ticks(randticks(1, 2)).await;
                  inventory.left_click(empty_slot);

                  if let Some(slot) = convert_inventory_slot_to_hotbar_slot(empty_slot as usize) {
                    if bot.selected_hotbar_slot() != slot {
                      bot.set_selected_hotbar_slot(slot);
                      bot.wait_ticks(1).await;
                    }

                    Self::use_potion(bot, potion.kind).await;
                  }
                } else {
                  let random_slot = randuint(36, 44) as usize;

                  inventory.shift_click(random_slot);

                  bot.wait_ticks(1).await;

                  inventory.left_click(slot);
                  bot.wait_ticks(randticks(1, 2)).await;
                  inventory.left_click(random_slot);

                  let hotbar_slot = convert_inventory_slot_to_hotbar_slot(random_slot).unwrap_or(0);

                  if bot.selected_hotbar_slot() != hotbar_slot {
                    bot.set_selected_hotbar_slot(hotbar_slot);
                  }

                  bot.wait_ticks(1).await;

                  Self::use_potion(bot, potion.kind).await;
                }
              }

              STATES.set_plugin_activity(&bot.username(), "auto-potion", false);
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
        bot.set_direction(bot.direction().0 + randfloat(-5.5, 5.5) as f32, randfloat(87.0, 90.0) as f32);

        bot.wait_ticks(randticks(1, 2)).await;

        let direction = bot.direction();

        bot.write_packet(ServerboundUseItem {
          hand: InteractionHand::MainHand,
          seq: 0,
          y_rot: direction.0,
          x_rot: direction.1
        });

        bot.wait_ticks(randticks(1, 2)).await;

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

    let mut potion = None;

    if is_in_lava {
      for p in potions.clone() {
        match p.name {
          PotionKind::FireResistance => {
            potion = Some(p);
            break;
          },
          PotionKind::LongFireResistance => {
            potion = Some(p);
            break;
          },
          _ => {}
        }
      }
    }

    if potion.is_none() && is_in_water {
      for p in potions.clone() {
        match p.name {
          PotionKind::WaterBreathing => {
            potion = Some(p);
            break;
          },
          PotionKind::LongWaterBreathing => {
            potion = Some(p);
            break;
          },
          _ => {}
        }
      }
    }

    if potion.is_none() && !on_ground && velocity_y < -0.5 {
      for p in potions.clone() {
        match p.name {
          PotionKind::SlowFalling => {
            potion = Some(p);
            break;
          },
          PotionKind::LongSlowFalling => {
            potion = Some(p);
            break;
          },
          _ => {}
        }
      }
    }

    if potion.is_none() && health.0 <= 8.0 {
      for p in potions.clone() {
        match p.name {
          PotionKind::TurtleMaster => {
            potion = Some(p);
            break;
          },
          PotionKind::LongTurtleMaster => {
            potion = Some(p);
            break;
          },
          PotionKind::StrongTurtleMaster => {
            potion = Some(p);
            break;
          },
          _ => {}
        }
      }
    }

    if potion.is_none() && health.0 <= 15.0 {
      for p in potions.clone() {
        match p.name {
          PotionKind::Regeneration => {
            potion = Some(p);
            break;
          },
          PotionKind::LongRegeneration => {
            potion = Some(p);
            break;
          },
          PotionKind::StrongRegeneration => {
            potion = Some(p);
            break;
          },
          PotionKind::Healing => {
            potion = Some(p);
            break;
          },
          PotionKind::StrongHealing => {
            potion = Some(p);
            break;
          },
          _ => {}
        }
      }
    }

    potion
  }

  fn find_potion_in_inventory(bot: &Client) -> Vec<Potion> {
    let mut potion_list = vec![];

    for (slot, item) in bot.menu().slots().iter().enumerate() {
      if let Some(potion) = Self::is_potion(Some(slot as u16), item) {
        potion_list.push(potion);
      }
    }

    potion_list
  }

  fn is_potion(slot: Option<u16>, item: &ItemStack) -> Option<Potion> {
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
