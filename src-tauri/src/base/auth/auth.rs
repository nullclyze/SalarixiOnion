use azalea::prelude::*;
use std::time::Duration;
use tokio::time::sleep;

use crate::base::{PROFILES, get_current_options};
use crate::emit::{EventType, LogEventPayload, emit_event};
use crate::tools::randuint;

// Функция default-авторизации бота
pub async fn default_authorize(bot: &Client) {
	let username = bot.username();

	if let Some(opts) = get_current_options() {
		let mut min_delay = 2000;
		let mut max_delay = 4000;

		let mut c = "!NONE".to_string();
		let mut template = "@cmd @pass".to_string();

		if let Some(profile) = PROFILES.get(&username) {
			if !profile.registered {
				if opts.use_auto_register && opts.register_mode == "default" {
					c = opts.register_command.as_str().trim().to_string();
					template = opts.register_template.trim().to_string();
					min_delay = opts.register_min_delay;
					max_delay = opts.register_max_delay;

					PROFILES.set_bool(&username, "registered", true);
					PROFILES.set_bool(&username, "logined", true);
				}
			} else if !profile.logined {
				if opts.use_auto_login && opts.login_mode == "default" {
					c = opts.login_command.as_str().trim().to_string();
					template = opts.login_template.trim().to_string();
					min_delay = opts.login_min_delay;
					max_delay = opts.login_max_delay;

					PROFILES.set_bool(&username, "logined", true);
				}
			}

			if c.as_str() != "!NONE" {
				sleep(Duration::from_millis(randuint(min_delay, max_delay))).await;

				let cmd = template
					.clone()
					.replace("@cmd", &c)
					.replace("@pass", &profile.password);

				bot.chat(&cmd);

				emit_event(EventType::Log(LogEventPayload {
					name: "info".to_string(),
					message: format!("Бот {} авторизировался: {}", &username, &cmd),
				}));
			}
		}
	}
}

// Функция trigger-авторизации бота
pub async fn trigger_authorize(bot: &Client, message: String) {
	let username = bot.username();

	if let Some(opts) = get_current_options() {
		if let Some(profile) = PROFILES.get(&username) {
			let pat = if !profile.registered { opts.register_trigger } else { opts.login_trigger };

			if !message.to_lowercase().contains(&pat) {
				return;
			}
		}
	}

	if let Some(profile) = PROFILES.get(&username) {
		if let Some(opts) = get_current_options() {
			let mut c = "!NONE".to_string();
			let mut template = "@cmd @pass".to_string();

			if !profile.registered {
				if opts.use_auto_register && opts.register_mode == "trigger" {
					c = opts.register_command.as_str().trim().to_string();
					template = opts.register_template.trim().to_string();

					PROFILES.set_bool(&username, "registered", true);
					PROFILES.set_bool(&username, "logined", true);
				}
			} else if !profile.logined && opts.login_mode == "trigger" {
				if opts.use_auto_login {
					c = opts.login_command.as_str().trim().to_string();
					template = opts.login_template.trim().to_string();

					PROFILES.set_bool(&username, "logined", true);
				}
			}

			if c.as_str() != "!NONE" {
				let cmd = template
					.clone()
					.replace("@cmd", &c)
					.replace("@pass", &profile.password);

				bot.chat(&cmd);

				emit_event(EventType::Log(LogEventPayload {
					name: "info".to_string(),
					message: format!("Бот {} авторизировался: {}", &username, &cmd),
				}));
			}	
		}
	}
}