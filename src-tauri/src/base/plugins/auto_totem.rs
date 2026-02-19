use azalea::prelude::*;
use azalea::registry::builtin::ItemKind;
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::common::{get_bot_inventory_menu, inventory_move_item};
use crate::methods::SafeClientMethods;

pub struct AutoTotemPlugin;

impl AutoTotemPlugin {
  pub fn new() -> Self {
    Self
  }

  pub fn enable(&'static self, username: String) {
    tokio::spawn(async move {
      loop {
        if let Some(arc) = get_flow_manager() {
          if !arc.read().active {
            break;
          }
        }

        let _ = BOT_REGISTRY
          .get_bot(&username, async |bot| {
            if !bot.workable() {
              return;
            }

            self.take_totem(&bot).await;
          })
          .await;

        sleep(Duration::from_millis(50)).await;
      }
    });
  }

  pub async fn take_totem(&self, bot: &Client) {
    if let Some(menu) = get_bot_inventory_menu(bot) {
      if let Some(item) = menu.slot(45) {
        if !item.is_empty() && item.kind() != ItemKind::Shield {
          return;
        }
      }

      for (slot, item) in menu.slots().iter().enumerate() {
        if slot != 45 {
          if item.kind() == ItemKind::TotemOfUndying {
            inventory_move_item(bot, ItemKind::TotemOfUndying, slot, 45, true).await;
          }
        }
      }
    }
  }
}
