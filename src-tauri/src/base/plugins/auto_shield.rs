use azalea::ecs::entity::Entity;
use azalea::prelude::*;
use azalea::protocol::packets::game::s_interact::InteractionHand;
use azalea::registry::builtin::ItemKind;
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::common::*;
use crate::common::{
  get_entity_position, get_inventory_menu, get_nearest_entity, inventory_move_item,
  release_use_item, start_use_item, EntityFilter,
};

pub struct AutoShieldPlugin;

impl AutoShieldPlugin {
  pub fn new() -> Self {
    Self
  }

  pub fn enable(&'static self, username: String) {
    tokio::spawn(async move {
      loop {
        if let Some(arc) = get_flow_manager() {
          if !arc.read().active {
            break;
          }
        }

        let _ = BOT_REGISTRY
          .get_bot(&username, async |bot| {
            self.defend(&bot).await;
          })
          .await;

        sleep(Duration::from_millis(50)).await;
      }
    });
  }

  pub async fn defend(&self, bot: &Client) {
    let nickname = bot.username();

    if let Some(menu) = get_inventory_menu(bot) {
      if let Some(item) = menu.slot(45) {
        if item.is_empty() || item.kind() == ItemKind::Shield {
          if !STATES.get_state(&nickname, "is_eating")
            && !STATES.get_state(&nickname, "is_drinking")
          {
            let mut shield_equipped = false;

            if item.kind() != ItemKind::Shield {
              for (slot, item) in menu.slots().iter().enumerate() {
                if slot != 45 {
                  if item.kind() == ItemKind::Shield {
                    inventory_move_item(bot, ItemKind::Shield, slot, 45, true).await;
                    shield_equipped = true;
                    break;
                  }
                }
              }
            } else {
              shield_equipped = true;
            }

            if shield_equipped {
              self.start_defending(bot).await;
            }
          }
        }
      }
    }
  }

  fn get_nearest_dangerous_entity(&self, bot: &Client) -> Option<Entity> {
    let mut nearest_entity = None;

    if let Some(nearest_player) = get_nearest_entity(bot, EntityFilter::new(bot, "player", 8.0)) {
      nearest_entity = Some(nearest_player);
    } else {
      if let Some(nearest_monster) = get_nearest_entity(bot, EntityFilter::new(bot, "monster", 8.0))
      {
        nearest_entity = Some(nearest_monster);
      }
    }

    nearest_entity
  }

  async fn start_defending(&self, bot: &Client) {
    if let Some(entity) = self.get_nearest_dangerous_entity(bot) {
      let nickname = bot.username();

      if STATES.get_state(&nickname, "can_looking")
        && STATES.get_state(&nickname, "can_interacting")
      {
        STATES.set_mutual_states(&nickname, "looking", true);
        STATES.set_mutual_states(&nickname, "interacting", true);

        start_use_item(bot, InteractionHand::OffHand);

        sleep(Duration::from_millis(50)).await;

        bot.look_at(get_entity_position(bot, entity));

        sleep(Duration::from_millis(randuint(50, 100))).await;

        for _ in 0..=randint(2, 4) {
          if let Some(e) = self.get_nearest_dangerous_entity(bot) {
            bot.look_at(get_entity_position(bot, e));
          }

          sleep(Duration::from_millis(randuint(50, 100))).await;
        }

        release_use_item(bot);

        STATES.set_mutual_states(&nickname, "looking", false);
        STATES.set_mutual_states(&nickname, "interacting", false);
      }
    }
  }
}
