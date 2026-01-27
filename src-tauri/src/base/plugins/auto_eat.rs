use azalea::inventory::ItemStack;
use azalea::prelude::*;
use azalea::prelude::ContainerClientExt;
use azalea::protocol::packets::game::ServerboundUseItem;
use azalea::protocol::packets::game::s_interact::InteractionHand;
use azalea::registry::builtin::ItemKind;
use std::time::Duration;
use tokio::time::sleep;

use crate::base::get_flow_manager;
use crate::tools::randuint;
use crate::common::{find_empty_slot_in_hotbar, convert_inventory_slot_to_hotbar_slot};


#[derive(Clone)]
struct Food {
  slot: Option<u16>,
  priority: u8
}

pub struct AutoEatPlugin;

impl AutoEatPlugin {
  pub fn enable(bot: Client) {
    tokio::spawn(async move {
      loop {
        if let Some(arc) = get_flow_manager() {
          if !arc.read().active {
            break;
          }

          Self::eat(&bot).await;
        }

        sleep(Duration::from_millis(50)).await;
      }
    });
  } 

  async fn eat(bot: &Client) {
    let hunger = bot.hunger();

    if hunger.food < 20 {
      let food_list = Self::find_food_in_inventory(bot);

      if let Some(best_food) = Self::get_best_food(bot, food_list.clone()) {
        if let Some(food_slot) = best_food.slot {
          if let Some(empty_slot) = find_empty_slot_in_hotbar(bot) {
            let inventory = bot.get_inventory();

            inventory.left_click(food_slot);
            bot.wait_ticks(randuint(1, 2) as usize).await;
            inventory.left_click(empty_slot);

            if let Some(slot) = convert_inventory_slot_to_hotbar_slot(empty_slot as usize) {
              if bot.selected_hotbar_slot() != slot {
                bot.set_selected_hotbar_slot(slot);
                bot.wait_ticks(2).await;
              }

              Self::start_eating(bot).await;
            }
          } else {
            if food_slot >= 36 && food_slot <= 44 {
              if let Some(hotbar_slot) = convert_inventory_slot_to_hotbar_slot(food_slot as usize) {
                if bot.selected_hotbar_slot() != hotbar_slot {
                  bot.set_selected_hotbar_slot(hotbar_slot);
                  bot.wait_ticks(2).await;
                }

                Self::start_eating(bot).await;
              }
            } else {
              for food in food_list.clone() {
                if let Some(slot) = food.slot {
                  if slot >= 36 && slot <= 44 {
                    if let Some(hotbar_slot) = convert_inventory_slot_to_hotbar_slot(slot as usize) {
                      if bot.selected_hotbar_slot() != hotbar_slot {
                        bot.set_selected_hotbar_slot(hotbar_slot);
                        bot.wait_ticks(2).await;
                      }

                      Self::start_eating(bot).await;
                    }
                  }
                }
              }
            }
          }
        }
      }
    }
  }

  async fn start_eating(bot: &Client) {
    let direction = bot.direction();

    let packet = ServerboundUseItem {
      hand: InteractionHand::MainHand,
      seq: 0,
      y_rot: direction.0,
      x_rot: direction.1
    };

    bot.write_packet(packet);

    sleep(Duration::from_millis(4000)).await;
  }

  fn get_best_food(bot: &Client, food_list: Vec<Food>) -> Option<Food> {
    let health = bot.health() as u32;
    let hunger = bot.hunger().food;

    let mut best_food = None;

    if food_list.len() > 1 {
      let desired_health_priority;
      let desired_hunger_priority;

      if health < 20 && health >= 18 {
        desired_health_priority = 0;
      } else if health < 18 && health >= 14 {
        desired_health_priority = 1;
      } else if health < 14 && health >= 10 {
        desired_health_priority = 2;
      } else {
        desired_health_priority = 3;
      }

      if hunger < 20 && hunger >= 18 {
        desired_hunger_priority = 0;
      } else if hunger < 18 && hunger >= 14 {
        desired_hunger_priority = 1;
      } else if hunger < 14 && hunger >= 10 {
        desired_hunger_priority = 2;
      } else {
        desired_hunger_priority = 3;
      }

      let mut current_priority;

      if desired_health_priority > desired_hunger_priority {
        current_priority = desired_health_priority;
      } else {
        current_priority = (desired_health_priority + desired_hunger_priority) / 2;
      }

      for attempt in 0..3 {
        for food in food_list.clone() {
          if food.priority == current_priority {
            best_food = Some(food);
            break;
          }
        }

        if attempt > 0 {
          if current_priority + 1 <= 3 {
            current_priority += 1;
          } else {
            current_priority -= 1;
          }
        }
      }
    } else {
      best_food = food_list.get(0).cloned();
    }

    best_food
  }

  fn find_food_in_inventory(bot: &Client) -> Vec<Food> {
    let mut food_list = vec![];

    for (slot, item) in bot.menu().slots().iter().enumerate() {
      if let Some(food) = Self::is_food(Some(slot as u16), item) {
        food_list.push(food);
      }
    }

    food_list
  }

  fn is_food(slot: Option<u16>, item: &ItemStack) -> Option<Food> {
    match item.kind() {
      ItemKind::GoldenApple => return Some(Food { slot: slot, priority: 3 }),
      ItemKind::EnchantedGoldenApple => return Some(Food { slot: slot, priority: 3 }),
      ItemKind::GoldenCarrot => return Some(Food { slot: slot, priority: 3 }),

      ItemKind::CookedChicken => return Some(Food { slot: slot, priority: 2 }),
      ItemKind::CookedBeef => return Some(Food { slot: slot, priority: 2 }),
      ItemKind::CookedCod => return Some(Food { slot: slot, priority: 2 }),
      ItemKind::CookedMutton => return Some(Food { slot: slot, priority: 2 }),
      ItemKind::CookedPorkchop => return Some(Food { slot: slot, priority: 2 }),
      ItemKind::CookedRabbit => return Some(Food { slot: slot, priority: 2 }),
      ItemKind::CookedSalmon => return Some(Food { slot: slot, priority: 2 }),
      ItemKind::BakedPotato => return Some(Food { slot: slot, priority: 2 }),
      ItemKind::MushroomStew => return Some(Food { slot: slot, priority: 2 }),
      ItemKind::RabbitStew => return Some(Food { slot: slot, priority: 2 }),
      ItemKind::BeetrootSoup => return Some(Food { slot: slot, priority: 2 }),
      ItemKind::PumpkinPie => return Some(Food { slot: slot, priority: 2 }),

      ItemKind::Bread => return Some(Food { slot: slot, priority: 1 }),
      ItemKind::Chicken => return Some(Food { slot: slot, priority: 1 }),
      ItemKind::Beef => return Some(Food { slot: slot, priority: 1 }),
      ItemKind::Cod => return Some(Food { slot: slot, priority: 1 }),
      ItemKind::Mutton => return Some(Food { slot: slot, priority: 1 }),
      ItemKind::Porkchop => return Some(Food { slot: slot, priority: 1 }),
      ItemKind::Rabbit => return Some(Food { slot: slot, priority: 1 }),
      ItemKind::Apple => return Some(Food { slot: slot, priority: 1 }),

      ItemKind::Salmon => return Some(Food { slot: slot, priority: 0 }),
      ItemKind::TropicalFish => return Some(Food { slot: slot, priority: 0 }),
      ItemKind::Potato => return Some(Food { slot: slot, priority: 0 }),
      ItemKind::ChorusFruit => return Some(Food { slot: slot, priority: 0 }),
      ItemKind::MelonSlice => return Some(Food { slot: slot, priority: 0 }),
      ItemKind::SweetBerries => return Some(Food { slot: slot, priority: 0 }),
      ItemKind::GlowBerries => return Some(Food { slot: slot, priority: 0 }),
      ItemKind::Carrot => return Some(Food { slot: slot, priority: 0 }),
      ItemKind::Beetroot => return Some(Food { slot: slot, priority: 0 }),
      ItemKind::DriedKelp => return Some(Food { slot: slot, priority: 0 }),
      ItemKind::Cookie => return Some(Food { slot: slot, priority: 0 }),

      _ => {}
    }

    None
  }
}
