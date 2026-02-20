use serde::{Deserialize, Serialize};

use crate::base::BOT_REGISTRY;
use crate::common::{
  inventory_drop_item, inventory_left_click, inventory_right_click, inventory_swap_click,
};
use crate::emit::*;

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
      .get_bot(username, async |bot| {
        if let Some(s) = options.slot {
          if options.state.as_str() == "select" {
            if s <= 8 {
              bot.set_selected_hotbar_slot(s as u8);
            } else {
              send_log(
                format!(
                  "Бот {} не смог взять слот {} (неверный индекс слота)",
                  username, s
                ),
                "error",
              );
            }
          } else {
            match options.state.as_str() {
              "drop" => {
                inventory_drop_item(bot, s, true);
              }
              "left-click" => {
                inventory_left_click(bot, s, true);
              }
              "right-click" => {
                inventory_right_click(bot, s, true);
              }
              "swap" => {
                inventory_swap_click(
                  bot,
                  s,
                  if let Some(t) = options.target_slot {
                    t
                  } else {
                    0
                  },
                  true,
                )
                .await;
              }
              _ => {}
            }
          }
        }
      })
      .await;
  }
}
