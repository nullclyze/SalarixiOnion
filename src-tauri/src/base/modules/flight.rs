use azalea::prelude::*;
use azalea::Vec3;
use azalea::core::position::BlockPos; 
use azalea::entity::Physics;
use azalea::protocol::common::movements::MoveFlags;
use azalea::protocol::packets::game::ServerboundMovePlayerPos;
use serde::{Serialize, Deserialize};
use std::time::{Duration, Instant};
use tokio::time::sleep;

use crate::TASKS;
use crate::tools::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlightModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlightOptions {
  pub mode: String,
  pub settings: String,
  pub anti_cheat: String,
  pub min_delay: Option<u64>,
  pub max_delay: Option<u64>,
  pub min_change_y: Option<f64>,
  pub max_change_y: Option<f64>,
  pub use_ground_spoof: Option<String>,
  pub state: bool
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct FlightConfig {
  min_delay: u64,
  max_delay: u64,
  min_change_y: f64,
  max_change_y: f64,
  use_ground_spoof: bool,
  use_jitter: bool
}

impl FlightModule {
  fn create_adaptive_config(anti_cheat: String) -> FlightConfig {
    let config;

    match anti_cheat.as_str() {
      "vulcan" => {
        config = FlightConfig {
          min_delay: 2,
          max_delay: 5,
          min_change_y: 0.004,
          max_change_y: 0.007,
          use_ground_spoof: true,
          use_jitter: true
        };
      },
      "intave" => {
        config = FlightConfig {
          min_delay: 4,
          max_delay: 8,
          min_change_y: 0.0025,
          max_change_y: 0.0048,
          use_ground_spoof: true,
          use_jitter: true
        };
      },
      "grim" => {
        config = FlightConfig {
          min_delay: 2,
          max_delay: 4,
          min_change_y: 0.0011,
          max_change_y: 0.0018,
          use_ground_spoof: false,
          use_jitter: false
        };
      },
      _ => {
        config = FlightConfig {
          min_delay: 6,
          max_delay: 10,
          min_change_y: 0.065,
          max_change_y: 0.087,
          use_ground_spoof: true,
          use_jitter: true
        };
      }
    }

    config
  }

  fn get_physics(bot: &Client) -> Physics {
    let mut ecs = bot.ecs.lock(); 
    ecs.get_mut::<Physics>(bot.entity).unwrap().clone()
  }

  fn set_velocity_y(bot: &Client, velocity_y: f64) {
    let mut ecs = bot.ecs.lock(); 
    let mut physics = ecs.get_mut::<Physics>(bot.entity).unwrap();
           
    physics.velocity.y = velocity_y;
  }

  fn set_on_ground(bot: &Client, on_ground: bool) {
    let mut ecs = bot.ecs.lock(); 
    let mut physics = ecs.get_mut::<Physics>(bot.entity).unwrap();
           
    physics.set_on_ground(on_ground);
  }

  async fn hover(bot: &Client, time: Instant) {
    loop {
      if Instant::now() >= time {
        break;
      }

      Self::set_velocity_y(bot, 0.0);

      bot.wait_ticks(1).await;
    }
  }

  pub async fn vanilla_flight(bot: &Client, options: FlightOptions) {
    let config = if options.settings.as_str() == "adaptive" {
      Self::create_adaptive_config(options.anti_cheat)
    } else {
      FlightConfig {
        min_delay: options.min_delay.unwrap_or(4),
        max_delay: options.max_delay.unwrap_or(8),
        min_change_y: options.min_change_y.unwrap_or(0.05),
        max_change_y: options.max_change_y.unwrap_or(0.08),
        use_ground_spoof: if let Some(v) = options.use_ground_spoof { v.as_str() == "true" } else { true },
        use_jitter: false
      }
    };

    loop {
      if config.use_ground_spoof {
        Self::set_on_ground(bot, true);
      }

      sleep(Duration::from_millis(randuint(50, 80))).await;

      if config.use_jitter {
        for _ in 0..randint(4, 6) {
          Self::set_velocity_y(bot, randfloat(config.min_change_y, config.max_change_y));
          bot.wait_ticks(1).await;
        }
      } else {
        Self::set_velocity_y(bot, randfloat(config.min_change_y, config.max_change_y));
      }

      if config.use_ground_spoof {
        Self::set_on_ground(bot, false);
      }

      bot.wait_ticks(randuint(config.min_delay, config.max_delay) as usize).await;

      Self::hover(bot, Instant::now() + Duration::from_millis(randuint(100, 150))).await;

      if config.use_ground_spoof {
        Self::set_on_ground(bot, false);
      }
    }
  }

  pub async fn jump_flight(bot: &Client, options: FlightOptions) {
    let config = if options.settings.as_str() == "adaptive" {
      Self::create_adaptive_config(options.anti_cheat)
    } else {
      FlightConfig {
        min_delay: options.min_delay.unwrap_or(4),
        max_delay: options.max_delay.unwrap_or(7),
        min_change_y: options.min_change_y.unwrap_or(0.006),
        max_change_y: options.max_change_y.unwrap_or(0.008),
        use_ground_spoof: if let Some(v) = options.use_ground_spoof { v.as_str() == "true" } else { true },
        use_jitter: true
      }
    };

    let mut counter = 0;

    loop {
      counter += 1;

      if config.use_ground_spoof {
        Self::set_on_ground(bot, true);
      }

      if counter == 2 {
        bot.jump();
        counter = 0;
      }

      sleep(Duration::from_millis(randuint(50, 80))).await;

      let pos = bot.position();
      let physics = Self::get_physics(bot);

      let packet = ServerboundMovePlayerPos {
        pos: Vec3::new(pos.x, pos.y + randfloat(config.min_change_y + 0.8, config.max_change_y + 0.8), pos.z),
        flags: MoveFlags {
          on_ground: physics.on_ground(),
          horizontal_collision: physics.horizontal_collision
        }
      };

      if config.use_jitter {
        for _ in 0..randint(4, 6) {
          bot.write_packet(packet.clone());
          bot.wait_ticks(1).await;
        }
      } else {
        bot.write_packet(packet);
      }

      bot.wait_ticks(randuint(config.min_delay, config.max_delay) as usize).await;

      Self::hover(bot, Instant::now() + Duration::from_millis(randuint(100, 150))).await;

      if config.use_ground_spoof {
        Self::set_on_ground(bot, false);
      }
    }
  }

  pub async fn teleport_flight(bot: &Client, options: FlightOptions) {
    let config = if options.settings.as_str() == "adaptive" {
      Self::create_adaptive_config(options.anti_cheat)
    } else {
      FlightConfig {
        min_delay: options.min_delay.unwrap_or(3),
        max_delay: options.max_delay.unwrap_or(8),
        min_change_y: options.min_change_y.unwrap_or(0.08),
        max_change_y: options.max_change_y.unwrap_or(0.09),
        use_ground_spoof: if let Some(v) = options.use_ground_spoof { v.as_str() == "true" } else { true },
        use_jitter: false
      }
    };

    loop {
      sleep(Duration::from_millis(randuint(50, 100))).await;

      if config.use_ground_spoof {
        Self::set_on_ground(bot, true);
      }

      let pos = bot.position();
      let physics = Self::get_physics(bot);

      let packet = ServerboundMovePlayerPos {
        pos: Vec3::new(pos.x, pos.y + randfloat(config.min_change_y + 0.8, config.max_change_y + 0.8), pos.z),
        flags: MoveFlags {
          on_ground: physics.on_ground(),
          horizontal_collision: physics.horizontal_collision
        }
      };

      if config.use_jitter {
        for _ in 0..randint(4, 6) {
          bot.write_packet(packet.clone());
          bot.wait_ticks(1).await;
        }
      } else {
        bot.write_packet(packet);
      }

      bot.wait_ticks(randuint(config.min_delay, config.max_delay) as usize).await;

      Self::hover(bot, Instant::now() + Duration::from_millis(randuint(100, 150))).await;

      if config.use_ground_spoof {
        Self::set_on_ground(bot, false);
      }
    }
  }

  pub async fn bug_flight(bot: &Client, options: FlightOptions) {
    let config = if options.settings.as_str() == "adaptive" {
      Self::create_adaptive_config(options.anti_cheat)
    } else {
      FlightConfig {
        min_delay: options.min_delay.unwrap_or(4),
        max_delay: options.max_delay.unwrap_or(8),
        min_change_y: options.min_change_y.unwrap_or(0.02),
        max_change_y: options.max_change_y.unwrap_or(0.05),
        use_ground_spoof: if let Some(v) = options.use_ground_spoof { v.as_str() == "true" } else { true },
        use_jitter: true
      }
    };

    loop {
      sleep(Duration::from_millis(randuint(100, 200))).await;

      if config.use_ground_spoof {
        Self::set_on_ground(bot, true);
      }

      let pos = bot.position();

      let block_under = BlockPos::new(pos.x as i32, (pos.y - 1.0) as i32, pos.z as i32);

      let distance_vec = Vec3::new(
        pos.x - (block_under.x as f64 + 0.5),
        pos.y - (block_under.y as f64 + 0.5),
        pos.z - (block_under.z as f64 + 0.5)
      );
        
      let distance = distance_vec.length().max(0.1);
      let direction = distance_vec.normalize();
        
      let strength = (4.0 / distance).min(2.0);
        
      let variation = randfloat(-0.2, 0.2);
      let final_strength = strength + variation;

      if config.use_jitter {
        for _ in 0..randint(4, 6) {
          Self::set_velocity_y(bot, direction.y.abs() * final_strength * 0.2);
          bot.wait_ticks(1).await;
        }
      } else {
        Self::set_velocity_y(bot, direction.y.abs() * final_strength * 0.2);
      }

      bot.wait_ticks(randuint(config.min_delay, config.max_delay) as usize).await;

      Self::hover(bot, Instant::now() + Duration::from_millis(randuint(600, 800))).await;

      if config.use_ground_spoof {
        Self::set_on_ground(bot, false);
      }
    }
  }

  pub async fn enable(bot: &Client, options: FlightOptions) {
    match options.mode.as_str() {
      "vanilla" => { Self::vanilla_flight(bot, options).await; },
      "jump-fly" => { Self::jump_flight(bot, options).await; },
      "teleport-fly" => { Self::teleport_flight(bot, options).await; },
      "bug-fly" => { Self::bug_flight(bot, options).await; },
      _ => {}
    } 
  } 

  pub fn stop(nickname: &String) {
    TASKS.get(nickname).unwrap().write().unwrap().stop_task("flight");
  }
}