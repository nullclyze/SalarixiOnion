use azalea::entity::Physics;
use azalea::Client;

pub trait BotPhysicsExt {
  fn get_physics(&self) -> Option<Physics>;
  fn set_velocity_y(&self, velocity_y: f64);
  fn set_on_ground(&self, on_ground: bool);
}

impl BotPhysicsExt for Client {
  fn get_physics(&self) -> Option<Physics> {
    if let Some(physics) = self.get_component::<Physics>() {
      return Some(physics);
    }

    None
  }

  fn set_velocity_y(&self, velocity_y: f64) {
    let mut ecs = self.ecs.lock();

    if let Some(mut physics) = ecs.get_mut::<Physics>(self.entity) {
      physics.velocity.y = velocity_y;
    }
  }

  fn set_on_ground(&self, on_ground: bool) {
    let mut ecs = self.ecs.lock();

    if let Some(mut physics) = ecs.get_mut::<Physics>(self.entity) {
      physics.set_on_ground(on_ground);
    }
  }
}
