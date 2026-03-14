use azalea::entity::{Physics, Position};
use azalea::{Client, Vec3};

pub trait BotPhysicsExt {
  fn get_physics(&self) -> Option<Physics>;
  fn set_velocity(&self, axis: &str, velocity: f64);
  fn set_on_ground(&self, on_ground: bool);
  fn set_old_position(&self, x: f64, y: f64, z: f64);
  fn set_no_jump_delay(&self, delay: u32);
  fn set_was_touching_water(&self, state: bool);
  fn set_has_impulse(&self, state: bool);
}

impl BotPhysicsExt for Client {
  fn get_physics(&self) -> Option<Physics> {
    if let Some(physics) = self.get_component::<Physics>() {
      return Some(physics);
    }

    None
  }

  fn set_velocity(&self, axis: &str, velocity: f64) {
    let mut ecs = self.ecs.lock();

    if let Some(mut physics) = ecs.get_mut::<Physics>(self.entity) {
      match axis {
        "x" => {
          physics.velocity.x = velocity;
        }
        "y" => {
          physics.velocity.y = velocity;
        }
        "z" => {
          physics.velocity.z = velocity;
        }
        _ => {}
      }
    }
  }

  fn set_on_ground(&self, on_ground: bool) {
    let mut ecs = self.ecs.lock();

    if let Some(mut physics) = ecs.get_mut::<Physics>(self.entity) {
      physics.set_on_ground(on_ground);
    }
  }

  fn set_old_position(&self, x: f64, y: f64, z: f64) {
    let mut ecs = self.ecs.lock();

    if let Some(mut physics) = ecs.get_mut::<Physics>(self.entity) {
      let pos = Vec3::new(x, y, z);
      physics.set_old_pos(Position::new(pos));
    }
  }

  fn set_no_jump_delay(&self, delay: u32) {
    let mut ecs = self.ecs.lock();

    if let Some(mut physics) = ecs.get_mut::<Physics>(self.entity) {
      physics.no_jump_delay = delay;
    }
  }

  fn set_was_touching_water(&self, state: bool) {
    let mut ecs = self.ecs.lock();

    if let Some(mut physics) = ecs.get_mut::<Physics>(self.entity) {
      physics.was_touching_water = state;
    }
  }

  fn set_has_impulse(&self, state: bool) {
    let mut ecs = self.ecs.lock();

    if let Some(mut physics) = ecs.get_mut::<Physics>(self.entity) {
      physics.has_impulse = state;
    }
  }
}
