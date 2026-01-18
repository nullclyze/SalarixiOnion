use azalea::prelude::*;
use azalea::Vec3;
use azalea::core::position::BlockPos;
use azalea::entity::{Position, metadata::Player, LocalEntity};
use azalea::player::GameProfileComponent;  
use azalea::protocol::packets::game::ServerboundPlayerAction;
use azalea::protocol::packets::game::s_player_action::Action;
use azalea::packet::game::SendGamePacketEvent;
use azalea::core::direction::Direction;
use bevy_ecs::query::{With, Without};
use serde::{Serialize, Deserialize};

use crate::TASKS;
use crate::tools::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BowAimModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BowAimOptions {
  pub target: String,
  pub nickname: Option<String>,
  pub slot: Option<u8>,
  pub delay: Option<usize>,
  pub state: bool
}

impl BowAimModule {
  async fn shoot(bot: &Client, slot: Option<u8>, target_pos: Vec3) {
    bot.set_selected_hotbar_slot(slot.unwrap_or_else(|| { 0 }));

    bot.start_use_item();
      
    bot.wait_ticks(randuint(24, 28) as usize).await;

    let direction = bot.direction();

    bot.set_direction(direction.0 + randfloat(-1.5, 1.5) as f32, direction.1 + (target_pos.y * 0.2) as f32);

    bot.ecs.lock().trigger(SendGamePacketEvent::new(  
      bot.entity,  
      ServerboundPlayerAction {  
        action: Action::ReleaseUseItem,  
        pos: BlockPos::new(0, 0, 0),  
        direction: Direction::Down,  
        seq: 0
      }
    ));
  }

  async fn aiming(bot: &Client, options: BowAimOptions) {
    let mut counter = 0;

    loop {
      match options.target.as_str() {
        "nearest-player" => {
          let nearest_player = bot.nearest_entity_by::<(), (With<Player>, Without<LocalEntity>)>(|_: ()| true);
        
          if let Some(entity) = nearest_player {
            counter += 1;

            let position = bot.entity_component::<Position>(entity);
            let pos = Vec3::new(position.x, position.y, position.z);
            
            bot.look_at(Vec3::new(pos.x + randfloat(-1.5, 1.5), pos.y, pos.z + randfloat(-1.5, 1.5)));

            if counter >= 4 {
              Self::shoot(bot, options.slot, pos).await;
              counter = 0;
            }
          }
        },
        "custom-goal" => {
          if let Some(nickname) = options.nickname.clone() {
            let nearest_player = bot.nearest_entity_by::<&GameProfileComponent, (With<Player>, Without<LocalEntity>)>(|profile: &GameProfileComponent| profile.name == nickname);

            if let Some(entity) = nearest_player {
              counter += 1;

              let position = bot.entity_component::<Position>(entity);
              let pos = Vec3::new(position.x, position.y, position.z);
              
              bot.look_at(Vec3::new(pos.x + randfloat(-1.5, 1.5), pos.y, pos.z + randfloat(-1.5, 1.5)));

              if counter >= 4 {
                Self::shoot(bot, options.slot, pos).await;
                counter = 0;
              }
            }
          }
        },
        _ => {}
      }

      bot.wait_ticks(options.delay.unwrap_or_else(|| { 2 })).await;
    }
  }

  pub async fn enable(bot: &Client, options: BowAimOptions) {
    Self::aiming(bot, options).await;
  }

  pub fn stop(nickname: &String) {
    TASKS.get(nickname).unwrap().write().unwrap().stop_task("bow-aim");
  }
}
