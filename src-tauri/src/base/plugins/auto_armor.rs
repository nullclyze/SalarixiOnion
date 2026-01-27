use azalea::prelude::*;
use azalea::inventory::ItemStack;
use azalea::prelude::ContainerClientExt;
use azalea::registry::builtin::ItemKind;
use std::time::Duration;
use tokio::time::sleep;

use crate::base::get_flow_manager;
use crate::tools::randticks;
use crate::common::find_empty_slot_in_invenotry;


#[derive(Debug, Clone)]
struct Armor {
  part: String,
  slot: Option<u16>,
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
  pub fn enable(bot: Client) {
    tokio::spawn(async move {
      loop {
        if let Some(arc) = get_flow_manager() {
          if !arc.read().active {
            break;
          }

          Self::equip_armor(&bot).await;
        }

        sleep(Duration::from_millis(50)).await;
      }
    });
  } 

  async fn equip_armor(bot: &Client) {
    let mut armors = vec![];

    for (slot, item) in bot.menu().slots().iter().enumerate() {  
      if slot > 8 {
        if let Some(armor) = Self::is_armor(item, Some(slot as u16)) {
          armors.push(armor);
        }
      }
    }

    let armor_set = Self::get_best_armor(bot, armors);

    if let Some(helmet) = armor_set.helmet {
      if let Some(slot) = helmet.slot {
        if Self::is_this_armor_better(bot, helmet) {
          Self::equip(bot, 5, slot).await;
        }
      }
    }

    if let Some(chestplate) = armor_set.chestplate {
      if let Some(slot) = chestplate.slot {
        if Self::is_this_armor_better(bot, chestplate) {
          Self::equip(bot, 6, slot).await;
        }
      }
    }

    if let Some(leggings) = armor_set.leggings {
      if let Some(slot) = leggings.slot {
        if Self::is_this_armor_better(&bot, leggings) {
          Self::equip(bot,7, slot).await;
        }
      }
    }

    if let Some(boots) = armor_set.boots {
      if let Some(slot) = boots.slot {
        if Self::is_this_armor_better(&bot, boots) {
          Self::equip(bot, 8, slot).await;
        }
      }
    }
  }

  async fn equip(bot: &Client, armor_slot: u16, target_slot: u16) {
    let inventory = bot.get_inventory();

    if let Some(menu) = inventory.menu() {
      if let Some(item) = menu.slot(armor_slot as usize) {
        if !item.is_empty() {
          if let Some(empty_slot) = find_empty_slot_in_invenotry(bot) {
            inventory.left_click(armor_slot);
            bot.wait_ticks(randticks(1, 2)).await;
            inventory.left_click(empty_slot);
          } else {
            return;
          }
        }
      }
    }
    
    inventory.shift_click(target_slot);
  }

  fn is_armor(item: &ItemStack, slot: Option<u16>) -> Option<Armor> {
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

  fn get_best_armor(bot: &Client, armors: Vec<Armor>) -> ArmorSet {
    let mut armor_set = ArmorSet { 
      helmet: None, 
      chestplate: None,
      leggings: None,
      boots: None
    };

    for armor in armors {
      match armor.part.as_str() {
        "helmet" => {
          if let Some(helmet) = armor_set.helmet.clone() {
            if armor.priority > helmet.priority && Self::is_this_armor_better(bot, helmet) {
              armor_set.helmet = Some(armor);
            }
          } else {
            armor_set.helmet = Some(armor);
          }
        },
        "chestplate" => {
          if let Some(chestplate) = armor_set.chestplate.clone() {
            if armor.priority > chestplate.priority && Self::is_this_armor_better(bot, chestplate) {
              armor_set.chestplate = Some(armor);
            }
          } else {
            armor_set.chestplate = Some(armor);
          }
        },
        "leggings" => {
          if let Some(leggings) = armor_set.leggings.clone() {
            if armor.priority > leggings.priority && Self::is_this_armor_better(bot, leggings) {
              armor_set.leggings = Some(armor);
            }
          } else {
            armor_set.leggings = Some(armor);
          }
        },
        "boots" => {
          if let Some(boots) = armor_set.boots.clone() {
            if armor.priority > boots.priority && Self::is_this_armor_better(bot, boots) {
              armor_set.boots = Some(armor);
            }
          } else {
            armor_set.boots = Some(armor);
          }
        },
        _ => {}
      }
    }

    armor_set
  }

  fn is_this_armor_better(bot: &Client, armor: Armor) -> bool {
    let inventory = bot.get_inventory();

    if let Some(menu) = inventory.menu() {
      let target_slot = match armor.part.as_str() {
        "helmet" => 5,
        "chestplate" => 6,
        "leggings" => 7,
        "boots" => 8,
        _ => return false
      };

      if let Some(item) = menu.slot(target_slot) {
        if let Some(current_armor) = Self::is_armor(item, Some(target_slot as u16)) {
          if armor.part == current_armor.part {
            return armor.priority > current_armor.priority;
          }
        } else {
          return true;
        }
      }
    }

    false
  }
}
