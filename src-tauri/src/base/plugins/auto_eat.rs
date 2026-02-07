use azalea::prelude::*;
use azalea::entity::metadata::Health;
use azalea::inventory::ItemStack;
use azalea::local_player::Hunger;
use azalea::protocol::packets::game::s_interact::InteractionHand;
use azalea::registry::builtin::ItemKind;
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::common::get_inventory_menu;
use crate::tools::*;
use crate::common::{take_item, start_use_item};


#[derive(Clone)]
struct Food {
  slot: Option<usize>,
  priority: u8
}

pub struct AutoEatPlugin;

impl AutoEatPlugin {
  pub fn new() -> Self {
    Self
  }

  pub fn enable(&'static self, bot: Client) {
    tokio::spawn(async move {
      loop {
        if let Some(arc) = get_flow_manager() {
          if !arc.read().active {
            break;
          }
        }

        self.eat(&bot).await;

        sleep(Duration::from_millis(50)).await;
      }
    });
  } 

  async fn eat(&self, bot: &Client) {
    let satiety = if let Some(hunger) = bot.get_component::<Hunger>() {
      hunger.food
    } else {
      20
    };

    let health = if let Some(health) = bot.get_component::<Health>() {
      health.0
    } else {
      20.0
    };
    
    let nickname = bot.username();

    if satiety < 20 || health < 15.0 {
      let food_list = self.find_food_in_inventory(bot);

      if let Some(best_food) = self.get_best_food(bot, food_list.clone()) {
        if let Some(food_slot) = best_food.slot {
          if STATES.get_state(&nickname, "can_eating") {
            let mut should_eat = true;

            if STATES.get_state(&nickname, "is_attacking") || STATES.get_state(&nickname, "is_interacting") {
              should_eat = !(randchance(satiety as f64 / 20.0) && randchance(health as f64 / 20.0));
            }

            if should_eat {
              STATES.set_state(&nickname, "can_drinking", false);
              STATES.set_mutual_states(&nickname, "eating", true);

              take_item(bot, food_slot).await;
              sleep(Duration::from_millis(50)).await;
              self.start_eating(bot).await;
              sleep(Duration::from_millis(50)).await;

              STATES.set_state(&nickname, "can_drinking", true);
              STATES.set_state(&nickname, "can_walking", true);
              STATES.set_state(&nickname, "can_sprinting", true);
              STATES.set_mutual_states(&nickname, "eating", false);
            } else {
              sleep(Duration::from_millis(1800)).await;
            }
          }
        }
      }
    }
  }

  async fn start_eating(&self, bot: &Client) {
    start_use_item(bot, InteractionHand::MainHand);
    sleep(Duration::from_millis(1800)).await;
  }

  fn get_best_food(&self, bot: &Client, food_list: Vec<Food>) -> Option<Food> {
    if let Some(health_component) = bot.get_component::<Health>() {
      if let Some(hunger_component) = bot.get_component::<Hunger>() {
        let health = health_component.0;
        let hunger = hunger_component.food;

        let mut best_food = None;

        if food_list.len() > 1 {
          if hunger == 20 && health < 12.0 {
            for food in &food_list {
              if food.priority == 3 {
                best_food = Some(food.clone());
                break;
              }
            }
          }

          if best_food.is_none() {
            let desired_health_priority;
            let desired_hunger_priority;

            if health < 20.0 && health >= 18.0 {
              desired_health_priority = 0;
            } else if health < 18.0 && health >= 15.0 {
              desired_health_priority = 1;
            } else if health < 15.0 && health >= 12.0 {
              desired_health_priority = 2;
            } else {
              desired_health_priority = 3;
            }

            if hunger < 20 && hunger >= 18 {
              desired_hunger_priority = 0;
            } else if hunger < 18 && hunger >= 15 {
              desired_hunger_priority = 1;
            } else if hunger < 15 && hunger >= 12 {
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
              for food in &food_list {
                if food.priority == current_priority {
                  best_food = Some(food.clone());
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
          }
        } else {
          best_food = food_list.get(0).cloned();
        }

        return best_food;
      }
    }

    None
  }

  fn find_food_in_inventory(&self, bot: &Client) -> Vec<Food> {
    let mut food_list = vec![];

    if let Some(menu) = get_inventory_menu(bot) {
      for (slot, item) in menu.slots().iter().enumerate() {
        if let Some(food) = self.is_food(Some(slot), item) {
          food_list.push(food);
        }
      }
    }

    food_list
  }

  fn is_food(&self, slot: Option<usize>, item: &ItemStack) -> Option<Food> {
    match item.kind() {
      ItemKind::GoldenApple => return Some(Food { slot: slot, priority: 3 }),
      ItemKind::EnchantedGoldenApple => return Some(Food { slot: slot, priority: 3 }),

      ItemKind::GoldenCarrot => return Some(Food { slot: slot, priority: 2 }),
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
