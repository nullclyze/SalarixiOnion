use azalea::{
  entity::{
    dimensions::EntityDimensions, metadata::Health, Crouching, Jumping, LookDirection, Position,
  },
  local_player::{Hunger, TabList},
  player::GameProfileComponent,
  protocol::packets::game::{s_interact::InteractionHand, ServerboundSwing},
  world::MinecraftEntityId,
  Client, InGameState, SprintDirection, StartSprintEvent, StartWalkEvent, Vec3, WalkDirection,
};

use crate::core::STATES;

/// Трейт безопасных методов для Client
pub trait SafeClientMethods {
  fn workable(&self) -> bool;
  fn id(&self) -> MinecraftEntityId;
  fn feet_pos(&self) -> Vec3;
  fn eye_pos(&self) -> Vec3;
  fn get_health(&self) -> f32;
  fn get_satiety(&self) -> u32;
  fn get_players(&self) -> Option<TabList>;
  fn swing_arm(&self);
  fn start_walking(&self, direction: WalkDirection);
  fn start_sprinting(&self, direction: SprintDirection);
  fn stop_move(&self);
  fn freeze_move(&self);
  fn unfreeze_move(&self);
  fn start_jumping(&self);
  fn start_crouching(&self);
  fn stop_jumping(&self);
  fn stop_crouching(&self);
}

impl SafeClientMethods for Client {
  fn workable(&self) -> bool {
    let position_exists = self.get_component::<Position>().is_some();
    let look_direction_exists = self.get_component::<LookDirection>().is_some();
    let dimensions_exists = self.get_component::<EntityDimensions>().is_some();
    let profile_exist = self.get_component::<GameProfileComponent>().is_some();
    let in_game_state_exist = self.get_component::<InGameState>().is_some();
    let states_exist =
      self.get_component::<Crouching>().is_some() && self.get_component::<Jumping>().is_some();

    position_exists
      && look_direction_exists
      && dimensions_exists
      && profile_exist
      && in_game_state_exist
      && states_exist
  }

  fn id(&self) -> MinecraftEntityId {
    self
      .get_component::<MinecraftEntityId>()
      .unwrap_or(MinecraftEntityId::default())
  }

  fn feet_pos(&self) -> Vec3 {
    *self
      .get_component::<Position>()
      .unwrap_or(Position::new(Vec3::ZERO))
  }

  fn eye_pos(&self) -> Vec3 {
    let feet_pos = *self
      .get_component::<Position>()
      .unwrap_or(Position::new(Vec3::ZERO));

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

  fn start_walking(&self, direction: WalkDirection) {
    let username = self.username();

    if STATES.get_state(&username, "can_walking")
      && !STATES.get_state(&username, "is_eating")
      && !STATES.get_state(&username, "is_drinking")
      && !STATES.get_state(&username, "is_interacting")
    {
      self.ecs.lock().write_message(StartWalkEvent {
        entity: self.entity,
        direction: direction,
      });

      STATES.set_mutual_states(&username, "walking", true);
    }
  }

  fn start_sprinting(&self, direction: SprintDirection) {
    let username = self.username();

    if STATES.get_state(&username, "can_sprinting")
      && !STATES.get_state(&username, "is_eating")
      && !STATES.get_state(&username, "is_drinking")
      && !STATES.get_state(&username, "is_interacting")
    {
      self.ecs.lock().write_message(StartSprintEvent {
        entity: self.entity,
        direction: direction,
      });

      STATES.set_mutual_states(&username, "sprinting", true);
    }
  }

  fn stop_move(&self) {
    self.ecs.lock().write_message(StartWalkEvent {
      entity: self.entity,
      direction: WalkDirection::None,
    });

    let username = self.username();

    STATES.set_mutual_states(&username, "walking", false);
    STATES.set_mutual_states(&username, "sprinting", false);
  }

  fn freeze_move(&self) {
    self.ecs.lock().write_message(StartWalkEvent {
      entity: self.entity,
      direction: WalkDirection::None,
    });

    let username = self.username();

    STATES.set_state(&username, "is_walking", false);
    STATES.set_state(&username, "is_walking", false);
    STATES.set_state(&username, "can_walking", false);
    STATES.set_state(&username, "can_sprinting", false);
  }

  fn unfreeze_move(&self) {
    let username = self.username();

    STATES.set_state(&username, "can_walking", true);
    STATES.set_state(&username, "can_sprinting", true);
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
}
