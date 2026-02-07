use azalea::prelude::*;
use azalea::inventory::ItemStack;
use azalea::registry::builtin::ItemKind;
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::common::{close_inventory, find_empty_slot_in_invenotry, get_inventory, get_inventory_menu};


#[derive(Debug, Clone)]
struct Armor {
  part: String,
  slot: usize,
  priority: u8
}

#[derive(Debug, Clone)]
struct ArmorSet {
  helmet: Option<Armor>,
  chestplate: Option<Armor>,
  leggings: Option<Armor>,
  boots: Option<Armor>
}

pub struct AutoArmorPlugin;

impl AutoArmorPlugin {
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

        self.equip_armor(&bot).await;

        sleep(Duration::from_millis(50)).await;
      }
    });
  } 

  async fn equip_armor(&self, bot: &Client) {
    let mut armors = vec![];

      for slot in 0..=48 {  
        if let Some(item) = bot.menu().slot(slot) {
          if slot > 8 {
            if let Some(armor) = self.is_armor(item, slot) {
              armors.push(armor);
            }
          }
        }
      }

    let armor_set = self.get_best_armor(bot, armors);

    if let Some(helmet) = armor_set.helmet {
      if self.is_this_armor_better(bot, &helmet) {
        self.equip(bot, helmet.slot, 5).await;
      }
    }

    if let Some(chestplate) = armor_set.chestplate {
      if self.is_this_armor_better(bot, &chestplate) {
        self.equip(bot, chestplate.slot, 6).await;
      }
    }

    if let Some(leggings) = armor_set.leggings {
      if self.is_this_armor_better(&bot, &leggings) {
        self.equip(bot, leggings.slot, 7).await;
      }
    }

    if let Some(boots) = armor_set.boots {
      if self.is_this_armor_better(&bot, &boots) {
        self.equip(bot, boots.slot, 8).await;
      }
    }
  }

  async fn equip(&self, bot: &Client, armor_slot: usize, target_slot: usize) {
    if let Some(inventory) = get_inventory(bot) {
      let nickname = bot.username();

      if let Some(menu) = get_inventory_menu(bot) {
        if let Some(item) = menu.slot(target_slot) {
          if !item.is_empty() {
            if let Some(_) = find_empty_slot_in_invenotry(bot) {
              inventory.shift_click(target_slot);
              sleep(Duration::from_millis(50)).await;
            } else {
              return;
            }
          }
        }
      }
      
      inventory.shift_click(armor_slot);

      sleep(Duration::from_millis(50)).await;

      close_inventory(bot);

      STATES.set_state(&nickname, "can_walking", true);
      STATES.set_state(&nickname, "can_sprinting", true);
    }
  }

  fn is_armor(&self, item: &ItemStack, slot: usize) -> Option<Armor> {
    let mut armor = None;

    let helmet = "helmet".to_string();
    let chestplate = "chestplate".to_string();
    let leggings = "leggings".to_string();
    let boots = "boots".to_string();

    if !item.is_empty() {
      match item.kind() {
        // Шлема
        ItemKind::TurtleHelmet => { armor = Some(Armor { part: helmet, slot: slot, priority: 0 }) },
        ItemKind::LeatherHelmet => { armor = Some(Armor { part: helmet, slot: slot, priority: 1 }) },
        ItemKind::GoldenHelmet => { armor = Some(Armor { part: helmet, slot: slot, priority: 2 }) },
        ItemKind::ChainmailHelmet => { armor = Some(Armor { part: helmet, slot: slot, priority: 3 }) },
        ItemKind::CopperHelmet => { armor = Some(Armor { part: helmet, slot: slot, priority: 4 }) },
        ItemKind::IronHelmet => { armor = Some(Armor { part: helmet, slot: slot, priority: 5 }) },
        ItemKind::DiamondHelmet => { armor = Some(Armor { part: helmet, slot: slot, priority: 6 }) },
        ItemKind::NetheriteHelmet => { armor = Some(Armor { part: helmet, slot: slot, priority: 7 }) },

        // Нагрудники
        ItemKind::LeatherChestplate => { armor = Some(Armor { part: chestplate, slot: slot, priority: 0 }) },
        ItemKind::GoldenChestplate => { armor = Some(Armor { part: chestplate, slot: slot, priority: 1 }) },
        ItemKind::ChainmailChestplate => { armor = Some(Armor { part: chestplate, slot: slot, priority: 2 }) },
        ItemKind::CopperChestplate => { armor = Some(Armor { part: chestplate, slot: slot, priority: 3 }) },
        ItemKind::IronChestplate => { armor = Some(Armor { part: chestplate, slot: slot, priority: 4 }) },
        ItemKind::DiamondChestplate => { armor = Some(Armor { part: chestplate, slot: slot, priority: 5 }) },
        ItemKind::NetheriteChestplate => { armor = Some(Armor { part: chestplate, slot: slot, priority: 6 }) },

        // Поножи
        ItemKind::LeatherLeggings => { armor = Some(Armor { part: leggings, slot: slot, priority: 0 }) },
        ItemKind::GoldenLeggings => { armor = Some(Armor { part: leggings, slot: slot, priority: 1 }) },
        ItemKind::ChainmailLeggings => { armor = Some(Armor { part: leggings, slot: slot, priority: 2 }) },
        ItemKind::CopperLeggings => { armor = Some(Armor { part: leggings, slot: slot, priority: 3 }) },
        ItemKind::IronLeggings => { armor = Some(Armor { part: leggings, slot: slot, priority: 4 }) },
        ItemKind::DiamondLeggings => { armor = Some(Armor { part: leggings, slot: slot, priority: 5 }) },
        ItemKind::NetheriteLeggings => { armor = Some(Armor { part: leggings, slot: slot, priority: 6 }) },

        // Ботинки
        ItemKind::LeatherBoots => { armor = Some(Armor { part: boots, slot: slot, priority: 0 }) },
        ItemKind::GoldenBoots => { armor = Some(Armor { part: boots, slot: slot, priority: 1 }) },
        ItemKind::ChainmailBoots => { armor = Some(Armor { part: boots, slot: slot, priority: 2 }) },
        ItemKind::CopperBoots => { armor = Some(Armor { part: boots, slot: slot, priority: 3 }) },
        ItemKind::IronBoots => { armor = Some(Armor { part: boots, slot: slot, priority: 4 }) },
        ItemKind::DiamondBoots => { armor = Some(Armor { part: boots, slot: slot, priority: 5 }) },
        ItemKind::NetheriteBoots => { armor = Some(Armor { part: boots, slot: slot, priority: 6 }) },

        _ => {}
      }
    }

    armor
  }

  fn get_best_armor(&self, bot: &Client, armors: Vec<Armor>) -> ArmorSet {
    let mut armor_set = ArmorSet { 
      helmet: None, 
      chestplate: None,
      leggings: None,
      boots: None
    };

    for armor in armors {
      match armor.part.as_str() {
        "helmet" => {
          if let Some(helmet) = &armor_set.helmet {
            if armor.priority <= helmet.priority {
              continue;
            }
          }
          
          if self.is_this_armor_better(bot, &armor) {
            armor_set.helmet = Some(armor);
          }
        },
        "chestplate" => {
          if let Some(chestplate) = &armor_set.chestplate {
            if armor.priority <= chestplate.priority {
              continue;
            }
          }
          
          if self.is_this_armor_better(bot, &armor) {
            armor_set.chestplate = Some(armor);
          }
        },
        "leggings" => {
          if let Some(leggings) = &armor_set.leggings {
            if armor.priority <= leggings.priority {
              continue;
            }
          }
          
          if self.is_this_armor_better(bot, &armor) {
            armor_set.leggings = Some(armor);
          }
        },
        "boots" => {
          if let Some(boots) = &armor_set.boots {
            if armor.priority <= boots.priority {
              continue;
            }
          }
          
          if self.is_this_armor_better(bot, &armor) {
            armor_set.boots = Some(armor);
          }
        },
        _ => {}
      }
    }

    armor_set
  }

  fn is_this_armor_better(&self, bot: &Client, armor: &Armor) -> bool {
    let target_slot = match armor.part.as_str() {
      "helmet" => 5,
      "chestplate" => 6,
      "leggings" => 7,
      "boots" => 8,
      _ => return false
    };

    if let Some(item) = bot.menu().slot(target_slot) {
      if let Some(current_armor) = self.is_armor(item, target_slot) {
        if armor.part == current_armor.part {
          return armor.priority > current_armor.priority;
        }
      } else {
        return true;
      }
    }

    false
  }
}
