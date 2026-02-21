use azalea::{
  entity::{
    dimensions::EntityDimensions, metadata::Health, Crouching, Jumping, LookDirection, Position,
  },
  local_player::{Hunger, TabList},
  player::GameProfileComponent,
  protocol::packets::game::{s_interact::InteractionHand, ServerboundSwing},
  world::MinecraftEntityId,
  Client, InGameState, Vec3,
};

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
}
