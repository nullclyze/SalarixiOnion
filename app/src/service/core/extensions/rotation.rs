use azalea::prelude::*;
use azalea::BlockPos;
use azalea::Client;
use azalea::Vec3;
use bevy_ecs::entity::Entity;
use std::time::Duration;
use tokio::time::sleep;

use crate::service::core::extensions::BotDefaultExt;
use crate::service::generators::randchance;
use crate::service::generators::randfloat;

pub trait BotRotationExt {
  fn look_at_entity(&self, entity: Entity, with_eye_height: bool);
  async fn look_at_block(&self, block_pos: BlockPos, sloppy: bool);
}

impl BotRotationExt for Client {
  fn look_at_entity(&self, entity: Entity, with_eye_height: bool) {
    let mut entity_pos = self.get_entity_position(entity);

    if with_eye_height {
      entity_pos.y += self.get_entity_eye_height(entity);
    }

    self.look_at(Vec3 {
      x: entity_pos.x,
      y: entity_pos.y,
      z: entity_pos.z,
    });
  }

  async fn look_at_block(&self, block_pos: BlockPos, sloppy: bool) {
    let center = block_pos.center();

    if sloppy && randchance(0.4) {
      let sloppy_pos = Vec3::new(
        center.x as f64 + randfloat(-1.3289, 1.3289),
        center.y as f64 + randfloat(-1.3289, 1.3289),
        center.z as f64 + randfloat(-1.3289, 1.3289),
      );

      self.look_at(sloppy_pos);

      sleep(Duration::from_millis(100)).await;
    }

    self.look_at(Vec3 {
      x: center.x,
      y: center.y,
      z: center.z,
    });
  }
}
