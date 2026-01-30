use azalea::entity::metadata::Health;
use azalea::inventory::ItemStack;
use azalea::local_player::Hunger;
use azalea::prelude::*;
use azalea::BlockPos;
use azalea::protocol::packets::game::{ServerboundPlayerAction, ServerboundUseItem};
use azalea::protocol::packets::game::s_interact::InteractionHand;
use azalea::registry::builtin::ItemKind;
use azalea::protocol::packets::game::s_player_action::Action;
use std::time::Duration;
use tokio::time::sleep;

use crate::base::get_flow_manager;
use crate::state::STATES;
use crate::common::move_item_to_hotbar;
use crate::tasks::TASKS;
use crate::tools::randchance;
use crate::tools::randuint;


#[derive(Clone)]
struct Food {
  slot: Option<usize>,
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
        }

        Self::eat(&bot).await;

        sleep(Duration::from_millis(50)).await;
      }
    });
  } 

  async fn eat(bot: &Client) {
    let satiety = if let Some(hunger) = bot.get_component::<Hunger>() {
      hunger.food
    } else {
      20
    };

    let nickname = bot.username();

    if satiety < 20 {
      let food_list = Self::find_food_in_inventory(bot);

      if let Some(best_food) = Self::get_best_food(bot, food_list.clone()) {
        if let Some(food_slot) = best_food.slot {
          if !STATES.get_plugin_activity(&nickname, "auto-potion") {
            let mut should_eat = true;

            if TASKS.get_task_activity(&nickname, "killaura") {
              should_eat = randchance(0.5);
            }

            if should_eat {
              STATES.set_plugin_activity(&nickname, "auto-eat", true);

              move_item_to_hotbar(bot, food_slot).await;
              sleep(Duration::from_millis(randuint(50, 100))).await;
              Self::start_eating(bot).await;

              STATES.set_plugin_activity(&nickname, "auto-eat", false);
            }
          }
        }
      }
    }
  }

  async fn start_eating(bot: &Client) {
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

  fn get_best_food(bot: &Client, food_list: Vec<Food>) -> Option<Food> {
    if let Some(health_component) = bot.get_component::<Health>() {
      if let Some(hunger_component) = bot.get_component::<Hunger>() {
        let health = health_component.0;
        let hunger = hunger_component.food;

        let mut best_food = None;

        if food_list.len() > 1 {
          let desired_health_priority;
          let desired_hunger_priority;

          if health < 20.0 && health >= 18.0 {
            desired_health_priority = 0;
          } else if health < 18.0 && health >= 14.0 {
            desired_health_priority = 1;
          } else if health < 14.0 && health >= 10.0 {
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

        return best_food;
      }
    }

    None
  }

  fn find_food_in_inventory(bot: &Client) -> Vec<Food> {
    let mut food_list = vec![];

    for (slot, item) in bot.menu().slots().iter().enumerate() {
      if let Some(food) = Self::is_food(Some(slot), item) {
        food_list.push(food);
      }
    }

    food_list
  }

  fn is_food(slot: Option<usize>, item: &ItemStack) -> Option<Food> {
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
