use azalea::prelude::*;
use azalea::Vec3;
use azalea::ecs::prelude::*;
use azalea::world::MinecraftEntityId;
use azalea::entity::{Dead, Position};
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::tools::*;


pub struct AutoLookPlugin;

impl AutoLookPlugin {
  pub fn enable(bot: Client) {
    tokio::spawn(async move {
      loop {
        if let Some(arc) = get_flow_manager() {
          if !arc.read().active {
            break;
          }
        }

        Self::look(&bot).await;

        sleep(Duration::from_millis(50)).await;
      }
    });
  } 

  async fn look(bot: &Client) {
    let eye_pos = bot.eye_position();

    let bot_id = if let Some(bot_id) = bot.get_entity_component::<MinecraftEntityId>(bot.entity) {
      bot_id
    } else {
      return;
    };

    let nearest_entity = bot.nearest_entity_by::<(&Position, &MinecraftEntityId), Without<Dead>>(|data: (&Position, &MinecraftEntityId)| {
      eye_pos.distance_to(**data.0) <= 14.0 && *data.1 != bot_id
    });

    if let Some(entity) = nearest_entity {
      if let Some(entity_pos) = bot.get_entity_component::<Position>(entity) {
        let nickname = bot.username();

        if bot.is_goto_target_reached() {
          if STATES.get_state(&nickname, "can_looking") && !TASKS.get_task_activity(&nickname, "bow-aim") && !TASKS.get_task_activity(&nickname, "killaura") && !TASKS.get_task_activity(&nickname, "scaffold") && !TASKS.get_task_activity(&nickname, "miner") && !TASKS.get_task_activity(&nickname, "farmer") {
            STATES.set_state(&nickname, "is_looking", true);

            let pos = Vec3::new(
              entity_pos.x + randfloat(-0.1, 0.1), 
              entity_pos.y + randfloat(-0.1, 0.1), 
              entity_pos.z + randfloat(-0.1, 0.1)
            );

            bot.look_at(pos);

            sleep(Duration::from_millis(randuint(50, 100))).await;

            STATES.set_state(&nickname, "is_looking", false);
          }
        }
      }
    }
  }
}
