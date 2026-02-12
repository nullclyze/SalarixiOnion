use azalea::entity::metadata::Health;
use azalea::inventory::components::PotionContents;
use azalea::inventory::ItemStack;
use azalea::prelude::*;
use azalea::protocol::packets::game::s_interact::InteractionHand;
use azalea::registry::builtin::ItemKind;
use azalea::registry::builtin::Potion as PotionKind;
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::common::{get_bot_physics, get_health, get_inventory_menu, start_use_item, take_item};
use crate::tools::*;

#[derive(Clone)]
struct Potion {
  kind: String,
  slot: usize,
  name: PotionKind,
}

pub struct AutoPotionPlugin;

impl AutoPotionPlugin {
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

        self.drink(&bot).await;

        sleep(Duration::from_millis(50)).await;
      }
    });
  }

  async fn drink(&self, bot: &Client) {
    let health = get_health(bot);

    let nickname = bot.username();

    if health < 20 {
      let potions = self.find_potion_in_inventory(bot);

      if potions.len() > 0 {
        if let Some(potion) = self.get_best_potion(bot, potions) {
          if health < 10 && !STATES.get_state(&nickname, "is_eating") {
            STATES.set_state(&nickname, "can_eating", false);
          } else {
            STATES.set_state(&nickname, "can_eating", true);
          }

          if STATES.get_state(&nickname, "can_drinking")
            && !STATES.get_state(&nickname, "is_eating")
            && !STATES.get_state(&nickname, "is_interacting")
          {
            let mut should_drink = true;

            if STATES.get_state(&nickname, "is_attacking") {
              should_drink = !randchance(health as f64 / 20.0);
            }

            if should_drink {
              STATES.set_state(&nickname, "can_eating", false);
              STATES.set_state(&nickname, "can_attacking", false);
              STATES.set_state(&nickname, "can_interacting", false);
              STATES.set_mutual_states(&nickname, "drinking", true);

              take_item(bot, potion.slot).await;
              sleep(Duration::from_millis(50)).await;
              self.use_potion(bot, potion.kind).await;
              sleep(Duration::from_millis(50)).await;

              STATES.set_state(&nickname, "can_eating", true);
              STATES.set_state(&nickname, "can_interacting", true);
              STATES.set_state(&nickname, "can_walking", true);
              STATES.set_state(&nickname, "can_sprinting", true);
              STATES.set_state(&nickname, "can_attacking", true);
              STATES.set_mutual_states(&nickname, "drinking", false);
            }
          }
        }
      }
    }
  }

  async fn use_potion(&self, bot: &Client, kind: String) {
    match kind.as_str() {
      "default" => {
        start_use_item(bot, InteractionHand::MainHand);
        sleep(Duration::from_millis(2600)).await;
      }
      "splash" => {
        let direction = bot.direction();

        bot.set_direction(
          direction.0 + randfloat(-5.5, 5.5) as f32,
          randfloat(87.0, 90.0) as f32,
        );

        sleep(Duration::from_millis(randuint(400, 600))).await;
        start_use_item(bot, InteractionHand::MainHand);
        sleep(Duration::from_millis(randuint(300, 500))).await;

        bot.set_direction(
          direction.0 + randfloat(-2.5, 2.5) as f32,
          direction.1 + randfloat(-2.5, 2.5) as f32,
        );
      }
      _ => {}
    }
  }

  fn get_best_potion(&self, bot: &Client, potions: Vec<Potion>) -> Option<Potion> {
    let health = if let Some(health) = bot.get_component::<Health>() {
      health
    } else {
      return None;
    };

    if let Some(physics) = get_bot_physics(bot) {
      let is_in_lava = physics.is_in_lava();
      let is_burning = physics.remaining_fire_ticks > 0;
      let is_in_water = physics.is_in_water();
      let on_ground = physics.on_ground();
      let velocity_y = physics.velocity.y;

      let mut best_potion = None;

      for p in potions {
        if best_potion
          .as_ref()
          .unwrap_or(&Potion {
            kind: "deafult".to_string(),
            slot: 0,
            name: PotionKind::Awkward,
          })
          .kind
          .as_str()
          != "splash"
        {
          if best_potion.is_none() && is_in_lava || is_burning {
            match p.name {
              PotionKind::FireResistance => {
                best_potion = Some(p.clone());
              }
              PotionKind::LongFireResistance => {
                best_potion = Some(p.clone());
              }
              _ => {}
            }
          }

          if best_potion.is_none() && is_in_water {
            match p.name {
              PotionKind::WaterBreathing => {
                best_potion = Some(p.clone());
              }
              PotionKind::LongWaterBreathing => {
                best_potion = Some(p.clone());
              }
              _ => {}
            }
          }

          if best_potion.is_none() && !on_ground && velocity_y < -0.5 {
            match p.name {
              PotionKind::SlowFalling => {
                best_potion = Some(p.clone());
              }
              PotionKind::LongSlowFalling => {
                best_potion = Some(p.clone());
              }
              _ => {}
            }
          }

          if best_potion.is_none() && health.0 <= 8.0 {
            match p.name {
              PotionKind::TurtleMaster => {
                best_potion = Some(p.clone());
              }
              PotionKind::LongTurtleMaster => {
                best_potion = Some(p.clone());
              }
              PotionKind::StrongTurtleMaster => {
                best_potion = Some(p.clone());
              }
              _ => {}
            }
          }

          if best_potion.is_none() && health.0 <= 15.0 {
            match p.name {
              PotionKind::Regeneration => {
                best_potion = Some(p);
                break;
              }
              PotionKind::LongRegeneration => {
                best_potion = Some(p);
                break;
              }
              PotionKind::StrongRegeneration => {
                best_potion = Some(p);
                break;
              }
              PotionKind::Healing => {
                best_potion = Some(p);
                break;
              }
              PotionKind::StrongHealing => {
                best_potion = Some(p);
                break;
              }
              _ => {}
            }
          }

          if p.kind.as_str() == "splash" {
            break;
          }
        }
      }

      return best_potion;
    }

    None
  }

  fn find_potion_in_inventory(&self, bot: &Client) -> Vec<Potion> {
    let mut potion_list = vec![];

    if let Some(menu) = get_inventory_menu(bot) {
      for (slot, item) in menu.slots().iter().enumerate() {
        if let Some(potion) = self.is_potion(slot, item) {
          potion_list.push(potion);
        }
      }
    }

    potion_list
  }

  fn is_potion(&self, slot: usize, item: &ItemStack) -> Option<Potion> {
    match item.kind() {
      ItemKind::Potion => {
        if let Some(contents) = item.get_component::<PotionContents>() {
          if let Some(potion) = contents.potion {
            return Some(Potion {
              kind: "default".to_string(),
              slot: slot,
              name: potion,
            });
          }
        }
      }
      ItemKind::SplashPotion => {
        if let Some(contents) = item.get_component::<PotionContents>() {
          if let Some(potion) = contents.potion {
            return Some(Potion {
              kind: "splash".to_string(),
              slot: slot,
              name: potion,
            });
          }
        }
      }
      _ => {}
    }

    None
  }
}
