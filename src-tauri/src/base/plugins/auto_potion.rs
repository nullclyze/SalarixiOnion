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
use crate::tools::{randticks, randuint};
use crate::common::{convert_inventory_slot_to_hotbar_slot, find_empty_slot_in_hotbar, get_bot_physics};


#[derive(Clone)]
struct Potion {
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

    let physics = get_bot_physics(bot);

    let is_in_lava = physics.is_in_lava();
    let is_in_water = physics.is_in_water();
    let on_ground = physics.on_ground();
    let velocity_y = physics.velocity.y;

    if health.0 < 20.0 {
      let potions = Self::find_potion_in_inventory(bot);

      if potions.len() > 0 {
        let inventory = bot.get_inventory();

        let mut potion_slot = None;

        if is_in_lava {
          for potion in potions.clone() {
            match potion.name {
              PotionKind::FireResistance => {
                potion_slot = potion.slot;
                break;
              },
              PotionKind::LongFireResistance => {
                potion_slot = potion.slot;
                break;
              },
              _ => {}
            }
          }
        }

        if potion_slot.is_none() && is_in_water {
          for potion in potions.clone() {
            match potion.name {
              PotionKind::WaterBreathing => {
                potion_slot = potion.slot;
                break;
              },
              PotionKind::LongWaterBreathing => {
                potion_slot = potion.slot;
                break;
              },
              _ => {}
            }
          }
        }

        if potion_slot.is_none() && !on_ground && velocity_y < -0.5 {
          for potion in potions.clone() {
            match potion.name {
              PotionKind::SlowFalling => {
                potion_slot = potion.slot;
                break;
              },
              PotionKind::LongSlowFalling => {
                potion_slot = potion.slot;
                break;
              },
              _ => {}
            }
          }
        }

        if potion_slot.is_none() && health.0 < 15.0 {
          for potion in potions.clone() {
            match potion.name {
              PotionKind::Regeneration => {
                potion_slot = potion.slot;
                break;
              },
              PotionKind::LongRegeneration => {
                potion_slot = potion.slot;
                break;
              },
              PotionKind::StrongRegeneration => {
                potion_slot = potion.slot;
                break;
              },
              PotionKind::Healing => {
                potion_slot = potion.slot;
                break;
              },
              PotionKind::StrongHealing => {
                potion_slot = potion.slot;
                break;
              },
              _ => {}
            }
          }
        }

        if let Some(slot) = potion_slot {
          if !STATES.get_plugin_activity(&bot.username(), "auto-eat") {
            STATES.set_plugin_activity(&bot.username(), "auto-potion", true);

            if let Some(hotbar_slot) = convert_inventory_slot_to_hotbar_slot(slot as usize) {
              if bot.selected_hotbar_slot() != hotbar_slot {
                bot.set_selected_hotbar_slot(hotbar_slot);
              }

              Self::start_drinking(bot).await;
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

                  Self::start_drinking(bot).await;
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

                Self::start_drinking(bot).await;
              }
            }

            STATES.set_plugin_activity(&bot.username(), "auto-potion", false);
          }
        }
      }
    }
  }

  async fn start_drinking(bot: &Client) {
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
