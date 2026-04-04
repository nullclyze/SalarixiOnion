use azalea::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

use crate::core::{current_options, PROFILES};
use crate::emit::send_log;
use crate::generators::*;

const DEFAULT_AUTH_TEMPLATE: &str = "$cmd $pass";

/// Функция default-авторизации бота
pub async fn default_authorize(bot: &Client) {
  let username = bot.username();

  if let Some(opts) = current_options() {
    let mut min_delay = 2000;
    let mut max_delay = 4000;

    let mut c = "!NONE".to_string();
    let mut template = DEFAULT_AUTH_TEMPLATE;

    if let Some(profile) = PROFILES.get(&username) {
      let mut action = "";

      if !profile.registered {
        if opts.basic.use_auto_register && opts.basic.register_mode == "default" {
          c = opts.basic.register_command.as_str().trim().to_string();
          template = opts.basic.register_template.trim();
          min_delay = opts.basic.register_min_delay;
          max_delay = opts.basic.register_max_delay;

          PROFILES.set_bool(&username, "registered", true);

          action = "зарегистрировался";

          if !opts.basic.use_double_auth {
            PROFILES.set_bool(&username, "logined", true);
          }
        }
      } else if !profile.logined {
        if opts.basic.use_auto_login && opts.basic.login_mode == "default" {
          c = opts.basic.login_command.as_str().trim().to_string();
          template = opts.basic.login_template.trim();
          min_delay = opts.basic.login_min_delay;
          max_delay = opts.basic.login_max_delay;

          action = "залогинился";

          PROFILES.set_bool(&username, "logined", true);
        }
      }

      if c.as_str() != "!NONE" {
        sleep(Duration::from_millis(randuint(min_delay, max_delay))).await;

        let cmd = template
          .replace("$cmd", &c)
          .replace("$pass", &profile.password.unwrap_or(String::new()))
          .replace("$email", &profile.email.unwrap_or(String::new()));

        bot.chat(&cmd);

        send_log(
          format!("Бот {} {}: {}", &username, action, &cmd),
          "info",
        );
      }
    }
  }
}

/// Функция trigger-авторизации бота
pub async fn trigger_authorize(bot: &Client, message: String) {
  let username = bot.username();

  if let Some(opts) = current_options() {
    if let Some(profile) = PROFILES.get(&username) {
      let pat = if !profile.registered {
        opts.basic.register_trigger
      } else {
        opts.basic.login_trigger
      };

      if !message.to_lowercase().contains(&pat) {
        return;
      }
    }
  }

  if let Some(profile) = PROFILES.get(&username) {
    if let Some(opts) = current_options() {
      let mut c = "!NONE".to_string();
      let mut template = DEFAULT_AUTH_TEMPLATE;

      let mut action = "";

      if !profile.registered {
        if opts.basic.use_auto_register && opts.basic.register_mode == "trigger" {
          c = opts.basic.register_command.as_str().trim().to_string();
          template = opts.basic.register_template.trim();

          action = "зарегистрировался";

          PROFILES.set_bool(&username, "registered", true);

          if !opts.basic.use_double_auth {
            PROFILES.set_bool(&username, "logined", true);
          }
        }
      } else if !profile.logined && opts.basic.login_mode == "trigger" {
        if opts.basic.use_auto_login {
          c = opts.basic.login_command.as_str().trim().to_string();
          template = opts.basic.login_template.trim();

          action = "залогинился";

          PROFILES.set_bool(&username, "logined", true);
        }
      }

      if c.as_str() != "!NONE" {
        let cmd = template
          .replace("$cmd", &c)
          .replace("$pass", &profile.password.unwrap_or(String::new()))
          .replace("$email", &profile.email.unwrap_or(String::new()));

        bot.chat(&cmd);

        send_log(
          format!("Бот {} {}: {}", &username, action, &cmd),
          "info",
        );
      }
    }
  }
}
