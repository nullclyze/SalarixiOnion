use azalea::world::MinecraftEntityId;
use azalea::prelude::*;
use azalea::protocol::packets::game::s_interact::InteractionHand;
use azalea::registry::builtin::ItemKind;
use azalea::entity::{Dead, Position, metadata::{Player, AbstractMonster}};
use azalea::ecs::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::tools::*;
use crate::common::{get_entity_position, move_item, release_use_item, start_use_item};


pub struct AutoShieldPlugin;

impl AutoShieldPlugin {
  pub fn enable(bot: Client) {
    tokio::spawn(async move {
      loop {
        if let Some(arc) = get_flow_manager() {
          if !arc.read().active {
            break;
          }
        }

        Self::defend(&bot).await;

        sleep(Duration::from_millis(50)).await;
      }
    });
  }

  pub async fn defend(bot: &Client) {
    let nickname = bot.username();

    let menu = bot.menu();

    if let Some(item) = menu.slot(45) {
      if item.is_empty() || item.kind() == ItemKind::Shield {
        if !STATES.get_state(&nickname, "is_eating") && !STATES.get_state(&nickname, "drinking") && TASKS.get_task_activity(&nickname, "bow-aim") {
          if item.kind() == ItemKind::Shield {
            Self::start_defending(bot).await;
          } else {
            for (slot, item) in menu.slots().iter().enumerate() {  
              if slot != 45 {
                if item.kind() == ItemKind::Shield {
                  move_item(bot, ItemKind::Shield, slot, 45).await;

                  sleep(Duration::from_millis(50)).await;

                  Self::start_defending(bot).await;
                }
              }
            }
          }
        }
      }
    } 
  }

  async fn start_defending(bot: &Client) {
    let eye_pos = bot.eye_position();

    let bot_id = if let Some(bot_id) = bot.get_entity_component::<MinecraftEntityId>(bot.entity) {
      bot_id
    } else {
      return;
    };

    let nearest_entity = bot.nearest_entity_by::<(&Position, &MinecraftEntityId), (With<Player>, With<AbstractMonster>, Without<Dead>)>(|data: (&Position, &MinecraftEntityId)| {
      eye_pos.distance_to(**data.0) <= 8.0 && *data.1 != bot_id
    });

    let nickname = bot.username();

    start_use_item(bot, InteractionHand::OffHand);

    if let Some(entity) = nearest_entity {
      if STATES.get_state(&nickname, "can_looking") {
        STATES.set_state(&nickname, "can_looking", false);
        STATES.set_state(&nickname, "is_looking", true);

        bot.look_at(get_entity_position(bot, entity));

        sleep(Duration::from_millis(randuint(50, 100))).await;

        for _ in 0..6 {
          bot.look_at(get_entity_position(bot, entity));
          sleep(Duration::from_millis(50)).await;
        }

        STATES.set_state(&nickname, "can_looking", true);
        STATES.set_state(&nickname, "is_looking", false);
      }
    }

    release_use_item(bot);
  }
}
