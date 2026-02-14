use azalea::{
  entity::{dimensions::EntityDimensions, metadata::Health, Position},
  local_player::Hunger,
  protocol::packets::game::{s_interact::InteractionHand, ServerboundSwing},
  world::MinecraftEntityId,
  Client, Vec3,
};

// Трейт безопасных функций для Client
pub trait SafeClientImpls {
  fn workable(&self) -> bool;
  fn id(&self) -> MinecraftEntityId;
  fn feet_pos(&self) -> Vec3;
  fn eye_pos(&self) -> Vec3;
  fn get_health(&self) -> f32;
  fn get_satiety(&self) -> u32;
  fn swing_arm(&self);
}

impl SafeClientImpls for Client {
  fn workable(&self) -> bool {
    let position_exists = self.get_component::<Position>().is_some();
    let dimensions_exists = self.get_component::<EntityDimensions>().is_some();

    position_exists && dimensions_exists
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

  fn swing_arm(&self) {
    self.write_packet(ServerboundSwing {
      hand: InteractionHand::MainHand,
    });
  }
}
