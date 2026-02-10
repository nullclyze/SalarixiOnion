use azalea::prelude::*;
use serde::{Serialize, Deserialize};

use crate::emit::*;
use crate::common::{inventory_drop_item, inventory_left_click, inventory_right_click, inventory_swap_click};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryOptions {
  pub slot: Option<usize>,
  pub target_slot: Option<usize>,
  pub state: String
}

impl InventoryModule { 
  pub fn new() -> Self {
    Self
  }

  pub async fn interact(&self, bot: &Client, options: &InventoryOptions) {
    if let Some(s) = options.slot {
      let nickname = bot.username();

      if options.state.as_str() == "select" {
        if s <= 8 {
          bot.set_selected_hotbar_slot(s as u8);
        } else {
          emit_event(EventType::Log(LogEventPayload {
            name: "error".to_string(),
            message: format!("Бот {} не смог взять слот {} (неверный индекс слота)", nickname, s)
          }));
        }
      } else {
        match options.state.as_str() {
          "drop" => {
            inventory_drop_item(bot, s);
          },
          "left-click" => {
            inventory_left_click(bot, s);
          },
          "right-click" => {
            inventory_right_click(bot, s);
          },
          "swap" => {
            inventory_swap_click(bot, s, if let Some(t) = options.target_slot { t } else { 0 }).await;
          },
          _ => {}
        }
      }
    } 
  }
}
