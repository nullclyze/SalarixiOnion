use azalea::SprintDirection;
use azalea::WalkDirection;
use azalea::container::ContainerHandleRef;
use azalea::core::direction::Direction;
use azalea::entity::LocalEntity;
use azalea::entity::dimensions::EntityDimensions;
use azalea::entity::metadata::AbstractAnimal;
use azalea::entity::metadata::Health;
use azalea::entity::{Physics, Dead, Position};
use azalea::entity::inventory::Inventory;
use azalea::entity::metadata::{Player, AbstractMonster};
use azalea::inventory::Menu;
use azalea::inventory::operations::ThrowClick;
use azalea::local_player::Hunger;
use azalea::pathfinder::PathfinderOpts;
use azalea::pathfinder::astar::PathfinderTimeout;
use azalea::pathfinder::goals::XZGoal;
use azalea::pathfinder::moves::basic::basic_move;
use azalea::player::GameProfileComponent;
use azalea::prelude::*;
use azalea::ecs::prelude::*;
use azalea::Vec3;
use azalea::core::position::BlockPos; 
use azalea::block::BlockState;
use azalea::protocol::packets::game::{ServerboundSwing, ServerboundUseItem, ServerboundPlayerAction};
use azalea::protocol::packets::game::s_interact::InteractionHand;
use azalea::protocol::packets::game::s_player_action::Action;
use azalea::registry::builtin::ItemKind;
use azalea::world::MinecraftEntityId;
use bevy_ecs::entity::Entity;
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::tools::*;


// Функция нахождения пустого слота в инвентаре
pub fn find_empty_slot_in_invenotry(menu: Menu) -> Option<usize> {
  for (slot, item) in menu.slots().iter().enumerate() { 
    if slot > 9 {
      if item.is_empty() {
        return Some(slot);
      }
    }
  }

  None
}

// Функция нахождения пустого слота в хотбаре
pub fn find_empty_slot_in_hotbar(menu: Menu) -> Option<u8> {
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
pub fn get_bot_physics(bot: &Client) -> Option<Physics> {
  if let Some(physics) = bot.get_component::<Physics>() {
    return Some(physics);
  }

  None
}

// Функция получения состояния блока
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

// Функция конвертации индекса hotbar-слота в индекс inventory-слота
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
    _ => 36
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
pub fn stop_bot_move(bot: &Client) {
  let nickname = bot.username();

  bot.stop_pathfinding();
  bot.walk(WalkDirection::None);

  STATES.set_mutual_states(&nickname, "walking", false);
  STATES.set_mutual_states(&nickname, "sprinting", false);
}

// Функция безопасного получения позиции глаз
pub fn get_eye_position(bot: &Client) -> Vec3 {
  if let Some(dimensions) = bot.get_component::<EntityDimensions>() {
    return bot.position().up(dimensions.eye_height as f64);
  }

  Vec3::ZERO
}

// Функция безопасного открытия инвентаря
pub fn get_inventory(bot: &Client) -> Option<ContainerHandleRef> {
  if let Some(inventory) = bot.get_component::<Inventory>() {
    return Some(ContainerHandleRef::new(inventory.id, bot.clone()));
  }

  None
}

// Вспомогательная функция переключения состояний
pub fn start_interacting_with_inventory(bot: &Client, nickname: &String) {
  stop_bot_move(bot);

  STATES.set_state(&nickname, "can_walking", false);
  STATES.set_state(&nickname, "can_sprinting", false);
  STATES.set_state(&nickname, "can_attacking", false);
  STATES.set_state(&nickname, "can_interacting", false);
}

// Вспомогательная функция переключения состояний
pub fn stop_interacting_with_inventory(nickname: &String) {
  STATES.set_state(&nickname, "can_walking", true);
  STATES.set_state(&nickname, "can_sprinting", true);
  STATES.set_state(&nickname, "can_attacking", true);
  STATES.set_state(&nickname, "can_interacting", true);
}

// Функция безопасного получения выбранного слота в hotbar
pub fn get_selected_hotbar_slot(bot: &Client) -> u8 {
  if let Some(inventory) = bot.get_component::<Inventory>() {
    return inventory.selected_hotbar_slot;
  }

  0
}

// Функция безопасного получения текущего меню инвентаря
pub fn get_inventory_menu(bot: &Client) -> Option<Menu> {
  if let Some(inventory) = bot.get_component::<Inventory>() {
    return Some(inventory.menu().clone());
  }

  None
}

// Функция, позволяющая боту безопасно переместить предмет в hotbar и взять его
pub async fn take_item(bot: &Client, source_slot: usize) {
  if let Some(hotbar_slot) = convert_inventory_slot_to_hotbar_slot(source_slot) {
    if get_selected_hotbar_slot(bot) != hotbar_slot {
      bot.set_selected_hotbar_slot(hotbar_slot);
    }
  } else {
    if let Some(menu) = get_inventory_menu(bot) {
      if let Some(empty_slot) = find_empty_slot_in_hotbar(menu) {
        inventory_swap_click(bot, source_slot, empty_slot as usize).await;

        if let Some(slot) = convert_inventory_slot_to_hotbar_slot(empty_slot as usize) {
          if get_selected_hotbar_slot(bot) != slot {
            sleep(Duration::from_millis(50)).await;
            bot.set_selected_hotbar_slot(slot);
          }
        }
      } else {
        let random_slot = randuint(36, 44) as usize;

        inventory_shift_click(bot, random_slot);
        sleep(Duration::from_millis(50)).await;
        inventory_swap_click(bot, source_slot, random_slot).await;

        sleep(Duration::from_millis(50)).await;

        let hotbar_slot = convert_inventory_slot_to_hotbar_slot(random_slot).unwrap_or(0);

        if get_selected_hotbar_slot(bot) != hotbar_slot {
          bot.set_selected_hotbar_slot(hotbar_slot);
        }
      }
    }
  }
}

// Функция безопасного перемещения предмета в offhand
pub fn move_item_to_offhand(bot: &Client, kind: ItemKind) {
  if let Some(menu) = get_inventory_menu(bot) {
    if let Some(item) = menu.slot(45) {
      if item.kind() == kind {
        return;
      }
    }
  }

  bot.write_packet(ServerboundPlayerAction {
    action: Action::SwapItemWithOffhand,
    pos: BlockPos::new(0, 0, 0),
    direction: Direction::Down,
    seq: 0
  });
}

// Функция безопасного shift-клика в инвентаре
pub fn inventory_shift_click(bot: &Client, slot: usize) {
  if let Some(inventory) = get_inventory(bot) {
    let nickname = bot.username();
    start_interacting_with_inventory(bot, &nickname);
    inventory.shift_click(slot);
    stop_interacting_with_inventory(&nickname);
  }
}

// Функция безопасного left-клика в инвентаре
pub fn inventory_left_click(bot: &Client, slot: usize) {
  if let Some(inventory) = get_inventory(bot) {
    let nickname = bot.username();
    start_interacting_with_inventory(bot, &nickname);
    inventory.left_click(slot);
    stop_interacting_with_inventory(&nickname);
  }
}

// Функция безопасного right-клика в инвентаре
pub fn inventory_right_click(bot: &Client, slot: usize) {
  if let Some(inventory) = get_inventory(bot) {
    let nickname = bot.username();
    start_interacting_with_inventory(bot, &nickname);
    inventory.shift_click(slot);
    stop_interacting_with_inventory(&nickname);
  }
}

// Функция безопасного swap-клика в инвентаре
pub async fn inventory_swap_click(bot: &Client, source_slot: usize, target_slot: usize) {
  if let Some(inventory) = get_inventory(bot) {
    let nickname = bot.username();

    start_interacting_with_inventory(bot, &nickname);

    if let Some(menu) = get_inventory_menu(bot) {
      if let Some(item) = menu.slot(target_slot) {
        if !item.is_empty() {
          if let Some(empty_slot) = find_empty_slot_in_invenotry(menu) {
            inventory.left_click(target_slot);
            sleep(Duration::from_millis(50)).await;
            inventory.left_click(empty_slot);
          } else {
            inventory_drop_item(bot, target_slot);
          }

          sleep(Duration::from_millis(50)).await;
        }
      }
    }

    inventory.left_click(source_slot);
    sleep(Duration::from_millis(50)).await;
    inventory.left_click(target_slot);

    stop_interacting_with_inventory(&nickname);
  }
}

// Функция безопасного выбрасывания предмета
pub fn inventory_drop_item(bot: &Client, slot: usize) {
  if let Some(inventory) = get_inventory(bot) {
    let nickname = bot.username();

    start_interacting_with_inventory(bot, &nickname);

    if let Some(menu) = get_inventory_menu(bot) {
      if let Some(item) = menu.slot(slot) {
        if !item.is_empty() {
          inventory.click(ThrowClick::All { slot: slot as u16 });
        }
      }
    }

    stop_interacting_with_inventory(&nickname);
  }
}

// Функция безопасного перемещения предмета
pub async fn inventory_move_item(bot: &Client, kind: ItemKind, source_slot: usize, target_slot: usize) {
  if let Some(menu) = get_inventory_menu(bot) {
    if let Some(item) = menu.slot(target_slot) {
      if item.kind() == kind {
        return;
      }
    }
  }

  inventory_swap_click(bot, source_slot, target_slot).await;
}

// Функция безопасного задания направления хотьбы для бота
pub fn go(bot: &Client, direction: WalkDirection) {
  let nickname = bot.username();

  if STATES.get_state(&nickname, "can_walking") {
    STATES.set_mutual_states(&nickname, "walking", true);
    bot.walk(direction);
  }
}

// Функция безопасного задания направления бега для бота
pub fn run(bot: &Client, direction: SprintDirection) {
  let nickname = bot.username();
  
  if STATES.get_state(&nickname, "can_sprinting") {
    STATES.set_mutual_states(&nickname, "sprinting", true);
    bot.sprint(direction);
  }
}

// Функция безопасного задания координат по X и Z для бота
pub fn go_to(bot: Client, x: i32, z: i32) {
  let nickname = bot.username();
  
  if STATES.get_state(&nickname, "can_walking") && STATES.get_state(&nickname, "can_sprinting") {
    tokio::spawn(async move {
      STATES.set_mutual_states(&nickname, "sprinting", true);
      STATES.set_mutual_states(&nickname, "walking", true);

      let goal = XZGoal { x: x, z: z };
      let opts = PathfinderOpts::new()  
        .min_timeout(PathfinderTimeout::Time(Duration::from_millis(500)))  
        .max_timeout(PathfinderTimeout::Time(Duration::from_millis(1000)))  
        .allow_mining(false)  
        .successors_fn(basic_move);
      
      bot.goto_with_opts(goal, opts).await;

      STATES.set_mutual_states(&nickname, "sprinting", false);
      STATES.set_mutual_states(&nickname, "walking", false);
    });
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

// Функция распознавания твёрдого блока
pub fn this_is_solid_block(kind: ItemKind) -> bool {
  match kind {
    ItemKind::GrassBlock => return true,
    ItemKind::Podzol => return true,
    ItemKind::Mycelium => return true,
    ItemKind::DirtPath => return true,
    ItemKind::Dirt => return true,
    ItemKind::CoarseDirt => return true,
    ItemKind::RootedDirt => return true,
    ItemKind::Farmland => return true,
    ItemKind::Mud => return true,
    ItemKind::Clay => return true,
    ItemKind::Sandstone => return true,
    ItemKind::RedSandstone => return true,
    ItemKind::Ice => return true,
    ItemKind::PackedIce => return true,
    ItemKind::BlueIce => return true,
    ItemKind::SnowBlock => return true,
    ItemKind::MossBlock => return true,
    ItemKind::PaleMossBlock => return true,
    ItemKind::Stone => return true,
    ItemKind::Deepslate => return true,
    ItemKind::Granite => return true,
    ItemKind::Diorite => return true,
    ItemKind::Andesite => return true,
    ItemKind::Calcite => return true,
    ItemKind::Tuff => return true,
    ItemKind::DripstoneBlock => return true,
    ItemKind::Prismarine => return true,
    ItemKind::Obsidian => return true,
    ItemKind::CryingObsidian => return true,
    ItemKind::Netherrack => return true,
    ItemKind::CrimsonNylium => return true,
    ItemKind::WarpedNylium => return true,
    ItemKind::SoulSoil => return true,
    ItemKind::BoneBlock => return true,
    ItemKind::Blackstone => return true,
    ItemKind::Basalt => return true,
    ItemKind::SmoothBasalt => return true,
    ItemKind::EndStone => return true,
    ItemKind::OakLog => return true,
    ItemKind::SpruceLog => return true,
    ItemKind::BirchLog => return true,
    ItemKind::JungleLog => return true,
    ItemKind::AcaciaLog => return true,
    ItemKind::DarkOakLog => return true,
    ItemKind::MangroveLog => return true,
    ItemKind::CherryLog => return true,
    ItemKind::PaleOakLog => return true,
    ItemKind::MushroomStem => return true,
    ItemKind::CrimsonStem => return true,
    ItemKind::WarpedStem => return true,
    ItemKind::WhiteWool => return true,
    ItemKind::LightGrayWool => return true,
    ItemKind::GrayWool => return true,
    ItemKind::BlackWool => return true,
    ItemKind::BrownWool => return true,
    ItemKind::RedWool => return true,
    ItemKind::OrangeWool => return true,
    ItemKind::YellowWool => return true,
    ItemKind::LimeWool => return true,
    ItemKind::GreenWool => return true,
    ItemKind::CyanWool => return true,
    ItemKind::LightBlueWool => return true,
    ItemKind::BlueWool => return true,
    ItemKind::PurpleWool => return true,
    ItemKind::MagentaWool => return true,
    ItemKind::PinkWool => return true,
    ItemKind::WhiteTerracotta => return true,
    ItemKind::LightGrayTerracotta => return true,
    ItemKind::GrayTerracotta => return true,
    ItemKind::BlackTerracotta => return true,
    ItemKind::BrownTerracotta => return true,
    ItemKind::RedTerracotta => return true,
    ItemKind::OrangeTerracotta => return true,
    ItemKind::YellowTerracotta => return true,
    ItemKind::LimeTerracotta => return true,
    ItemKind::GreenTerracotta => return true,
    ItemKind::CyanTerracotta => return true,
    ItemKind::LightBlueTerracotta => return true,
    ItemKind::BlueTerracotta => return true,
    ItemKind::PurpleTerracotta => return true,
    ItemKind::MagentaTerracotta => return true,
    ItemKind::PinkTerracotta => return true,
    ItemKind::WhiteConcrete => return true,
    ItemKind::LightGrayConcrete => return true,
    ItemKind::GrayConcrete => return true,
    ItemKind::BlackConcrete => return true,
    ItemKind::BrownConcrete => return true,
    ItemKind::RedConcrete => return true,
    ItemKind::OrangeConcrete => return true,
    ItemKind::YellowConcrete => return true,
    ItemKind::LimeConcrete => return true,
    ItemKind::GreenConcrete => return true,
    ItemKind::CyanConcrete => return true,
    ItemKind::LightBlueConcrete => return true,
    ItemKind::BlueConcrete => return true,
    ItemKind::PurpleConcrete => return true,
    ItemKind::MagentaConcrete => return true,
    ItemKind::PinkConcrete => return true,
    _ => return false
  }
}

// Структура EntityFilter
#[derive(Clone)]
pub struct EntityFilter {
  target: String,
  distance: f64,
  excluded_name: String,
  excluded_id: MinecraftEntityId
}

impl EntityFilter {
  pub fn new(bot: &Client, target: &str, distance: f64) -> Self {
    let entity_id = if let Some(id) = bot.get_entity_component::<MinecraftEntityId>(bot.entity) {
      id
    } else {
      MinecraftEntityId::default()
    };

    Self {
      target: target.to_string(),
      distance: distance,
      excluded_name: bot.username(),
      excluded_id: entity_id
    }
  }
}

// Функция получения ближайшей сущности
pub fn get_nearest_entity(bot: &Client, filter: EntityFilter) -> Option<Entity> {
  let eye_pos = get_eye_position(bot);

  match filter.target.as_str() {
    "player" => {
      return bot.nearest_entity_by::<(&GameProfileComponent, &Position, &MinecraftEntityId), (With<Player>, Without<LocalEntity>, Without<Dead>)>(|data: (&GameProfileComponent, &Position, &MinecraftEntityId)| {
        *data.0.0.name != filter.excluded_name && eye_pos.distance_to(**data.1) <= filter.distance && *data.2 != filter.excluded_id
      });
    },
    "monster" => {
      return bot.nearest_entity_by::<(&Position, &MinecraftEntityId), (With<AbstractMonster>, Without<LocalEntity>, Without<Dead>)>(|data: (&Position, &MinecraftEntityId)| {
        eye_pos.distance_to(**data.0) <= filter.distance && *data.1 != filter.excluded_id
      });
    },
    "animal" => {
      return bot.nearest_entity_by::<(&Position, &MinecraftEntityId), (With<AbstractAnimal>, Without<LocalEntity>, Without<Dead>)>(|data: (&Position, &MinecraftEntityId)| {
        eye_pos.distance_to(**data.0) <= filter.distance && *data.1 != filter.excluded_id
      });
    },
    "any" => {
      return bot.nearest_entity_by::<(&Position, &MinecraftEntityId), (Without<LocalEntity>, Without<Dead>)>(|data: (&Position, &MinecraftEntityId)| {
        eye_pos.distance_to(**data.0) <= filter.distance && *data.1 != filter.excluded_id
      });
    },
    _ => {
      return bot.nearest_entity_by::<(&GameProfileComponent, &Position, &MinecraftEntityId), (With<Player>, Without<LocalEntity>, Without<Dead>)>(|data: (&GameProfileComponent, &Position, &MinecraftEntityId)| {
        data.0.0.name == filter.target && eye_pos.distance_to(**data.1) <= filter.distance && *data.2 != filter.excluded_id
      });
    }
  }
}

// Функция безопасного получения здоровья
pub fn get_health(bot: &Client) -> u32 {
  if let Some(health) = bot.get_component::<Health>() {
    health.0 as u32
  } else {
    20
  }
}

// Функция безопасного получения сытости
pub fn get_satiety(bot: &Client) -> u32 {
  if let Some(hunger) = bot.get_component::<Hunger>() {
    hunger.food
  } else {
    20
  }
}