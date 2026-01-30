use azalea::{WalkDirection, prelude::*};
use azalea::prelude::ContainerClientExt;
use azalea::registry::builtin::ItemKind;
use std::time::Duration;
use tokio::time::sleep;

use crate::base::get_flow_manager;
use crate::state::STATES;


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
    let nickname = bot.username();

    if let Some(item) = bot.menu().slot(45) {
      if item.is_empty() || item.kind() == ItemKind::Shield {
        if !STATES.get_plugin_activity(&nickname, "auto-shield") {
          for (slot, item) in bot.menu().slots().iter().enumerate() {  
            if slot != 45 {
              if item.kind() == ItemKind::TotemOfUndying {
                STATES.set_plugin_activity(&nickname, "auto-totem", true);
                
                let inventory = bot.get_inventory();

                bot.walk(WalkDirection::None);

                inventory.left_click(slot);
                bot.wait_ticks(1).await;
                inventory.left_click(45 as usize);

                STATES.set_plugin_activity(&nickname, "auto-totem", false);
              }
            }
          }
        }
      }
    } 
  }
}
