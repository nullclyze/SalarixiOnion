use azalea::prelude::*;
use azalea::protocol::packets::game::s_interact::InteractionHand;
use azalea::registry::builtin::ItemKind;
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::tools::*;
use crate::common::{EntityFilter, get_entity_position, get_nearest_entity, move_item, release_use_item, start_use_item};


pub struct AutoShieldPlugin;

impl AutoShieldPlugin {
  pub fn new() -> Self {
    Self
  }

  pub fn enable(&'static self, bot: Client) {
    tokio::spawn(async move {
      loop {
        if let Some(arc) = get_flow_manager() {
          if !arc.read().active {
            break;
          }
        }

        self.defend(&bot).await;

        sleep(Duration::from_millis(50)).await;
      }
    });
  }

  pub async fn defend(&self, bot: &Client) {
    let nickname = bot.username();

    let menu = bot.menu();

    if let Some(item) = menu.slot(45) {
      if item.is_empty() || item.kind() == ItemKind::Shield {
        if !STATES.get_state(&nickname, "is_eating") && !STATES.get_state(&nickname, "is_drinking") {
          STATES.set_state(&nickname, "can_eating", false);
          STATES.set_state(&nickname, "can_drinking", false);

          let mut shield_equipped = false;

          if item.kind() != ItemKind::Shield {
            for (slot, item) in menu.slots().iter().enumerate() {  
              if slot != 45 {
                if item.kind() == ItemKind::Shield {
                  move_item(bot, ItemKind::Shield, slot, 45).await;
                  shield_equipped = true;
                  sleep(Duration::from_millis(50)).await;

                  STATES.set_state(&nickname, "can_walking", true);
                  STATES.set_state(&nickname, "can_sprinting", true);

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

          STATES.set_state(&nickname, "can_eating", true);
          STATES.set_state(&nickname, "can_drinking", true);
        }
      }
    } 
  }

  async fn start_defending(&self, bot: &Client) {
    if let Some(entity) = get_nearest_entity(bot, EntityFilter::new(bot, "danger", 8.0)) {
      let nickname = bot.username();

      if STATES.get_state(&nickname, "can_looking") && STATES.get_state(&nickname, "can_interacting") {
        STATES.set_mutual_states(&nickname, "looking", true);
        STATES.set_mutual_states(&nickname, "interacting", true);

        start_use_item(bot, InteractionHand::OffHand);

        bot.look_at(get_entity_position(bot, entity));

        sleep(Duration::from_millis(randuint(50, 100))).await;

        for _ in 0..=randint(2, 4) {
          if let Some(e) = get_nearest_entity(bot, EntityFilter::new(bot, "danger", 8.0)) {
            bot.look_at(get_entity_position(bot, e));
            sleep(Duration::from_millis(50)).await;
          }
        }

        release_use_item(bot);

        STATES.set_mutual_states(&nickname, "looking", false);
        STATES.set_mutual_states(&nickname, "interacting", false);
      }
    }
  }
}
