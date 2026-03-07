use azalea::block::BlockState;
use azalea::core::position::BlockPos;
use azalea::prelude::*;
use azalea::Vec3;

use crate::core::*;
use crate::extensions::BotDefaultExt;

/// Функция получения состояния блока
pub fn get_block_state(bot: &Client, block_pos: BlockPos) -> Option<BlockState> {
  if let Some(state) = bot.world().read().get_block_state(block_pos) {
    return Some(state);
  }

  None
}

// Функция получения средних координат ботов
pub fn get_average_coordinates_of_bots(positions: &Vec<Vec3>) -> (f64, f64, f64) {
  let mut x_coords = vec![];
  let mut y_coords = vec![];
  let mut z_coords = vec![];

  for pos in positions {
    x_coords.push(pos.x);
    y_coords.push(pos.x);
    z_coords.push(pos.z);
  }

  let mut x_global = 0.0;
  let mut y_global = 0.0;
  let mut z_global = 0.0;

  for coordinate in &x_coords {
    x_global = x_global + *coordinate;
  }

  for coordinate in &y_coords {
    y_global = y_global + *coordinate;
  }

  for coordinate in &z_coords {
    z_global = z_global + *coordinate;
  }

  let x_average = x_global / x_coords.len() as f64;
  let y_average = y_global / y_coords.len() as f64;
  let z_average = z_global / z_coords.len() as f64;

  (x_average, y_average, z_average)
}

/// Функция получения UUID игрока
pub async fn get_player_uuid(nickname: String) -> Option<String> {
  for username in PROFILES.get_all().keys() {
    let uuid = BOT_REGISTRY
      .async_get_bot(username, async |bot| {
        let Some(tab_list) = bot.get_players() else {
          return None;
        };

        for (uuid, info) in tab_list.iter() {
          if info.profile.name == nickname {
            return Some(uuid.to_string());
          }
        }

        None
      })
      .await;

    if let Some(u) = uuid {
      return u;
    }
  }

  None
}

/// Функция конвертации индекса inventory-слота в индекс hotbar-слота
pub fn convert_inventory_slot_to_hotbar_slot(slot: usize) -> Option<u8> {
  match slot {
    36 => Some(0),
    37 => Some(1),
    38 => Some(2),
    39 => Some(3),
    40 => Some(4),
    41 => Some(5),
    42 => Some(6),
    43 => Some(7),
    44 => Some(8),
    _ => None,
  }
}

/// Функция конвертации индекса hotbar-слота в индекс inventory-слота
pub fn convert_hotbar_slot_to_inventory_slot(slot: u8) -> usize {
  match slot {
    0 => 36,
    1 => 37,
    2 => 38,
    3 => 39,
    4 => 40,
    5 => 41,
    6 => 42,
    7 => 43,
    8 => 44,
    _ => 36,
  }
}
