use azalea::prelude::*;
use serde::{Serialize, Deserialize};
use std::time::Duration;
use tokio::time::sleep;

use crate::common::swing_arm;
use crate::tools::*;
use crate::base::*;


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionModule;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionOptions {
  pub action: String,
  pub use_sync: bool,
  pub use_impulsiveness: bool,
  pub state: bool
}

impl ActionModule {
  pub async fn jumping(bot: &Client, options: ActionOptions) {
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

  pub async fn shifting(bot: &Client, options: ActionOptions) {
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

  pub async fn waving(bot: &Client, options: ActionOptions) {
    if options.use_impulsiveness {
      loop {
        if options.use_sync {
          sleep(Duration::from_millis(300)).await;
        } else {
          sleep(Duration::from_millis(randuint(300, 800))).await;
        }

        swing_arm(bot);
      }
    } else {
      if !options.use_sync {
        sleep(Duration::from_millis(randuint(200, 2000))).await;
      }

      loop {
        swing_arm(bot);

        sleep(Duration::from_millis(300)).await;
      }
    }
  }

  pub fn stop(bot: &Client, action: &str) {
    TASKS.get(&bot.username()).unwrap().write().unwrap().kill_task(action);

    match action {
      "jumping" => { bot.set_jumping(false); },
      "shifting" => { bot.set_crouching(false); },
      _ => {}
    }
  }
}