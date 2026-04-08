use serde::{Deserialize, Serialize};

use crate::api::emit::*;
use crate::service::core::extensions::{BotInventoryExt, InvClick};
use crate::service::core::registry::BOT_REGISTRY;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InventoryOptions {
  pub slot: Option<usize>,
  pub target_slot: Option<usize>,
  pub state: String,
}

impl InventoryModule {
  pub fn new() -> Self {
    Self
  }

  pub async fn interact(&self, username: &str, options: &InventoryOptions) {
    BOT_REGISTRY
      .async_get_bot(username, async |bot| {
        if let Some(s) = options.slot {
          if options.state.as_str() == "select" {
            if s <= 8 {
              bot.set_selected_hotbar_slot(s as u8);
            } else {
              send_log(format!("Бот {} не смог взять слот {} (неверный индекс слота)", username, s), "error");
            }
          } else {
            match options.state.as_str() {
              "drop" => {
                bot.inventory_click(s, InvClick::from(4), true);
              }
              "left-click" => {
                bot.inventory_click(s, InvClick::from(0), true);
              }
              "right-click" => {
                bot.inventory_click(s, InvClick::from(1), true);
              }
              "swap" => {
                bot.inventory_swap_click(s, if let Some(t) = options.target_slot { t } else { 0 }, true).await;
              }
              _ => {}
            }
          }
        }
      })
      .await;
  }
}
