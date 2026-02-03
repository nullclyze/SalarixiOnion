use azalea::prelude::*;
use azalea::inventory::operations::{SwapClick, ThrowClick};
use serde::{Serialize, Deserialize};
use std::time::Duration;
use tokio::time::sleep;

use crate::base::STATES;
use crate::common::{get_inventory, stop_bot_sprinting, stop_bot_walking};
use crate::emit::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryOptions {
  pub slot: Option<u16>,
  pub target_slot: Option<u16>,
  pub state: String
}

impl InventoryModule { 
  pub async fn action(bot: &Client, options: InventoryOptions) {
    if let Some(s) = options.slot {
      if let Some(inventory) = get_inventory(bot) {
        let nickname = bot.username();

        stop_bot_sprinting(bot).await;
        stop_bot_walking(bot).await;

        match options.state.as_str() {
          "select" => { 
            if s <= 8 {
              bot.set_selected_hotbar_slot(s as u8);
            } else {
              emit_event(EventType::Log(LogEventPayload {
                name: "error".to_string(),
                message: format!("Бот {} не смог взять слот {} (индекс слота не должен превышать 8)", nickname, s)
              }));
            }
          },
          "drop" => {
            inventory.click(ThrowClick::All { slot: s });
          },
          "left-click" => {
            inventory.left_click(s);
          },
          "right-click" => {
            inventory.right_click(s);
          },
          "swap" => {
            inventory.click(SwapClick { source_slot: s, target_slot: if let Some(t) = options.target_slot { t as u8 } else { 0 }});
          },
          _ => {}
        }

        sleep(Duration::from_millis(50)).await;

        inventory.close();

        STATES.set_state(&nickname, "can_sprinting", true);
        STATES.set_state(&nickname, "can_walking", true);
      }
    } 
  }
}
