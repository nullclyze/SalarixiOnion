use azalea::prelude::*;
use azalea::WalkDirection;
use azalea::interact::SwingArmEvent;  
use serde::{Serialize, Deserialize};
use core::time;

use crate::TASKS;
use crate::tools::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiAfkModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiAfkOptions {
  pub mode: String,
  pub min_delay: Option<u64>,
  pub max_delay: Option<u64>,
  pub state: bool
}

impl AntiAfkModule {
  async fn minimal(bot: &Client, options: AntiAfkOptions) {
    let min_delay = if let Some(delay) = options.min_delay { delay } else { 2000 };
    let max_delay = if let Some(delay) = options.max_delay { delay } else { 5000 };

    loop {
      tokio::time::sleep(time::Duration::from_millis(randuint(min_delay, max_delay))).await;

      let original_direction = bot.direction();

      bot.set_direction(original_direction.0 + randfloat(-50.0, 50.0) as f32, original_direction.1 + randfloat(-12.0, 12.0) as f32);

      if randfloat(0.0, 100.0) > 40.0 {
        bot.ecs.lock().trigger(SwingArmEvent { entity: bot.entity });  
      }

      bot.wait_ticks(randticks(8, 16)).await;

      bot.set_direction(original_direction.0 + randfloat(-8.0, 8.0) as f32, original_direction.1 + randfloat(-4.5, 4.5) as f32);
    }
  }

  async fn normal(bot: &Client, options: AntiAfkOptions) {
    let min_delay = if let Some(delay) = options.min_delay { delay } else { 2000 };
    let max_delay = if let Some(delay) = options.max_delay { delay } else { 5000 };

    loop {
      tokio::time::sleep(time::Duration::from_millis(randuint(min_delay, max_delay))).await;

      let walk_directions = vec![
        WalkDirection::Forward, WalkDirection::Backward, WalkDirection::Left, WalkDirection::Right,
        WalkDirection::ForwardLeft, WalkDirection::ForwardRight, WalkDirection::BackwardLeft, WalkDirection::BackwardRight
      ];

      let walk_direction = randelem(&walk_directions).unwrap();

      bot.walk(*walk_direction);

      let direction = bot.direction();

      bot.set_direction(direction.0 + randfloat(-40.0, 40.0) as f32, direction.1 + randfloat(-40.0, 40.0) as f32);

      tokio::time::sleep(time::Duration::from_millis(randuint(150, 400))).await;

      bot.walk(WalkDirection::None);
    }
  }

  pub async fn enable(bot: &Client, options: AntiAfkOptions) {
    match options.mode.as_str() {
      "minimal" => { Self::minimal(bot, options).await; },
      "normal" => { Self::normal(bot, options).await; },
      _ => {}
    }
  } 

  pub fn stop(bot: &Client) {
    TASKS.get(&bot.username()).unwrap().write().unwrap().stop_task("anti-afk");
    bot.walk(WalkDirection::None);
  }
}