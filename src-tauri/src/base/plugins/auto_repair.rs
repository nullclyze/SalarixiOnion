use azalea::inventory::components::{Damage, Enchantments, MaxDamage};
use azalea::inventory::ItemStack;
use azalea::prelude::*;
use azalea::protocol::packets::game::s_interact::InteractionHand;
use azalea::registry::builtin::ItemKind;
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::common::{get_bot_inventory_menu, move_item_to_offhand, start_use_item, take_item};
use crate::generators::*;
use crate::methods::SafeClientMethods;

#[derive(Clone, Debug)]
struct BrokenItem {
  slot: usize,
  kind: ItemKind,
}

pub struct AutoRepairPlugin;

impl AutoRepairPlugin {
  pub fn new() -> Self {
    Self
  }

  pub fn enable(&'static self, username: String) {
    let nickname = username.clone();

    let task = tokio::spawn(async move {
      loop {
        if !process_is_active() {
          break;
        }

        let _ = BOT_REGISTRY
          .get_bot(&nickname, async |bot| {
            if !bot.workable() {
              return;
            }

            self.repair_items(&bot).await;
          })
          .await;

        sleep(Duration::from_millis(50)).await;
      }
    });

    PLUGIN_MANAGER.push_task(&username, "auto-repair", task);
  }

  async fn repair_items(&self, bot: &Client) {
    let broken_items = self.find_broken_items(bot);

    let nickname = bot.username();

    for broken_item in broken_items {
      if !STATES.get_state(&nickname, "is_eating") && !STATES.get_state(&nickname, "is_drinking") {
        self.repair_item(bot, broken_item).await;
      }
    }
  }

  async fn take_experience_bottles(&self, bot: &Client) -> Option<i32> {
    if let Some(menu) = get_bot_inventory_menu(bot) {
      for (slot, item) in menu.slots().iter().enumerate() {
        if item.kind() == ItemKind::ExperienceBottle {
          take_item(bot, slot, true).await;
          return Some(item.count());
        }
      }
    }

    None
  }

  async fn repair_item(&self, bot: &Client, broken_item: BrokenItem) {
    if let Some(count) = self.take_experience_bottles(bot).await {
      let nickname = bot.username();

      for _ in 0..=count {
        if STATES.get_state(&nickname, "can_interacting")
          && STATES.get_state(&nickname, "can_looking")
        {
          STATES.set_mutual_states(&nickname, "interacting", true);
          STATES.set_mutual_states(&nickname, "looking", true);

          if broken_item.slot != 45 && broken_item.slot > 8 {
            take_item(bot, broken_item.slot, false).await;
            sleep(Duration::from_millis(50)).await;
            move_item_to_offhand(bot, broken_item.kind);
            sleep(Duration::from_millis(randuint(50, 150))).await;
          }

          let mut slot = 45;

          if broken_item.slot >= 5 && broken_item.slot <= 8 {
            slot = broken_item.slot;
          }

          if let Some(menu) = get_bot_inventory_menu(bot) {
            if let Some(item) = menu.slot(slot) {
              let current_damage = self.get_current_item_damage(item);
              let max_durability = self.get_max_item_durability(item);

              if current_damage != 0 && max_durability - current_damage < max_durability / 2 {
                let direction = bot.direction();

                if direction.1 < 84.0 {
                  bot.set_direction(direction.0, randfloat(84.0, 90.0) as f32);
                  sleep(Duration::from_millis(randuint(150, 200))).await;
                }

                start_use_item(bot, InteractionHand::MainHand);

                sleep(Duration::from_millis(randuint(200, 250))).await;
              } else {
                return;
              }
            }
          }

          STATES.set_mutual_states(&nickname, "interacting", false);
          STATES.set_mutual_states(&nickname, "looking", false);
        }

        sleep(Duration::from_millis(50)).await;
      }
    }
  }

  fn get_current_item_damage(&self, item: &ItemStack) -> i32 {
    if let Some(damage) = item.get_component::<Damage>() {
      return damage.amount;
    }

    0
  }

  fn get_max_item_durability(&self, item: &ItemStack) -> i32 {
    if let Some(damage) = item.get_component::<MaxDamage>() {
      return damage.amount;
    }

    0
  }

  fn item_has_mending(&self, bot: &Client, item: &ItemStack) -> bool {
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

  fn find_broken_items(&self, bot: &Client) -> Vec<BrokenItem> {
    let mut broken_items = vec![];

    if let Some(menu) = get_bot_inventory_menu(bot) {
      for (slot, item) in menu.slots().iter().enumerate() {
        if !item.is_empty() {
          let current_damage = self.get_current_item_damage(item);
          let max_durability = self.get_max_item_durability(item);

          if current_damage != 0 && max_durability - current_damage < max_durability / 2 {
            if self.item_has_mending(bot, item) {
              broken_items.push(BrokenItem {
                slot: slot,
                kind: item.kind(),
              });
            }
          }
        }
      }
    }

    broken_items
  }
}
