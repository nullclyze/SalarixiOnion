use azalea::prelude::*;
use azalea::prelude::ContainerClientExt;
use azalea::registry::builtin::ItemKind;
use std::time::Duration;
use tokio::time::sleep;

use crate::base::get_flow_manager;
use crate::state::STATES;
use crate::tools::randticks;
use crate::common::is_this_slot_empty;


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
    if is_this_slot_empty(bot, 45) {
      for (slot, item) in bot.menu().slots().iter().enumerate(){  
        if slot != 45 {
          if item.kind() == ItemKind::TotemOfUndying {
            STATES.set_plugin_activity(&bot.username(), "auto-totem", true);
            
            let inventory = bot.get_inventory();

            inventory.left_click(slot);
            bot.wait_ticks(randticks(1, 2)).await;
            inventory.left_click(45 as usize);

            STATES.set_plugin_activity(&bot.username(), "auto-totem", false);
          }
        }
      }
    } 
  }
}
