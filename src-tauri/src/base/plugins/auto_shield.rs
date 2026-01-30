use azalea::world::MinecraftEntityId;
use azalea::{BlockPos, WalkDirection, prelude::*};
use azalea::prelude::ContainerClientExt;
use azalea::protocol::packets::game::s_player_action::Action;
use azalea::protocol::packets::game::{ServerboundPlayerAction, ServerboundUseItem};
use azalea::protocol::packets::game::s_interact::InteractionHand;
use azalea::registry::builtin::ItemKind;
use azalea::entity::{Dead, Position, metadata::{Player, AbstractMonster}};
use azalea::ecs::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

use crate::base::get_flow_manager;
use crate::state::STATES;
use crate::tasks::TASKS;
use crate::tools::randticks;
use crate::common::get_entity_position;


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
        if !STATES.get_plugin_activity(&nickname, "auto-eat") && !STATES.get_plugin_activity(&nickname, "auto-potion") && !STATES.get_plugin_activity(&nickname, "auto-totem") && TASKS.get_task_activity(&nickname, "bow-aim") {
          if item.kind() == ItemKind::Shield {
            Self::start_defending(bot).await;
          } else {
            for (slot, item) in menu.slots().iter().enumerate() {  
              if slot != 45 {
                if item.kind() == ItemKind::Shield {
                  STATES.set_plugin_activity(&nickname, "auto-shield", true);

                  let inventory = bot.get_inventory();

                  bot.walk(WalkDirection::None);

                  inventory.left_click(slot);
                  bot.wait_ticks(1).await;
                  inventory.left_click(45 as usize);

                  bot.wait_ticks(1).await;

                  Self::start_defending(bot).await;

                  STATES.set_plugin_activity(&nickname, "auto-shield", false);
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

    let direction = bot.direction();

    bot.write_packet(ServerboundUseItem {
      hand: InteractionHand::OffHand,
      seq: 0,
      y_rot: direction.0,
      x_rot: direction.1
    });

    if let Some(entity) = nearest_entity {
      bot.look_at(get_entity_position(bot, entity));

      bot.wait_ticks(randticks(1, 2)).await;

      for _ in 0..6 {
        bot.look_at(get_entity_position(bot, entity));
        bot.wait_ticks(2).await;
      }
    }

    bot.write_packet(ServerboundPlayerAction {
      action: Action::ReleaseUseItem,
      pos: BlockPos::new(0, 0, 0),
      direction: azalea::core::direction::Direction::Down,
      seq: 0
    });
  }
}
