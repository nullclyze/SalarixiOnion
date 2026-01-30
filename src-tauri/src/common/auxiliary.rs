use azalea::SprintDirection;
use azalea::WalkDirection;
use azalea::entity::Position;
use azalea::prelude::*;
use azalea::Vec3;
use azalea::core::position::BlockPos; 
use azalea::entity::Physics;
use azalea::block::BlockState;
use bevy_ecs::entity::Entity;
use std::time::Duration;
use tokio::time::sleep;

use crate::base::get_flow_manager;
use crate::state::STATES;
use crate::tools::randuint;


// Функция нахождения пустого слота в инвентаре
pub fn find_empty_slot_in_invenotry(bot: &Client) -> Option<usize> {
  for (slot, item) in bot.menu().slots().iter().enumerate() { 
    if slot > 9 {
      if item.is_empty() {
        return Some(slot);
      }
    }
  }

  None
}

// Функция нахождения пустого слота в хотбаре
pub fn find_empty_slot_in_hotbar(bot: &Client) -> Option<u8> {
  let menu = bot.menu();

  for slot in menu.hotbar_slots_range() { 
    if let Some(item) = menu.slot(slot) {
      if item.is_empty() {
        return Some(slot as u8);
      }
    }
  }

  None
}

// Функция проверки слота на наличие предмета
pub fn is_this_slot_empty(bot: &Client, slot: usize) -> bool {
  if let Some(item) = bot.menu().slot(slot) {
    if !item.is_empty() {
      return false;
    }
  }

  true
}

// Функция получения физики бота
pub fn get_bot_physics(bot: &Client) -> Physics {
  let mut ecs = bot.ecs.lock(); 
  ecs.get_mut::<Physics>(bot.entity).unwrap().clone()
}

// Функция получения состояния блока
pub fn get_block_state(bot: &Client, block_pos: BlockPos) -> Option<BlockState> {
  let world_clone = bot.world().clone();
  let world = world_clone.write();

  if let Some(state) = world.get_block_state(block_pos) {
    return Some(state);
  }

  None
}

// Функция получения средних координат ботов
pub fn get_average_coordinates_of_bots(positions: Vec<Vec3>) -> (f64, f64, f64) {
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

  for coordinate in x_coords.clone() {
    x_global = x_global + coordinate;
  }

  for coordinate in y_coords.clone() {
    y_global = y_global + coordinate;
  }

  for coordinate in z_coords.clone() {
    z_global = z_global + coordinate;
  }

  let x_average = x_global / x_coords.len() as f64;
  let y_average = y_global / y_coords.len() as f64;
  let z_average = z_global / z_coords.len() as f64;

  (x_average, y_average, z_average)
}

// Функция установки velocity по Y
pub fn set_bot_velocity_y(bot: &Client, velocity_y: f64) {
  let mut ecs = bot.ecs.lock(); 

  if let Some(mut physics) = ecs.get_mut::<Physics>(bot.entity) {
    physics.velocity.y = velocity_y;
  }
} 

// Функция установки параметра on_ground
pub fn set_bot_on_ground(bot: &Client, on_ground: bool) {
  let mut ecs = bot.ecs.lock(); 
  
  if let Some(mut physics) = ecs.get_mut::<Physics>(bot.entity) {
    physics.set_on_ground(on_ground);
  }
}

// Функция конвертировки индекса inventory-слота в индекс hotbar-слота
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
    _ => None
  }
}

// Функция получения UUID игрока
pub fn get_player_uuid(nickname: String) -> Option<String> {
  if let Some(arc) = get_flow_manager() {
    let fm = arc.write();

    if let Some(swarm) = fm.swarm.clone() {
      for bot in swarm {
        let tab_list = bot.tab_list();

        for (uuid, info) in tab_list {
          if info.profile.name == nickname {
            return Some(uuid.to_string());
          }
        }
      }
    }
  }

  None
}

// Функция получения позиции сущности
pub fn get_entity_position(bot: &Client, entity: Entity) -> Vec3 {
  let position = bot.get_entity_component::<Position>(entity);

  if let Some(pos) = position {
    return Vec3::new(pos.x, pos.y, pos.z);
  }

  Vec3::new(0.0, 0.0, 0.0)
}

// Функция перемещения предмета в hotbar
pub async fn move_item_to_hotbar(bot: &Client, source_slot: usize) {
  let nickname = bot.username();

  STATES.set(&nickname, "can_walk", "false".to_string());
  
  let inventory = bot.get_inventory();

  if let Some(hotbar_slot) = convert_inventory_slot_to_hotbar_slot(source_slot) {
    if bot.selected_hotbar_slot() != hotbar_slot {
      bot.set_selected_hotbar_slot(hotbar_slot);
    }
  } else {
    if let Some(empty_slot) = find_empty_slot_in_hotbar(bot) {
      bot.walk(WalkDirection::None);

      inventory.left_click(source_slot);
      sleep(Duration::from_millis(50)).await;
      inventory.left_click(empty_slot);

      if let Some(slot) = convert_inventory_slot_to_hotbar_slot(empty_slot as usize) {
        if bot.selected_hotbar_slot() != slot {
          bot.set_selected_hotbar_slot(slot);
          sleep(Duration::from_millis(50)).await;
        }
      }
    } else {
      let random_slot = randuint(36, 44) as usize;
      
      bot.walk(WalkDirection::None);

      inventory.shift_click(random_slot);
      sleep(Duration::from_millis(50)).await;
      inventory.left_click(source_slot);
      sleep(Duration::from_millis(50)).await;
      inventory.left_click(random_slot);

      let hotbar_slot = convert_inventory_slot_to_hotbar_slot(random_slot).unwrap_or(0);

      if bot.selected_hotbar_slot() != hotbar_slot {
        bot.set_selected_hotbar_slot(hotbar_slot);
      }
    }
  }

  STATES.set(&nickname, "can_walk", "true".to_string());
}

// Функция безопасного задания направления хотьбы для бота
pub fn go(bot: &Client, direction: WalkDirection) {
  if STATES.can_walk(&bot.username()) {
    bot.walk(direction);
  }
}

// Функция безопасного задания направления бега для бота
pub fn run(bot: &Client, direction: SprintDirection) {
  if STATES.can_walk(&bot.username()) {
    bot.sprint(direction);
  }
}