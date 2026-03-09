use azalea::prelude::*;
use azalea::WalkDirection;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

use crate::core::*;
use crate::extensions::{BotDefaultExt, BotMovementExt};
use crate::generators::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiAfkModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiAfkOptions {
  pub mode: String,
  pub min_delay: Option<u64>,
  pub max_delay: Option<u64>,
  pub state: bool,
}

impl AntiAfkModule {
  pub fn new() -> Self {
    Self
  }

  async fn minimal(&self, bot: &Client, options: &AntiAfkOptions) {
    let min_delay = options.min_delay.unwrap_or(1000);
    let max_delay = options.max_delay.unwrap_or(2500);

    loop {
      let original_direction = bot.direction();

      bot.set_direction(
        original_direction.0 + randfloat(-50.0, 50.0) as f32,
        original_direction.1 + randfloat(-12.0, 12.0) as f32,
      );

      if randchance(0.4) {
        bot.swing_arm();
      }

      sleep(Duration::from_millis(randuint(400, 800))).await;

      bot.set_direction(
        original_direction.0 + randfloat(-10.0, 10.0) as f32,
        original_direction.1 + randfloat(-4.5, 4.5) as f32,
      );

      sleep(Duration::from_millis(randuint(min_delay, max_delay))).await;
    }
  }

  async fn normal(&self, bot: &Client, options: &AntiAfkOptions) {
    let min_delay = options.min_delay.unwrap_or(1000);
    let max_delay = options.max_delay.unwrap_or(2500);

    loop {
      let walk_directions = vec![
        WalkDirection::Forward,
        WalkDirection::Backward,
        WalkDirection::Left,
        WalkDirection::Right,
        WalkDirection::ForwardLeft,
        WalkDirection::ForwardRight,
        WalkDirection::BackwardLeft,
        WalkDirection::BackwardRight,
      ];

      let walk_direction = randelem(&walk_directions).unwrap();

      bot.start_walking(*walk_direction);

      let direction = bot.direction();

      bot.set_direction(
        direction.0 + randfloat(-35.0, 35.0) as f32,
        direction.1 + randfloat(-20.0, 20.0) as f32,
      );

      sleep(Duration::from_millis(randuint(150, 400))).await;

      bot.stop_move();

      sleep(Duration::from_millis(randuint(min_delay, max_delay))).await;
    }
  }

  async fn advanced(&self, bot: &Client, options: &AntiAfkOptions) {
    let min_delay = options.min_delay.unwrap_or(1000);
    let max_delay = options.max_delay.unwrap_or(2500);

    let username = bot.username();

    tokio::spawn(async move {
      loop {
        if let Some(bot) = BOT_REGISTRY.get_bot(&username) {
          let direction = bot.direction();

          bot.set_direction(
            direction.0 + randfloat(-35.0, 35.0) as f32,
            direction.1 + randfloat(-10.0, 10.0) as f32,
          );
        }

        if !TASKS.get_task_activity(&username, "anti-afk") {
          set_mutual_states(&username, "looking", false);
          break;
        }

        sleep(Duration::from_millis(50)).await;
      }
    });

    loop {
      bot.start_walking(WalkDirection::Forward);

      if randchance(0.4) {
        bot.swing_arm();
      }

      if randchance(0.4) {
        bot.start_crouching();
        sleep(Duration::from_millis(randuint(300, 500))).await;
        bot.stop_crouching();
      }

      if randchance(0.3) {
        bot.jump();
      }

      if !TASKS.get_task_activity(&bot.username(), "anti-afk") {
        bot.stop_move();
        break;
      }

      sleep(Duration::from_millis(randuint(min_delay, max_delay))).await;
    }
  }

  pub async fn enable(&self, username: &str, options: &AntiAfkOptions) {
    set_mutual_states(username, "looking", true);

    BOT_REGISTRY
      .async_get_bot(username, async |bot| match options.mode.as_str() {
        "minimal" => {
          self.minimal(bot, options).await;
        }
        "normal" => {
          self.normal(bot, options).await;
        }
        "advanced" => {
          self.advanced(bot, options).await;
        }
        _ => {}
      })
      .await;
  }

  pub async fn stop(&self, username: &str) {
    kill_task(username, "anti-afk");

    if let Some(bot) = BOT_REGISTRY.get_bot(username) {
      bot.stop_move();
      bot.stop_crouching();
    }

    set_mutual_states(&username, "looking", false);
  }
}
