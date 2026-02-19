use std::time::Duration;

use azalea::BlockPos;
use azalea::prelude::*;
use azalea::Vec3;
use bevy_ecs::entity::Entity;
use tokio::time::sleep;

use crate::generators::randchance;
use crate::generators::randfloat;

use super::auxiliary::{get_entity_eye_height, get_entity_position};

/// Функция поворота на сущность
pub fn look_at_entity(bot: &Client, entity: Entity, with_eye_height: bool) {
  let mut entity_pos = get_entity_position(bot, entity);

  if with_eye_height {
    entity_pos.y += get_entity_eye_height(bot, entity);
  }

  bot.look_at(Vec3 {
    x: entity_pos.x,
    y: entity_pos.y,
    z: entity_pos.z,
  });
}


/// Функция поворота на блок
pub async fn look_at_block(bot: &Client, block_pos: BlockPos, sloppy: bool) {
  let center = block_pos.center();

  if sloppy && randchance(0.4) {
    let sloppy_pos = Vec3::new(
      center.x as f64 + randfloat(-1.3289, 1.3289),
      center.y as f64 + randfloat(-1.3289, 1.3289),
      center.z as f64 + randfloat(-1.3289, 1.3289),
    );

    bot.look_at(sloppy_pos);

    sleep(Duration::from_millis(100)).await;
  }

  bot.look_at(Vec3 { 
    x: center.x, 
    y: center.y, 
    z: center.z 
  });
}
