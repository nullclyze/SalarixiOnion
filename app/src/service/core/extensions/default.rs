use azalea::ecs::entity::Entity;
use azalea::ecs::query::{With, Without};
use azalea::entity::metadata::{AbstractAnimal, AbstractMonster, AbstractVehicle, Player};
use azalea::entity::{dimensions::EntityDimensions, metadata::Health, Crouching, Jumping, LookDirection, Position};
use azalea::entity::{Dead, LocalEntity};
use azalea::local_player::{Hunger, TabList};
use azalea::player::GameProfileComponent;
use azalea::protocol::packets::game::{s_interact::InteractionHand, ServerboundSwing};
use azalea::world::MinecraftEntityId;
use azalea::{Client, InGameState, Vec3};

pub enum EntityType {
  Player,
  Monster,
  Animal,
  Vehicle,
  Any,
  Custom(String),
}

pub fn entity_type_from(value: String) -> EntityType {
  match value.as_str() {
    "player" => EntityType::Player,
    "monster" => EntityType::Monster,
    "animal" => EntityType::Animal,
    "vehicle" => EntityType::Vehicle,
    "any" => EntityType::Any,
    _ => EntityType::Custom(value),
  }
}

pub trait BotDefaultExt {
  fn workable(&self) -> bool;
  fn name(&self) -> String;
  fn id(&self) -> MinecraftEntityId;
  fn ping(&self) -> u32;
  fn feet_pos(&self) -> Vec3;
  fn eye_pos(&self) -> Vec3;
  fn get_health(&self) -> f32;
  fn get_satiety(&self) -> u32;
  fn get_players(&self) -> Option<TabList>;
  fn swing_arm(&self);
  fn start_jumping(&self);
  fn start_crouching(&self);
  fn stop_jumping(&self);
  fn stop_crouching(&self);
  fn get_entity_eye_height(&self, entity: Entity) -> f64;
  fn get_entity_position(&self, entity: Entity) -> Vec3;
  fn find_nearest_entity(&self, entity_type: EntityType, distance: f64) -> Option<Entity>;
}

impl BotDefaultExt for Client {
  fn workable(&self) -> bool {
    let position_exists = self.get_component::<Position>().is_some();
    let look_direction_exists = self.get_component::<LookDirection>().is_some();
    let dimensions_exists = self.get_component::<EntityDimensions>().is_some();
    let profile_exist = self.get_component::<GameProfileComponent>().is_some();
    let in_game_state_exist = self.get_component::<InGameState>().is_some();
    let states_exist = self.get_component::<Crouching>().is_some() && self.get_component::<Jumping>().is_some();

    position_exists && look_direction_exists && dimensions_exists && profile_exist && in_game_state_exist && states_exist
  }

  fn name(&self) -> String {
    if let Some(profile) = self.get_component::<GameProfileComponent>() {
      return profile.name.to_owned();
    }

    String::new()
  }

  fn id(&self) -> MinecraftEntityId {
    self.get_component::<MinecraftEntityId>().unwrap_or(MinecraftEntityId::default())
  }

  fn ping(&self) -> u32 {
    if let Some(tab) = self.get_players() {
      let mut result = 0;

      for (_, info) in tab.iter() {
        if info.profile.name == self.name() {
          result = info.latency as u32;
          break;
        }
      }

      result
    } else {
      0
    }
  }

  fn feet_pos(&self) -> Vec3 {
    *self.get_component::<Position>().unwrap_or(Position::new(Vec3::ZERO))
  }

  fn eye_pos(&self) -> Vec3 {
    let feet_pos = *self.get_component::<Position>().unwrap_or(Position::new(Vec3::ZERO));

    let Some(dimensions) = self.get_component::<EntityDimensions>() else {
      return feet_pos;
    };

    Vec3 {
      x: feet_pos.x,
      y: feet_pos.y + dimensions.eye_height as f64,
      z: feet_pos.z,
    }
  }

  fn get_health(&self) -> f32 {
    if let Some(health) = self.get_component::<Health>() {
      health.0
    } else {
      20.0
    }
  }

  fn get_satiety(&self) -> u32 {
    if let Some(hunger) = self.get_component::<Hunger>() {
      hunger.food
    } else {
      20
    }
  }

  fn get_players(&self) -> Option<TabList> {
    self.get_component::<TabList>()
  }

  fn swing_arm(&self) {
    self.write_packet(ServerboundSwing {
      hand: InteractionHand::MainHand,
    });
  }

  fn stop_jumping(&self) {
    if let Some(jumping) = self.get_component::<Jumping>() {
      if jumping.0 {
        self.set_jumping(false);
      }
    }
  }

  fn stop_crouching(&self) {
    if self.crouching() {
      self.set_crouching(false);
    }
  }

  fn start_jumping(&self) {
    if let Some(jumping) = self.get_component::<Jumping>() {
      if !jumping.0 {
        self.set_jumping(true);
      }
    }
  }

  fn start_crouching(&self) {
    if !self.crouching() {
      self.set_crouching(true);
    }
  }

  fn get_entity_position(&self, entity: Entity) -> Vec3 {
    let position = self.get_entity_component::<Position>(entity);

    if let Some(pos) = position {
      return Vec3::new(pos.x, pos.y, pos.z);
    }

    Vec3::new(0.0, 0.0, 0.0)
  }

  fn get_entity_eye_height(&self, entity: Entity) -> f64 {
    if let Some(dimensions) = self.get_entity_component::<EntityDimensions>(entity) {
      return dimensions.eye_height as f64;
    }

    0.0
  }

  fn find_nearest_entity(&self, entity_type: EntityType, distance: f64) -> Option<Entity> {
    let excluded_name = self.name();
    let excluded_id = self.id();
    let eye_pos = self.eye_pos();

    match entity_type {
      EntityType::Player => {
        return self.nearest_entity_by::<(&GameProfileComponent, &Position, &MinecraftEntityId), (With<Player>, Without<LocalEntity>, Without<Dead>)>(
          |data: (&GameProfileComponent, &Position, &MinecraftEntityId)| {
            *data.0 .0.name != excluded_name && eye_pos.distance_to(**data.1) <= distance && *data.2 != excluded_id
          },
        );
      }
      EntityType::Monster => {
        return self.nearest_entity_by::<(&Position, &MinecraftEntityId), (With<AbstractMonster>, Without<LocalEntity>, Without<Dead>)>(
          |data: (&Position, &MinecraftEntityId)| eye_pos.distance_to(**data.0) <= distance && *data.1 != excluded_id,
        );
      }
      EntityType::Animal => {
        return self.nearest_entity_by::<(&Position, &MinecraftEntityId), (With<AbstractAnimal>, Without<LocalEntity>, Without<Dead>)>(
          |data: (&Position, &MinecraftEntityId)| eye_pos.distance_to(**data.0) <= distance && *data.1 != excluded_id,
        );
      }
      EntityType::Vehicle => {
        return self.nearest_entity_by::<(&Position, &MinecraftEntityId), (With<AbstractVehicle>, Without<LocalEntity>, Without<Dead>)>(
          |data: (&Position, &MinecraftEntityId)| eye_pos.distance_to(**data.0) <= distance && *data.1 != excluded_id,
        );
      }
      EntityType::Any => {
        return self.nearest_entity_by::<(&Position, &MinecraftEntityId), (Without<LocalEntity>, Without<Dead>)>(|data: (&Position, &MinecraftEntityId)| {
          eye_pos.distance_to(**data.0) <= distance && *data.1 != excluded_id
        });
      }
      EntityType::Custom(username) => {
        return self.nearest_entity_by::<(&GameProfileComponent, &Position, &MinecraftEntityId), (With<Player>, Without<LocalEntity>, Without<Dead>)>(
          |data: (&GameProfileComponent, &Position, &MinecraftEntityId)| {
            data.0 .0.name == username && eye_pos.distance_to(**data.1) <= distance && *data.2 != excluded_id
          },
        );
      }
    }
  }
}
