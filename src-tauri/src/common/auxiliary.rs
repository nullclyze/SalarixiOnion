use azalea::block::BlockState;
use azalea::container::ContainerHandleRef;
use azalea::core::position::BlockPos;
use azalea::ecs::query::{With, Without};
use azalea::entity::dimensions::EntityDimensions;
use azalea::entity::inventory::Inventory;
use azalea::entity::metadata::{AbstractAnimal, AbstractMonster, Player};
use azalea::entity::{Dead, LocalEntity, Physics, Position};
use azalea::inventory::Menu;
use azalea::local_player::TabList;
use azalea::player::GameProfileComponent;
use azalea::prelude::*;
use azalea::world::MinecraftEntityId;
use azalea::Vec3;
use bevy_ecs::entity::Entity;

use crate::base::*;
use crate::methods::SafeClientMethods;

/// Функция получения физики бота
pub fn get_bot_physics(bot: &Client) -> Option<Physics> {
  if let Some(physics) = bot.get_component::<Physics>() {
    return Some(physics);
  }

  None
}

/// Функция получения состояния блока
pub fn get_block_state(bot: &Client, block_pos: BlockPos) -> Option<BlockState> {
  let world_clone = bot.world();
  let world = world_clone.write();

  if let Some(state) = world.get_block_state(block_pos) {
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

/// Функция установки velocity по Y
pub fn set_bot_velocity_y(bot: &Client, velocity_y: f64) {
  let mut ecs = bot.ecs.lock();

  if let Some(mut physics) = ecs.get_mut::<Physics>(bot.entity) {
    physics.velocity.y = velocity_y;
  }
}

/// Функция установки параметра on_ground
pub fn set_bot_on_ground(bot: &Client, on_ground: bool) {
  let mut ecs = bot.ecs.lock();

  if let Some(mut physics) = ecs.get_mut::<Physics>(bot.entity) {
    physics.set_on_ground(on_ground);
  }
}

/// Функция получения UUID игрока
pub async fn get_player_uuid(nickname: String) -> Option<String> {
  for username in PROFILES.get_all().keys() {
    let uuid = BOT_REGISTRY
      .get_bot(username, async |bot| {
        let Some(tab_list) = bot.get_component::<TabList>() else {
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

/// Функция получения позиции сущности
pub fn get_entity_position(bot: &Client, entity: Entity) -> Vec3 {
  let position = bot.get_entity_component::<Position>(entity);

  if let Some(pos) = position {
    return Vec3::new(pos.x, pos.y, pos.z);
  }

  Vec3::new(0.0, 0.0, 0.0)
}

/// Функция безопасного получения высоты глаз сущности
pub fn get_entity_eye_height(bot: &Client, entity: Entity) -> f64 {
  if let Some(dimensions) = bot.get_entity_component::<EntityDimensions>(entity) {
    return dimensions.eye_height as f64;
  }

  0.0
}

/// Функция безопасного получения инвентаря бота
pub fn get_bot_inventory(bot: &Client) -> Option<ContainerHandleRef> {
  if let Some(inventory) = bot.get_component::<Inventory>() {
    return Some(ContainerHandleRef::new(inventory.id, bot.clone()));
  }

  None
}

/// Функция безопасного получения выбранного слота в hotbar у бота
pub fn get_bot_selected_hotbar_slot(bot: &Client) -> u8 {
  if let Some(inventory) = bot.get_component::<Inventory>() {
    return inventory.selected_hotbar_slot;
  }

  0
}

/// Функция безопасного получения текущего меню инвентаря у бота
pub fn get_bot_inventory_menu(bot: &Client) -> Option<Menu> {
  if let Some(inventory) = bot.get_component::<Inventory>() {
    return Some(inventory.menu().clone());
  }

  None
}

/// Структура EntityFilter
#[derive(Clone)]
pub struct EntityFilter {
  target: String,
  distance: f64,
  excluded_name: String,
  excluded_id: MinecraftEntityId,
}

impl EntityFilter {
  pub fn new(bot: &Client, target: &str, distance: f64) -> Self {
    Self {
      target: target.to_string(),
      distance: distance,
      excluded_name: bot.username(),
      excluded_id: bot.id(),
    }
  }
}

/// Функция получения ближайшей сущности
pub fn get_nearest_entity(bot: &Client, filter: EntityFilter) -> Option<Entity> {
  let eye_pos = bot.eye_pos();

  match filter.target.as_str() {
    "player" => {
      return bot.nearest_entity_by::<(&GameProfileComponent, &Position, &MinecraftEntityId), (With<Player>, Without<LocalEntity>, Without<Dead>)>(|data: (&GameProfileComponent, &Position, &MinecraftEntityId)| {
        *data.0.0.name != filter.excluded_name && eye_pos.distance_to(**data.1) <= filter.distance && *data.2 != filter.excluded_id
      });
    }
    "monster" => {
      return bot.nearest_entity_by::<(&Position, &MinecraftEntityId), (With<AbstractMonster>, Without<LocalEntity>, Without<Dead>)>(|data: (&Position, &MinecraftEntityId)| {
        eye_pos.distance_to(**data.0) <= filter.distance && *data.1 != filter.excluded_id
      });
    }
    "animal" => {
      return bot.nearest_entity_by::<(&Position, &MinecraftEntityId), (With<AbstractAnimal>, Without<LocalEntity>, Without<Dead>)>(|data: (&Position, &MinecraftEntityId)| {
        eye_pos.distance_to(**data.0) <= filter.distance && *data.1 != filter.excluded_id
      });
    }
    "any" => {
      return bot
				.nearest_entity_by::<(&Position, &MinecraftEntityId), (Without<LocalEntity>, Without<Dead>)>(
					|data: (&Position, &MinecraftEntityId)| {
						eye_pos.distance_to(**data.0) <= filter.distance && *data.1 != filter.excluded_id
					},
				);
    }
    _ => {
      return bot.nearest_entity_by::<(&GameProfileComponent, &Position, &MinecraftEntityId), (With<Player>, Without<LocalEntity>, Without<Dead>)>(|data: (&GameProfileComponent, &Position, &MinecraftEntityId)| {
        data.0.0.name == filter.target && eye_pos.distance_to(**data.1) <= filter.distance && *data.2 != filter.excluded_id
      });
    }
  }
}
