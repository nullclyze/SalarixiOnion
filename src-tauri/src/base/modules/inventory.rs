use azalea::prelude::*;
use azalea::inventory::operations::{SwapClick, ThrowClick};
use serde::{Serialize, Deserialize};


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
      match options.state.as_str() {
        "select" => { 
          bot.set_selected_hotbar_slot(s as u8);
        },
        "drop" => {
          bot.get_inventory().click(ThrowClick::All { slot: s });
        },
        "left-click" => {
          bot.get_inventory().left_click(s);
        },
        "right-click" => {
          bot.get_inventory().right_click(s);
        },
        "swap" => {
          bot.get_inventory().click(SwapClick { source_slot: s, target_slot: if let Some(t) = options.target_slot { t as u8 } else { 0 }});
        },
        _ => {}
      }
    } 
  }
}
