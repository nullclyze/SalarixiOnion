use azalea::prelude::*;
use azalea::registry::builtin::ItemKind;
use std::io;
use std::time::Duration;
use tokio::time::sleep;

use crate::core::*;
use crate::extensions::{BotDefaultExt, BotInventoryExt};

pub struct AutoTotemPlugin;

impl AutoTotemPlugin {
  async fn take_totem(&self, bot: &Client) {
    if let Some(menu) = bot.get_inventory_menu() {
      if let Some(item) = menu.slot(45) {
        if !item.is_empty() && item.kind() != ItemKind::Shield {
          return;
        }
      }

      for (slot, item) in menu.slots().iter().enumerate() {
        if slot != 45 {
          if item.kind() == ItemKind::TotemOfUndying {
            bot
              .inventory_move_item(ItemKind::TotemOfUndying, slot, 45, true)
              .await;
          }
        }
      }
    }
  }
}

impl SalarixiPlugin for AutoTotemPlugin {
  fn new() -> Self {
    Self
  }

  fn activate(&'static self, username: String) -> io::Result<()> {
    let nickname = username.clone();

    let task = tokio::spawn(async move {
      loop {
        if !process_is_active() {
          break;
        }

        let _ = BOT_REGISTRY
          .async_get_bot(&nickname, async |bot| {
            if !bot.workable() {
              return;
            }

            self.take_totem(bot).await;
          })
          .await;

        sleep(Duration::from_millis(50)).await;
      }
    });

    PLUGIN_MANAGER.push_task(&username, "auto-totem", task);

    Ok(())
  }
}
