use azalea::container::ContainerHandleRef;
use azalea::core::direction::Direction;
use azalea::core::position::BlockPos;
use azalea::entity::inventory::Inventory;
use azalea::inventory::components::ItemName;
use azalea::inventory::operations::ThrowClick;
use azalea::inventory::Menu;
use azalea::prelude::*;
use azalea::protocol::packets::game::s_player_action::Action;
use azalea::protocol::packets::game::ServerboundPlayerAction;
use azalea::registry::builtin::ItemKind;
use std::time::Duration;
use tokio::time::sleep;

use crate::service::core::bot::{get_state, set_mutual_states, set_state};
use crate::service::core::common::convert_inventory_slot_to_hotbar_slot;
use crate::service::core::extensions::{BotDefaultExt, BotMovementExt};
use crate::service::generators::randint;

pub trait BotInventoryExt {
  fn get_selected_slot(&self) -> u8;
  fn get_current_inventory(&self) -> Option<ContainerHandleRef>;
  fn get_inventory_menu(&self) -> Option<Menu>;
  fn find_empty_slot_in_invenotry(&self) -> Option<usize>;
  fn start_interacting_with_inventory(&self);
  fn stop_interacting_with_inventory(&self);
  fn move_item_to_offhand(&self, kind: ItemKind);
  fn inventory_click(&self, slot: usize, click: InvClick, lock: bool);
  fn inventory_click_on(&self, name: &str, click: InvClick, lock: bool);
  async fn take_item(&self, source_slot: usize, lock: bool);
  async fn inventory_swap_click(&self, source_slot: usize, target_slot: usize, lock: bool);
  async fn inventory_move_item(&self, kind: ItemKind, source_slot: usize, target_slot: usize, lock: bool);
}

#[derive(Debug, Clone)]
pub enum InvClickMode {
  Left,
  Right,
  Shift,
  Drop,
  DropAll,
}

#[derive(Debug, Clone)]
pub struct InvClick(InvClickMode);

impl InvClick {
  pub fn from(n: u8) -> Self {
    Self(match n {
      0 => InvClickMode::Left,
      1 => InvClickMode::Right,
      2 => InvClickMode::Shift,
      3 => InvClickMode::Drop,
      4 => InvClickMode::DropAll,

      // Значение по умолчанию
      _ => InvClickMode::Shift,
    })
  }
}

impl BotInventoryExt for Client {
  fn get_current_inventory(&self) -> Option<ContainerHandleRef> {
    if let Some(inventory) = self.get_component::<Inventory>() {
      return Some(ContainerHandleRef::new(inventory.id, self.clone()));
    }

    None
  }

  fn get_selected_slot(&self) -> u8 {
    if let Some(inventory) = self.get_component::<Inventory>() {
      return inventory.selected_hotbar_slot;
    }

    0
  }

  fn get_inventory_menu(&self) -> Option<Menu> {
    if let Some(inventory) = self.get_component::<Inventory>() {
      return Some(inventory.menu().clone());
    }

    None
  }

  fn find_empty_slot_in_invenotry(&self) -> Option<usize> {
    let Some(menu) = self.get_inventory_menu() else {
      return None;
    };

    for (slot, item) in menu.slots().iter().enumerate() {
      if slot > 9 {
        if item.is_empty() {
          return Some(slot);
        }
      }
    }

    None
  }

  fn start_interacting_with_inventory(&self) {
    let username = self.name();

    self.freeze_move();

    set_state(&username, "can_eating", false);
    set_state(&username, "can_drinking", false);
    set_state(&username, "can_attacking", false);

    set_mutual_states(&username, "interacting", true);
  }

  fn stop_interacting_with_inventory(&self) {
    let username = self.name();

    self.unfreeze_move();

    set_state(&username, "can_eating", true);
    set_state(&username, "can_drinking", true);
    set_state(&username, "can_attacking", true);

    set_mutual_states(&username, "interacting", false);
  }

  async fn take_item(&self, source_slot: usize, lock: bool) {
    if let Some(hotbar_slot) = convert_inventory_slot_to_hotbar_slot(source_slot) {
      if self.get_selected_slot() != hotbar_slot {
        self.set_selected_hotbar_slot(hotbar_slot);
      }
    } else {
      if let Some(empty_slot) = self.find_empty_slot_in_invenotry() {
        self.inventory_swap_click(source_slot, empty_slot as usize, lock).await;

        if let Some(slot) = convert_inventory_slot_to_hotbar_slot(empty_slot as usize) {
          if self.get_selected_slot() != slot {
            sleep(Duration::from_millis(50)).await;
            self.set_selected_hotbar_slot(slot);
          }
        }
      } else {
        let random_slot = randint(36, 44) as usize;

        self.inventory_click(random_slot, InvClick::from(2), lock);
        sleep(Duration::from_millis(50)).await;
        self.inventory_swap_click(source_slot, random_slot, lock).await;

        sleep(Duration::from_millis(50)).await;

        let hotbar_slot = convert_inventory_slot_to_hotbar_slot(random_slot).unwrap_or(0);

        if self.get_selected_slot() != hotbar_slot {
          self.set_selected_hotbar_slot(hotbar_slot);
        }
      }
    }
  }

  fn move_item_to_offhand(&self, kind: ItemKind) {
    if let Some(menu) = self.get_inventory_menu() {
      if let Some(item) = menu.slot(45) {
        if item.kind() == kind {
          return;
        }
      }
    }

    self.write_packet(ServerboundPlayerAction {
      action: Action::SwapItemWithOffhand,
      pos: BlockPos::new(0, 0, 0),
      direction: Direction::Down,
      seq: 0,
    });
  }

  async fn inventory_swap_click(&self, source_slot: usize, target_slot: usize, lock: bool) {
    let username = self.name();

    if let Some(inventory) = self.get_current_inventory() {
      if lock {
        if !get_state(&username, "can_interacting") {
          return;
        }

        self.start_interacting_with_inventory();
      }

      if let Some(menu) = self.get_inventory_menu() {
        if let Some(item) = menu.slot(target_slot) {
          if !item.is_empty() {
            if let Some(empty_slot) = self.find_empty_slot_in_invenotry() {
              inventory.left_click(target_slot);
              sleep(Duration::from_millis(100)).await;
              inventory.left_click(empty_slot);
            } else {
              self.inventory_click(target_slot, InvClick::from(4), false);
            }

            sleep(Duration::from_millis(100)).await;
          }
        }
      }

      inventory.left_click(source_slot);
      sleep(Duration::from_millis(100)).await;
      inventory.left_click(target_slot);

      sleep(Duration::from_millis(50)).await;

      if lock {
        self.stop_interacting_with_inventory();
      }
    }
  }

  fn inventory_click(&self, slot: usize, click: InvClick, lock: bool) {
    let username = self.name();

    if let Some(inventory) = self.get_current_inventory() {
      if lock {
        if !get_state(&username, "can_interacting") {
          return;
        }

        self.start_interacting_with_inventory();
      }

      match click.0 {
        InvClickMode::Left => {
          inventory.left_click(slot);
        }
        InvClickMode::Right => {
          inventory.right_click(slot);
        }
        InvClickMode::Shift => {
          inventory.shift_click(slot);
        }
        InvClickMode::Drop => {
          inventory.click(ThrowClick::Single { slot: slot as u16 });
        }
        InvClickMode::DropAll => {
          inventory.click(ThrowClick::All { slot: slot as u16 });
        }
      }

      if lock {
        self.stop_interacting_with_inventory();
      }
    }
  }

  fn inventory_click_on(&self, name: &str, click: InvClick, lock: bool) {
    let string_name = name.to_string();

    if let Some(menu) = self.get_inventory_menu() {
      for (slot, item) in menu.slots().iter().enumerate() {
        let Some(item_name) = item.get_component::<ItemName>() else {
          continue;
        };

        if item_name.name.to_string() == string_name {
          self.inventory_click(slot, click, lock);
          break;
        }
      }
    }
  }

  async fn inventory_move_item(&self, kind: ItemKind, source_slot: usize, target_slot: usize, lock: bool) {
    if let Some(menu) = self.get_inventory_menu() {
      if let Some(item) = menu.slot(target_slot) {
        if item.kind() == kind {
          return;
        }
      }
    }

    self.inventory_swap_click(source_slot, target_slot, lock).await;
  }
}
