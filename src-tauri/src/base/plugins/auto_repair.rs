use azalea::inventory::components::{Damage, Enchantments, MaxDamage};
use azalea::protocol::packets::game::s_interact::InteractionHand;
use azalea::prelude::*;
use azalea::inventory::ItemStack;
use azalea::registry::builtin::ItemKind;
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::tools::*;
use crate::common::{move_item_to_offhand, start_use_item, take_item};


#[derive(Clone, Debug)]
struct BrokenItem {
  slot: usize,
  kind: ItemKind
}

pub struct AutoRepairPlugin;

impl AutoRepairPlugin {
  pub fn enable(bot: Client) {
    tokio::spawn(async move {
      loop {
        if let Some(arc) = get_flow_manager() {
          if !arc.read().active {
            break;
          }
        }

        Self::repair_items(&bot).await;

        sleep(Duration::from_millis(50)).await;
      }
    });
  } 

  async fn repair_items(bot: &Client) {
    let broken_items = Self::find_broken_items(bot);

    let nickname = bot.username();

    for broken_item in broken_items {
      if !STATES.get_state(&nickname, "is_eating") && !STATES.get_state(&nickname, "is_drinking") {
        if broken_item.slot != 45 && broken_item.slot > 8 {
          take_item(bot, broken_item.slot).await;
          sleep(Duration::from_millis(50)).await;
          move_item_to_offhand(bot, broken_item.kind);
          sleep(Duration::from_millis(randuint(50, 150))).await;
        } 

        Self::repair_item(bot, broken_item).await;
      }
    }
  }

  async fn take_experience_bottles(bot: &Client) -> Option<i32> {
    for (slot, item) in bot.menu().slots().iter().enumerate() {  
      if item.kind() == ItemKind::ExperienceBottle {
        take_item(bot, slot).await;
        return Some(item.count());
      }
    }

    None
  }

  async fn repair_item(bot: &Client, broken_item: BrokenItem) {
    if let Some(count) = Self::take_experience_bottles(bot).await {
      let nickname = bot.username();

      for _ in 0..=count {
        if !STATES.get_state(&nickname, "is_attacking") {
          let mut slot = 45;

          if broken_item.slot >= 5 && broken_item.slot <= 8 {
            slot = broken_item.slot;
          } 

          if let Some(item) = bot.menu().slot(slot) {
            let current_damage = Self::get_current_item_damage(item);
            let max_durability = Self::get_max_item_durability(item);

            if current_damage != 0 && max_durability - current_damage < max_durability / 2 {
              let direction = bot.direction();

              if direction.1 < 84.0 {
                if STATES.get_state(&nickname, "can_looking") {
                  STATES.set_state(&nickname, "can_looking", false);
                  STATES.set_state(&nickname, "is_looking", true);

                  bot.set_direction(direction.0, randfloat(84.0, 90.0) as f32);
                  sleep(Duration::from_millis(randuint(200, 250))).await;
                  
                  STATES.set_state(&nickname, "can_looking", true);
                  STATES.set_state(&nickname, "is_looking", false);
                } else {
                  continue;
                }
              } 

              start_use_item(bot, InteractionHand::MainHand);

              sleep(Duration::from_millis(randuint(200, 300))).await;
            } else {
              return;
            }
          }
        }
      }
    }
  }

  fn get_current_item_damage(item: &ItemStack) -> i32 {
    if let Some(damage) = item.get_component::<Damage>() {
      return damage.amount;
    }

    0
  }

  fn get_max_item_durability(item: &ItemStack) -> i32 {
    if let Some(damage) = item.get_component::<MaxDamage>() {
      return damage.amount;
    }

    0
  }

  fn item_has_mending(bot: &Client, item: &ItemStack) -> bool {
    if let Some(enchantments) = item.get_component::<Enchantments>() {
      for (enchantment, _) in &enchantments.levels {
        if let Some(id) = bot.resolve_registry_name(enchantment) {
          if id.to_string().contains("minecraft:mending") {
            return true;
          }
        }
      }
    }

    false
  }

  fn find_broken_items(bot: &Client) -> Vec<BrokenItem> {
    let mut broken_items = vec![];

    for (slot, item) in bot.menu().slots().iter().enumerate() {  
      if !item.is_empty() {
        let current_damage = Self::get_current_item_damage(item);
        let max_durability = Self::get_max_item_durability(item);

        if current_damage != 0 && max_durability - current_damage < max_durability / 2 {
          if Self::item_has_mending(bot, item) {
            broken_items.push(BrokenItem { 
              slot: slot, 
              kind: item.kind()
            });
          }
        }
      }
    }

    broken_items
  }
}
