use azalea::SprintDirection;
use azalea::WalkDirection;
use azalea::container::ContainerHandleRef;
use azalea::core::direction::Direction;
use azalea::entity::Position;
use azalea::entity::inventory::Inventory;
use azalea::prelude::*;
use azalea::Vec3;
use azalea::core::position::BlockPos; 
use azalea::entity::Physics;
use azalea::block::BlockState;
use azalea::protocol::packets::game::ServerboundSwing;
use azalea::protocol::packets::game::ServerboundUseItem;
use azalea::protocol::packets::game::s_interact::InteractionHand;
use azalea::protocol::packets::game::ServerboundPlayerAction;
use azalea::protocol::packets::game::s_player_action::Action;
use azalea::registry::builtin::ItemKind;
use bevy_ecs::entity::Entity;
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
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

// Функция конвертации индекса inventory-слота в индекс hotbar-слота
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

// Функция остановки движения бота
pub async fn stop_bot_walking(bot: &Client) {
  let nickname = bot.username();

  STATES.set_state(&nickname, "can_walking", false);
  STATES.set_state(&nickname, "can_sprinting", false);

  bot.walk(WalkDirection::None);

  STATES.set_state(&nickname, "is_walking", false);
  STATES.set_state(&nickname, "is_sprinting", false);

  sleep(Duration::from_millis(50)).await;
}

// Функция безопасного получения инвентаря
pub fn get_inventory(bot: &Client) -> Option<ContainerHandleRef> {
  if let Some(inventory) = bot.get_component::<Inventory>() {
    return Some(ContainerHandleRef::new(inventory.id, bot.clone()));
  }

  None
}

// Функция, позволяющая боту безопасно переместить предмет в hotbar и взять его
pub async fn take_item(bot: &Client, source_slot: usize) {
  if let Some(inventory) = get_inventory(bot) {
    let nickname = bot.username();

    stop_bot_walking(bot).await;

    if let Some(hotbar_slot) = convert_inventory_slot_to_hotbar_slot(source_slot) {
      if bot.selected_hotbar_slot() != hotbar_slot {
        bot.set_selected_hotbar_slot(hotbar_slot);
      }
    } else {
      if let Some(empty_slot) = find_empty_slot_in_hotbar(bot) {
        inventory.left_click(source_slot);
        sleep(Duration::from_millis(50)).await;
        inventory.left_click(empty_slot);

        if let Some(slot) = convert_inventory_slot_to_hotbar_slot(empty_slot as usize) {
          if bot.selected_hotbar_slot() != slot {
            sleep(Duration::from_millis(50)).await;
            bot.set_selected_hotbar_slot(slot);
            sleep(Duration::from_millis(50)).await;
          }
        }
      } else {
        let random_slot = randuint(36, 44) as usize;

        inventory.shift_click(random_slot);
        sleep(Duration::from_millis(50)).await;
        inventory.left_click(source_slot);
        sleep(Duration::from_millis(50)).await;
        inventory.left_click(random_slot);

        sleep(Duration::from_millis(50)).await;

        let hotbar_slot = convert_inventory_slot_to_hotbar_slot(random_slot).unwrap_or(0);

        if bot.selected_hotbar_slot() != hotbar_slot {
          bot.set_selected_hotbar_slot(hotbar_slot);
        }
      }

      inventory.close();
    }

    STATES.set_state(&nickname, "can_walking", true);
    STATES.set_state(&nickname, "can_sprinting", true);
  }
}

// Функция безопасного перемещения предмета
pub async fn move_item(bot: &Client, kind: ItemKind, source_slot: usize, target_slot: usize) {
  if let Some(inventory) = get_inventory(bot) {
    let nickname = bot.username();

    stop_bot_walking(bot).await;

    if let Some(item) = bot.menu().slot(target_slot) {
      if item.kind() == kind {
        return;
      }

      if !item.is_empty() {
        inventory.shift_click(target_slot);
        sleep(Duration::from_millis(50)).await;
      }
    }

    inventory.left_click(source_slot);
    sleep(Duration::from_millis(50)).await;
    inventory.left_click(target_slot);

    sleep(Duration::from_millis(50)).await;

    inventory.close();

    STATES.set_state(&nickname, "can_walking", true);
    STATES.set_state(&nickname, "can_sprinting", true);
  }
}

// Функция безопасного перемещения предмета в offhand
pub async fn move_item_to_offhand(bot: &Client, kind: ItemKind) {
  let nickname = bot.username();

  stop_bot_walking(bot).await;

  if let Some(item) = bot.menu().slot(45) {
    if item.kind() == kind {
      return;
    }
  }

  bot.write_packet(ServerboundPlayerAction {
    action: Action::SwapItemWithOffhand,
    pos: BlockPos::new(0, 0, 0),
    direction: Direction::Down,
    seq: 0
  });

  STATES.set_state(&nickname, "can_walking", true);
  STATES.set_state(&nickname, "can_sprinting", true);
}

// Функция безопасного задания направления хотьбы для бота
pub fn go(bot: &Client, direction: WalkDirection) {
  let nickname = bot.username();

  if STATES.get_state(&nickname, "can_walking") {
    STATES.set_state(&nickname, "is_walking", true);
    bot.walk(direction);
  }
}

// Функция безопасного задания направления бега для бота
pub fn run(bot: &Client, direction: SprintDirection) {
  let nickname = bot.username();
  
  if STATES.get_state(&nickname, "can_sprinting") {
    STATES.set_state(&nickname, "is_sprinting", true);
    bot.sprint(direction);
  }
}

// Функция отправки пакета SwingArm
pub fn swing_arm(bot: &Client) {
  bot.write_packet(ServerboundSwing {
    hand: InteractionHand::MainHand
  });
}

// Функция отправки пакета StartUseItem
pub fn start_use_item(bot: &Client, hand: InteractionHand) {
  let direction = bot.direction();

  bot.write_packet(ServerboundUseItem {  
    hand: hand,
    y_rot: direction.0,
    x_rot: direction.1,
    seq: 0
  });
}

// Функция отправки пакета ReleaseUseItem
pub fn release_use_item(bot: &Client) {
  bot.write_packet(ServerboundPlayerAction {  
    action: Action::ReleaseUseItem,  
    pos: BlockPos::new(0, 0, 0),  
    direction: Direction::Down,  
    seq: 0
  });
}