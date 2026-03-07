use azalea::ecs::entity::Entity;
use azalea::prelude::*;
use azalea::protocol::packets::game::s_interact::InteractionHand;
use azalea::registry::builtin::ItemKind;
use std::time::Duration;
use tokio::time::sleep;

use crate::core::*;
use crate::generators::*;
use crate::extensions::{BotDefaultExt, BotInteractExt, BotInventoryExt, EntityType};

pub struct AutoShieldPlugin;

impl AutoShieldPlugin {
  pub fn new() -> Self {
    Self
  }

  pub fn enable(&'static self, username: String) {
    let nickname = username.clone();

    let task = tokio::spawn(async move {
      loop {
        if !process_is_active() {
          break;
        }

        let _ = BOT_REGISTRY
          .async_get_bot(&nickname, async |bot| {
            if !bot.workable() {
              return;
            }

            self.defend(&bot).await;
          })
          .await;

        sleep(Duration::from_millis(50)).await;
      }
    });

    PLUGIN_MANAGER.push_task(&username, "auto-shield", task);
  }

  pub async fn defend(&self, bot: &Client) {
    let nickname = bot.name();

    if let Some(menu) = bot.get_inventory_menu() {
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
                    bot
                      .inventory_move_item(ItemKind::Shield, slot, 45, true)
                      .await;
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

    if let Some(nearest_player) = bot.find_nearest_entity(EntityType::Player, 8.0) {
      nearest_entity = Some(nearest_player);
    } else {
      if let Some(nearest_monster) = bot.find_nearest_entity(EntityType::Monster, 8.0)
      {
        nearest_entity = Some(nearest_monster);
      }
    }

    nearest_entity
  }

  async fn start_defending(&self, bot: &Client) {
    if let Some(entity) = self.get_nearest_dangerous_entity(bot) {
      let nickname = bot.name();

      if STATES.get_state(&nickname, "can_looking")
        && STATES.get_state(&nickname, "can_interacting")
      {
        STATES.set_mutual_states(&nickname, "looking", true);
        STATES.set_mutual_states(&nickname, "interacting", true);

        bot.start_use_held_item(InteractionHand::OffHand);

        sleep(Duration::from_millis(50)).await;

        bot.look_at(bot.get_entity_position(entity));

        sleep(Duration::from_millis(randuint(50, 100))).await;

        for _ in 0..=randint(2, 4) {
          if let Some(e) = self.get_nearest_dangerous_entity(bot) {
            bot.look_at(bot.get_entity_position(e));
          }

          sleep(Duration::from_millis(randuint(50, 100))).await;
        }

        bot.release_use_held_item();

        STATES.set_mutual_states(&nickname, "looking", false);
        STATES.set_mutual_states(&nickname, "interacting", false);
      }
    }
  }
}
