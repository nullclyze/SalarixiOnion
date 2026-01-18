use azalea::prelude::*;
use azalea::core::position::BlockPos;
use azalea::interact::SwingArmEvent;  
use azalea::entity::Physics;
use serde::{Serialize, Deserialize};

use crate::TASKS;
use crate::tools::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaffoldModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaffoldOptions {
  pub mode: String,
  pub delay: Option<usize>,
  pub min_degree_of_gaze: Option<f32>,
  pub max_degree_of_gaze: Option<f32>,
  pub state: bool
}

impl ScaffoldModule {
  fn simulate_inaccuracy(bot: &Client, direction: (f32, f32)) {
    let inaccurate_direction = (direction.0 + randfloat(-0.08, 0.08) as f32, direction.1 + randfloat(-0.08, 0.08) as f32);

    bot.set_direction(inaccurate_direction.0, inaccurate_direction.1);
  }

  fn direct_gaze(bot: &Client, min_x_rot: Option<f32>, max_x_rot: Option<f32>) {
    let direction = bot.direction();

    let min = if let Some(rot) = min_x_rot { rot } else { 70.0 } as f64;
    let max = if let Some(rot) = max_x_rot { rot } else { 85.0 } as f64;

    bot.set_direction(direction.0, randfloat(min, max) as f32); 
  }

  async fn ninja_bridge_scaffold(bot: &Client, options: ScaffoldOptions) {
    loop { 
      Self::direct_gaze(bot, options.min_degree_of_gaze, options.max_degree_of_gaze);

      let position = bot.position();
      let block_under = BlockPos::new(position.x.floor() as i32, (position.y - 0.5).floor() as i32 , position.z.floor() as i32);

      let is_air = {
        bot.world().read().get_block_state(block_under).map_or(true, |state| state.is_air())
      };

      if is_air {
        bot.set_crouching(true);

        {
          let mut ecs = bot.ecs.lock(); 
          let mut physics = ecs.get_mut::<Physics>(bot.entity).unwrap();  
          physics.set_on_ground(true); 
        }

        bot.wait_ticks(randuint(4, 6) as usize).await;
                      
        bot.ecs.lock().trigger(SwingArmEvent { entity: bot.entity });  
        bot.block_interact(block_under);  

        bot.set_crouching(false);

        Self::simulate_inaccuracy(bot, bot.direction());

        bot.wait_ticks(randuint(1, 2) as usize).await;

        {
          let mut ecs = bot.ecs.lock(); 
          let mut physics = ecs.get_mut::<Physics>(bot.entity).unwrap();  
          physics.set_on_ground(false); 
        }

        bot.wait_ticks(randuint(1, 2) as usize).await;
      }
              
      bot.wait_ticks(options.delay.unwrap_or_else(|| { 1 })).await;
    }    
  }

  async fn god_bridge_scaffold(bot: &Client, options: ScaffoldOptions) {
    loop { 
      Self::direct_gaze(bot, options.min_degree_of_gaze, options.max_degree_of_gaze);

      let position = bot.position();
      let block_under = BlockPos::new(position.x.floor() as i32, (position.y - 0.5).floor() as i32 , position.z.floor() as i32);

      let is_air = {
        bot.world().read().get_block_state(block_under).map_or(true, |state| state.is_air())
      };

      if is_air {
        {
          let mut ecs = bot.ecs.lock(); 
          let mut physics = ecs.get_mut::<Physics>(bot.entity).unwrap();  
          physics.set_on_ground(true); 
        }

        bot.wait_ticks(randuint(1, 2) as usize).await;
                      
        bot.ecs.lock().trigger(SwingArmEvent { entity: bot.entity });  
        bot.block_interact(block_under);  

        Self::simulate_inaccuracy(bot, bot.direction());

        {
          let mut ecs = bot.ecs.lock(); 
          let mut physics = ecs.get_mut::<Physics>(bot.entity).unwrap();  
          physics.set_on_ground(false); 
        }

        bot.wait_ticks(randuint(1, 2) as usize).await;
      }
              
      bot.wait_ticks(options.delay.unwrap_or_else(|| { 1 })).await;
    }    
  }

  async fn jump_bridge_scaffold(bot: &Client, options: ScaffoldOptions) {
    loop { 
      Self::direct_gaze(bot, options.min_degree_of_gaze, options.max_degree_of_gaze);

      let position = bot.position();
      let velocity = bot.ecs.lock().get::<Physics>(bot.entity).unwrap().clone().velocity;

      let block_under = BlockPos::new(
        position.x.floor() as i32, 
        (if velocity.y != 0.0 { position.y - 1.0 } else { position.y - 0.5 }).floor() as i32,
        position.z.floor() as i32
      );

      let is_air = {
        bot.world().read().get_block_state(block_under).map_or(true, |state| state.is_air())
      };
              
      if is_air {  
        {
          let mut ecs = bot.ecs.lock(); 
          let mut physics = ecs.get_mut::<Physics>(bot.entity).unwrap();  
          physics.set_on_ground(true); 
        }

        bot.jump();
                      
        bot.ecs.lock().trigger(SwingArmEvent { entity: bot.entity });  
        bot.block_interact(block_under);  

        Self::simulate_inaccuracy(bot, bot.direction());

        {
          let mut ecs = bot.ecs.lock(); 
          let mut physics = ecs.get_mut::<Physics>(bot.entity).unwrap();  
          physics.set_on_ground(false); 
        }

        bot.wait_ticks(randuint(1, 2) as usize).await;
      }  
              
      bot.wait_ticks(options.delay.unwrap_or_else(|| { 1 })).await;
    }    
  }

  pub async fn enable(bot: &Client, options: ScaffoldOptions) {
    match options.mode.as_str() {
      "ninja-bridge" => { Self::ninja_bridge_scaffold(bot, options).await; },
      "god-bridge" => { Self::god_bridge_scaffold(bot, options).await; },
      "jump-bridge" => { Self::jump_bridge_scaffold(bot, options).await; }
      _ => {}
    }
  } 

  pub fn stop(nickname: &String) {
    TASKS.get(nickname).unwrap().write().unwrap().stop_task("scaffold");
  }
}