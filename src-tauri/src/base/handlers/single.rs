use azalea::entity::HumanoidArm;
use azalea::prelude::*;
use azalea::protocol::common::client_information::ParticleStatus;
use azalea::protocol::packets::game::ClientboundGamePacket;
use azalea::{ClientInformation, NoState};
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::emit::*;
use crate::generators::*;
use crate::methods::SafeClientMethods;
use crate::webhook::*;

const ACCOUNTS_WITH_SKINS: &[&str] = &[
  "ordunury",
  "tqyd",
  "rittes",
  "Kalmdel",
  "rlyy",
  "tusxi",
  "hrfi",
  "seree",
  "NewGuyCri",
  "UnityForsaken",
  "B0RAA",
  "cigarist",
  "fumbled",
  "_Malrand_",
];

pub async fn single_handler(bot: Client, event: Event, _state: NoState) -> anyhow::Result<()> {
  match event {
    Event::Login => {
      let username = bot.username();

      if let Some(opts) = current_options() {
        bot.set_client_information(ClientInformation {
          view_distance: opts.view_distance,
          language: opts.language,
          chat_colors: opts.chat_colors,
          main_hand: if let Some(arm) = opts.humanoid_arm {
            if arm.as_str() == "left" {
              HumanoidArm::Left
            } else {
              HumanoidArm::Right
            }
          } else {
            if randchance(0.5) {
              HumanoidArm::Left
            } else {
              HumanoidArm::Right
            }
          },
          particle_status: ParticleStatus::Minimal,
          ..Default::default()
        });
      }

      send_log(format!("Бот {} подключается...", username), "system");

      PROFILES.set_str(&username, "status", "Соединение...");
    }
    Event::Spawn => {
      let username = bot.username();

      BOT_REGISTRY.register_bot(&username, bot.clone());

      BOT_REGISTRY.send_event(RegistryEvent::LoadPlugins {
        username: username.clone(),
      });

      TASKS.push(&username);
      STATES.push(&username);

      PROFILES.set_str(&username, "status", "Онлайн");

      if let Some(options) = current_options() {
        let pos = bot.feet_pos();
        let health = bot.get_health();

        let str_pos = format!("{}, {}, {}", pos.x, pos.y, pos.z);

        if options.use_webhook {
          send_webhook(
            options.webhook_settings.url,
            format!(
              "Бот {} заспавнился | Координаты (XYZ): {} | Здоровье: {} / 20",
              &username, str_pos, health
            ),
          );
        }

        send_log(
          format!(
            "Бот {} заспавнился, координаты (XYZ): {}",
            &username, str_pos
          ),
          "info",
        );

        send_message(
          "Система",
          format!(
            "Бот {} заспавнился<br><br>Координаты (XYZ): {}<br>Здоровье: {} / 20",
            &username, str_pos, health
          ),
        );

        default_authorize(&bot).await;
      }

      if let Some(profile) = PROFILES.get(&username) {
        if !profile.skin_is_set {
          if let Some(opts) = current_options() {
            match opts.skin_settings.skin_type.as_str() {
              "random" => {
                let command = format!(
                  "{} {}",
                  opts
                    .skin_settings
                    .set_skin_command
                    .unwrap_or("/skin".to_string()),
                  randelem(ACCOUNTS_WITH_SKINS).unwrap()
                );

                bot.chat(&command);

                sleep(Duration::from_millis(3000)).await;

                send_log(
                  format!("Бот {} успешно установил скин: {}", username, command),
                  "system",
                );

                bot.disconnect();

                BOT_REGISTRY.remove_bot(&username);
                PROFILES.set_bool(&username, "skin_is_set", true);
              }
              "custom" => {
                if let Some(n) = opts.skin_settings.custom_skin_by_nickname {
                  let command = format!(
                    "{} {}",
                    opts
                      .skin_settings
                      .set_skin_command
                      .unwrap_or("/skin".to_string()),
                    n
                  );

                  bot.chat(&command);

                  sleep(Duration::from_millis(3000)).await;

                  send_log(
                    format!("Бот {} успешно установил скин: {}", username, command),
                    "system",
                  );

                  bot.disconnect();

                  BOT_REGISTRY.remove_bot(&username);
                  PROFILES.set_bool(&username, "skin_is_set", true);
                }
              }
              _ => {}
            }
          }
        }
      }
    }
    Event::Disconnect(packet) => {
      let username = bot.username();

      TASKS.reset(&username);
      PLUGIN_MANAGER.destroy_all_tasks(&username);

      BOT_REGISTRY.remove_bot(&username);

      PROFILES.set_bool(&username, "logined", false);
      PROFILES.set_bool(&username, "captcha_caught", false);
      STATES.reset(&username);
      TASKS.remove(&username);

      PROFILES.set_str(&username, "status", "Оффлайн");

      if let Some(text) = packet {
        if let Some(options) = current_options() {
          if options.use_webhook {
            send_webhook(
              options.webhook_settings.url,
              format!("Бот {} отключился: {}", &username, text.to_html()),
            );
          }
        }

        send_log(
          format!("Бот {} отключился: {}", &username, text.to_string()),
          "info",
        );
        send_message(
          "Система",
          format!("Бот {} отключился: {}", &username, text.to_string()),
        );
      }
    }
    Event::Chat(packet) => {
      let nickname = bot.username();

      if let Some(options) = current_options() {
        if options.use_chat_monitoring {
          let sender = packet.sender().unwrap_or("unknown".to_string());

          let mut message = packet.message().to_html();

          for (username, _) in PROFILES.get_all() {
            if username == sender {
              message = format!("%hbБот%sc ~ {}", message);
              break;
            }
          }

          send_optional_event(OptionalEmitEvent::Chat(ChatEventPayload {
            receiver: nickname.clone(),
            message: message,
          }));
        }

        if options.use_anti_captcha {
          let opts = options.anti_captcha_settings.options.web;

          if options.anti_captcha_settings.captcha_type.as_str() == "web" {
            if let Some(url) = CAPTCHA_BYPASS.catch_url_from_message(
              packet.message().to_string(),
              opts
                .regex
                .unwrap_or(r"https?://[^\s]+".to_string())
                .as_str(),
              opts.required_url_part,
            ) {
              if let Some(profile) = PROFILES.get(&nickname) {
                if !profile.captcha_caught {
                  PROFILES.set_bool(&nickname, "captcha_caught", true);

                  if options.use_webhook && options.webhook_settings.information {
                    send_webhook(
                      options.webhook_settings.url,
                      format!("Бот {} получил ссылку на капчу: {}", nickname, url),
                    );
                  }

                  send_log(
                    format!("[ Анти-Капча ]: Бот {} получил ссылку на капчу", nickname),
                    "info",
                  );

                  if opts.mode.as_str() == "auto" {
                    CAPTCHA_BYPASS.send_event(CaptchaBypassEvent::SolveCaptcha { url: url.clone() });
                  } else {
                    send_optional_event(OptionalEmitEvent::AntiWebCaptcha(
                      AntiWebCaptchaEventPayload {
                        captcha_url: url,
                        nickname: nickname,
                      },
                    ));
                  }
                }
              }
            }
          }
        }
      }

      trigger_authorize(&bot, packet.message().to_string()).await;
    }
    Event::Tick => {
      let nickname = bot.username();

      if let Some(profile) = PROFILES.get(&nickname) {
        if profile.status.as_str() == "Онлайн" {
          if !bot.workable() {
            return Ok(());
          }

          let ping = if let Some(tab) = bot.get_players() {
            let mut result = 0;

            for (_, info) in tab.iter() {
              if info.profile.name == nickname {
                result = info.latency as u32;
                break;
              }
            }

            result
          } else {
            0
          };

          PROFILES.set_num(&nickname, "ping", ping);
          PROFILES.set_num(&nickname, "health", bot.get_health() as u32);
          PROFILES.set_num(&nickname, "satiety", bot.get_satiety());
        }
      }
    }
    Event::Packet(packet) => match &*packet {
      ClientboundGamePacket::MapItemData(data) => {
        let nickname = bot.username();

        if let Some(options) = current_options() {
          if options.use_anti_captcha {
            if options.anti_captcha_settings.captcha_type.as_str() == "map" {
              if let Some(map_patch) = &data.color_patch.0 {
                if let Some(profile) = PROFILES.get(&nickname) {
                  if !profile.captcha_caught {
                    PROFILES.set_bool(&nickname, "captcha_caught", true);

                    let base64_code = CAPTCHA_BYPASS.create_png_image(&map_patch.map_colors);

                    if options.use_webhook && options.webhook_settings.information {
                      send_webhook(
                        options.webhook_settings.url,
                        format!("Бот {} получил капчу с карты: {}", nickname, base64_code),
                      );
                    }

                    send_log(
                      format!("[ Анти-Капча ]: Бот {} получил капчу с карты", nickname),
                      "info",
                    );

                    send_optional_event(OptionalEmitEvent::AntiMapCaptcha(
                      AntiMapCaptchaEventPayload {
                        base64_code: base64_code,
                        nickname: nickname.to_string(),
                      },
                    ));
                  }
                }
              }
            }
          }
        }
      }
      _ => {}
    },
    _ => {}
  }

  Ok(())
}
