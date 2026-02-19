use azalea::core::direction::Direction;
use azalea::core::position::BlockPos;
use azalea::inventory::operations::ThrowClick;
use azalea::inventory::Menu;
use azalea::prelude::*;
use azalea::protocol::packets::game::s_player_action::Action;
use azalea::protocol::packets::game::ServerboundPlayerAction;
use azalea::registry::builtin::ItemKind;
use azalea::WalkDirection;
use std::time::Duration;
use tokio::time::sleep;

use super::auxiliary::*;
use crate::base::*;
use crate::generators::randuint;

/// Функция конвертации индекса inventory-слота в индекс hotbar-слота
pub fn convert_inventory_slot_to_hotbar_slot(slot: usize) -> Option<u8> {
  match slot {
    36 => Some(0),
    37 => Some(1),
    38 => Some(2),
    39 => Some(3),
    40 => Some(4),
    41 => Some(5),
    42 => Some(6),
    43 => Some(7),
    44 => Some(8),
    _ => None,
  }
}

/// Функция конвертации индекса hotbar-слота в индекс inventory-слота
pub fn convert_hotbar_slot_to_inventory_slot(slot: u8) -> usize {
  match slot {
    0 => 36,
    1 => 37,
    2 => 38,
    3 => 39,
    4 => 40,
    5 => 41,
    6 => 42,
    7 => 43,
    8 => 44,
    _ => 36,
  }
}

/// Функция нахождения пустого слота в инвентаре
pub fn find_empty_slot_in_invenotry(menu: Menu) -> Option<usize> {
  for (slot, item) in menu.slots().iter().enumerate() {
    if slot > 9 {
      if item.is_empty() {
        return Some(slot);
      }
    }
  }

  None
}

/// Функция нахождения пустого слота в хотбаре
pub fn find_empty_slot_in_hotbar(menu: Menu) -> Option<u8> {
  for slot in menu.hotbar_slots_range() {
    if let Some(item) = menu.slot(slot) {
      if item.is_empty() {
        return Some(slot as u8);
      }
    }
  }

  None
}

/// Вспомогательная функция переключения состояний
pub fn start_interacting_with_inventory(bot: &Client) {
  let username = bot.username();

  bot.walk(WalkDirection::None);

  STATES.set_state(&username, "can_walking", false);
  STATES.set_state(&username, "can_sprinting", false);
  STATES.set_state(&username, "can_attacking", false);

  STATES.set_mutual_states(&username, "interacting", true);
}

/// Вспомогательная функция переключения состояний
pub fn stop_interacting_with_inventory(username: &String) {
  STATES.set_state(username, "can_walking", true);
  STATES.set_state(username, "can_sprinting", true);
  STATES.set_state(username, "can_attacking", true);
  
  STATES.set_mutual_states(username, "interacting", false);
}

/// Функция, позволяющая боту безопасно переместить предмет в hotbar и взять его
pub async fn take_item(bot: &Client, source_slot: usize, lock: bool) {
  if let Some(hotbar_slot) = convert_inventory_slot_to_hotbar_slot(source_slot) {
    if get_bot_selected_hotbar_slot(bot) != hotbar_slot {
      bot.set_selected_hotbar_slot(hotbar_slot);
    }
  } else {
    if let Some(menu) = get_bot_inventory_menu(bot) {
      if let Some(empty_slot) = find_empty_slot_in_hotbar(menu) {
        inventory_swap_click(bot, source_slot, empty_slot as usize, lock).await;

        if let Some(slot) = convert_inventory_slot_to_hotbar_slot(empty_slot as usize) {
          if get_bot_selected_hotbar_slot(bot) != slot {
            sleep(Duration::from_millis(50)).await;
            bot.set_selected_hotbar_slot(slot);
          }
        }
      } else {
        let random_slot = randuint(36, 44) as usize;

        inventory_shift_click(bot, random_slot, lock);
        sleep(Duration::from_millis(50)).await;
        inventory_swap_click(bot, source_slot, random_slot, lock).await;

        sleep(Duration::from_millis(50)).await;

        let hotbar_slot = convert_inventory_slot_to_hotbar_slot(random_slot).unwrap_or(0);

        if get_bot_selected_hotbar_slot(bot) != hotbar_slot {
          bot.set_selected_hotbar_slot(hotbar_slot);
        }
      }
    }
  }
}

/// Функция безопасного перемещения предмета в offhand
pub fn move_item_to_offhand(bot: &Client, kind: ItemKind) {
  if let Some(menu) = get_bot_inventory_menu(bot) {
    if let Some(item) = menu.slot(45) {
      if item.kind() == kind {
        return;
      }
    }
  }

  bot.write_packet(ServerboundPlayerAction {
    action: Action::SwapItemWithOffhand,
    pos: BlockPos::new(0, 0, 0),
    direction: Direction::Down,
    seq: 0,
  });
}

/// Функция безопасного shift-клика в инвентаре
pub fn inventory_shift_click(bot: &Client, slot: usize, lock: bool) {
  let username = bot.username();

  if let Some(inventory) = get_bot_inventory(bot) {
    if lock {
      start_interacting_with_inventory(bot);
    }

    inventory.shift_click(slot);

    if lock {
      stop_interacting_with_inventory(&username);
    }
  }
}

/// Функция безопасного left-клика в инвентаре
pub fn inventory_left_click(bot: &Client, slot: usize, lock: bool) {
  let username = bot.username();

  if let Some(inventory) = get_bot_inventory(bot) {
    if lock {
      start_interacting_with_inventory(bot);
    }

    inventory.left_click(slot);

    if lock {
      stop_interacting_with_inventory(&username);
    }
  }
}

/// Функция безопасного right-клика в инвентаре
pub fn inventory_right_click(bot: &Client, slot: usize, lock: bool) {
  let username = bot.username();

  if let Some(inventory) = get_bot_inventory(bot) {
    if lock {
      start_interacting_with_inventory(bot);
    }

    inventory.shift_click(slot);

    if lock {
      stop_interacting_with_inventory(&username);
    }
  }
}

/// Функция безопасного swap-клика в инвентаре
pub async fn inventory_swap_click(
  bot: &Client,
  source_slot: usize,
  target_slot: usize,
  lock: bool,
) {
  let username = bot.username();

  if let Some(inventory) = get_bot_inventory(bot) {
    if lock {
      start_interacting_with_inventory(bot);
    }

    if let Some(menu) = get_bot_inventory_menu(bot) {
      if let Some(item) = menu.slot(target_slot) {
        if !item.is_empty() {
          if let Some(empty_slot) = find_empty_slot_in_invenotry(menu) {
            inventory.left_click(target_slot);
            sleep(Duration::from_millis(50)).await;
            inventory.left_click(empty_slot);
          } else {
            inventory_drop_item(bot, target_slot, false);
          }

          sleep(Duration::from_millis(50)).await;
        }
      }
    }

    inventory.left_click(source_slot);
    sleep(Duration::from_millis(50)).await;
    inventory.left_click(target_slot);

    if lock {
      stop_interacting_with_inventory(&username);
    }
  }
}

/// Функция безопасного выбрасывания предмета
pub fn inventory_drop_item(bot: &Client, slot: usize, lock: bool) {
  let username = bot.username();

  if let Some(inventory) = get_bot_inventory(bot) {
    if lock {
      start_interacting_with_inventory(bot);
    }

    if let Some(menu) = get_bot_inventory_menu(bot) {
      if let Some(item) = menu.slot(slot) {
        if !item.is_empty() {
          inventory.click(ThrowClick::All { slot: slot as u16 });
        }
      }
    }

    if lock {
      stop_interacting_with_inventory(&username);
    }
  }
}

/// Функция безопасного перемещения предмета
pub async fn inventory_move_item(
  bot: &Client,
  kind: ItemKind,
  source_slot: usize,
  target_slot: usize,
  lock: bool,
) {
  if let Some(menu) = get_bot_inventory_menu(bot) {
    if let Some(item) = menu.slot(target_slot) {
      if item.kind() == kind {
        return;
      }
    }
  }

  inventory_swap_click(bot, source_slot, target_slot, lock).await;
}
