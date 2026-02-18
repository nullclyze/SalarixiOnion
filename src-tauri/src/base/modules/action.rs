use azalea::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::common::SafeClientImpls;
use crate::common::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionOptions {
  pub action: String,
  pub use_sync: bool,
  pub use_impulsiveness: bool,
  pub state: bool,
}

impl ActionModule {
  pub fn new() -> Self {
    Self
  }

  pub async fn jumping(&self, bot: &Client, options: &ActionOptions) {
    if options.use_impulsiveness {
      loop {
        if options.use_sync {
          sleep(Duration::from_millis(1000)).await;
        } else {
          sleep(Duration::from_millis(randuint(900, 1800))).await;
        }

        bot.jump();
      }
    } else {
      if !options.use_sync {
        sleep(Duration::from_millis(randuint(200, 2000))).await;
      }

      bot.set_jumping(true);
    }
  }

  pub async fn shifting(&self, bot: &Client, options: &ActionOptions) {
    if options.use_impulsiveness {
      loop {
        if options.use_sync {
          sleep(Duration::from_millis(1000)).await;
        } else {
          sleep(Duration::from_millis(randuint(900, 1800))).await;
        }

        bot.set_crouching(true);
        sleep(Duration::from_millis(350)).await;
        bot.set_crouching(false);
      }
    } else {
      if !options.use_sync {
        sleep(Duration::from_millis(randuint(200, 2000))).await;
      }

      bot.set_crouching(true);
    }
  }

  pub async fn waving(&self, bot: &Client, options: &ActionOptions) {
    if options.use_impulsiveness {
      loop {
        if options.use_sync {
          sleep(Duration::from_millis(300)).await;
        } else {
          sleep(Duration::from_millis(randuint(300, 800))).await;
        }

        bot.swing_arm();
      }
    } else {
      if !options.use_sync {
        sleep(Duration::from_millis(randuint(200, 2000))).await;
      }

      loop {
        bot.swing_arm();
        sleep(Duration::from_millis(300)).await;
      }
    }
  }

  pub fn stop(&self, bot: &Client, action: &str) {
    kill_task(&bot.username(), action);

    match action {
      "jumping" => {
        bot.set_jumping(false);
      }
      "shifting" => {
        bot.set_crouching(false);
      }
      _ => {}
    }
  }
}
