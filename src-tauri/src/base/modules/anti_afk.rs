use azalea::SprintDirection;
use azalea::prelude::*;
use azalea::WalkDirection;
use serde::{Serialize, Deserialize};
use tokio::time::sleep;
use std::time::Duration;

use crate::common::swing_arm;
use crate::common::{go, run};
use crate::tools::*;
use crate::base::*;


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
    let max_delay = if let Some(delay) = options.max_delay { delay } else { 4000 };

    loop {
      let original_direction = bot.direction();

      bot.set_direction(original_direction.0 + randfloat(-50.0, 50.0) as f32, original_direction.1 + randfloat(-12.0, 12.0) as f32);

      if randchance(0.4) {
        swing_arm(bot);
      }

      sleep(Duration::from_millis(randuint(400, 800))).await;

      bot.set_direction(original_direction.0 + randfloat(-8.0, 8.0) as f32, original_direction.1 + randfloat(-4.5, 4.5) as f32);
    
      sleep(Duration::from_millis(randuint(min_delay, max_delay))).await;
    }
  }

  async fn normal(bot: &Client, options: AntiAfkOptions) {
    let min_delay = if let Some(delay) = options.min_delay { delay } else { 2000 };
    let max_delay = if let Some(delay) = options.max_delay { delay } else { 4000 };

    loop {
      let walk_directions = vec![
        WalkDirection::Forward, WalkDirection::Backward, WalkDirection::Left, WalkDirection::Right,
        WalkDirection::ForwardLeft, WalkDirection::ForwardRight, WalkDirection::BackwardLeft, WalkDirection::BackwardRight
      ];

      let walk_direction = randelem(&walk_directions).unwrap();

      go(bot, *walk_direction);

      let direction = bot.direction();

      bot.set_direction(direction.0 + randfloat(-40.0, 40.0) as f32, direction.1 + randfloat(-40.0, 40.0) as f32);

      sleep(Duration::from_millis(randuint(150, 400))).await;

      bot.walk(WalkDirection::None);

      sleep(Duration::from_millis(randuint(min_delay, max_delay))).await;
    }
  }

  async fn advanced(bot: &Client, options: AntiAfkOptions) {
    let min_delay = if let Some(delay) = options.min_delay { delay } else { 1000 };
    let max_delay = if let Some(delay) = options.max_delay { delay } else { 1800 };

    let bot_clone = bot.clone();

    tokio::spawn(async move {
      loop {
        if !TASKS.get_task_activity(&bot_clone.username(), "anti-afk") {
          break;
        }

        sleep(Duration::from_millis(50)).await;

        let direction = bot_clone.direction();

        bot_clone.set_direction(direction.0 + randfloat(-15.0, 15.0) as f32, direction.1 + randfloat(-15.0, 15.0) as f32);
      }
    });

    loop {
      run(bot, SprintDirection::Forward);

      if randchance(0.4) {
        swing_arm(bot);
      }

      sleep(Duration::from_millis(randuint(min_delay, max_delay))).await;
    }
  }

  pub async fn enable(bot: &Client, options: AntiAfkOptions) {
    match options.mode.as_str() {
      "minimal" => { Self::minimal(bot, options).await; },
      "normal" => { Self::normal(bot, options).await; },
      "advanced" => { Self::advanced(bot, options).await; },
      _ => {}
    }
  } 

  pub fn stop(bot: &Client) {
    TASKS.get(&bot.username()).unwrap().write().unwrap().kill_task("anti-afk");
    bot.walk(WalkDirection::None);
  }
}