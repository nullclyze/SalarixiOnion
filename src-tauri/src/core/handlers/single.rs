use azalea::entity::HumanoidArm;
use azalea::prelude::*;
use azalea::protocol::common::client_information::ParticleStatus;
use azalea::protocol::packets::game::ClientboundGamePacket;
use azalea::{ClientInformation, NoState};
use std::time::Duration;
use tokio::time::sleep;

use crate::core::*;
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
          view_distance: opts.basic.view_distance,
          language: opts.basic.language,
          chat_colors: true,
          main_hand: if let Some(arm) = opts.basic.humanoid_arm {
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

        if options.basic.use_webhook {
          send_webhook(
            options.webhook.url,
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
            match opts.basic.skin_type.as_str() {
              "random" => {
                let command = format!(
                  "{} {}",
                  opts.basic.set_skin_command.unwrap_or("/skin".to_string()),
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
                if let Some(n) = opts.basic.custom_skin_by_nickname {
                  let command = format!(
                    "{} {}",
                    opts.basic.set_skin_command.unwrap_or("/skin".to_string()),
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
          if options.basic.use_webhook {
            send_webhook(
              options.webhook.url,
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
        if options.basic.use_chat_monitoring {
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

        if options.basic.use_anti_captcha {
          if options.captcha_bypass.captcha_type.as_str() == "web" {
            if let Some(url) = WEB_CAPTCHA_BYPASS.catch_url_from_message(
              packet.message().to_string(),
              options
                .captcha_bypass
                .regex
                .unwrap_or(r"https?://[^\s]+".to_string())
                .as_str(),
              options.captcha_bypass.required_url_part,
            ) {
              if let Some(profile) = PROFILES.get(&nickname) {
                if !profile.captcha_caught {
                  PROFILES.set_bool(&nickname, "captcha_caught", true);

                  if options.basic.use_webhook && options.webhook.send_information {
                    send_webhook(
                      options.webhook.url,
                      format!("Бот {} получил ссылку на капчу: {}", nickname, url),
                    );
                  }

                  send_log(
                    format!("[ Анти-Капча ]: Бот {} получил ссылку на капчу", nickname),
                    "info",
                  );

                  if options.captcha_bypass.solve_mode.as_str() == "auto" {
                    WEB_CAPTCHA_BYPASS.send_webdriver_event(WebDriverEvent::OpenUrl {
                      url: url.clone(),
                      proxy: profile.proxy.proxy,
                      username: profile.proxy.username,
                      password: profile.proxy.password,
                    });
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
          if options.basic.use_anti_captcha {
            if options.captcha_bypass.captcha_type.as_str() == "map" {
              if let Some(map_patch) = &data.color_patch.0 {
                if let Some(profile) = PROFILES.get(&nickname) {
                  if !profile.captcha_caught {
                    PROFILES.set_bool(&nickname, "captcha_caught", true);

                    let base64_code = MAP_CAPTCHA_BYPASS.create_png_image(&map_patch.map_colors);

                    if options.basic.use_webhook && options.webhook.send_information {
                      send_webhook(
                        options.webhook.url,
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
