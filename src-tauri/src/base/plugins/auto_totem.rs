use azalea::prelude::*;
use azalea::registry::builtin::ItemKind;
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::common::move_item;


pub struct AutoTotemPlugin;

impl AutoTotemPlugin {
  pub fn enable(bot: Client) {
    tokio::spawn(async move {
      loop {
        if let Some(arc) = get_flow_manager() {
          if !arc.read().active {
            break;
          }
        }

        Self::take_totem(&bot).await;

        sleep(Duration::from_millis(50)).await;
      }
    });
  }

  pub async fn take_totem(bot: &Client) {
    if let Some(item) = bot.menu().slot(45) {
      if !item.is_empty() && item.kind() != ItemKind::Shield {
        return;
      }
    } 

    for (slot, item) in bot.menu().slots().iter().enumerate() {  
      if slot != 45 {
        if item.kind() == ItemKind::TotemOfUndying {
          move_item(bot, ItemKind::TotemOfUndying, slot, 45).await;
        }
      }
    }
  }
}
