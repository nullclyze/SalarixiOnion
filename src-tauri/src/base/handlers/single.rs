use azalea::entity::HumanoidArm;
use azalea::local_player::TabList;
use azalea::prelude::*;
use azalea::protocol::common::client_information::ParticleStatus;
use azalea::protocol::packets::game::ClientboundGamePacket;
use azalea::{ClientInformation, NoState};
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::common::*;
use crate::emit::*;
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
      let nickname = bot.username();

      if let Some(opts) = get_current_options() {
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

      emit_event(EventType::Log(LogEventPayload {
        name: "system".to_string(),
        message: format!("Бот {} подключается...", nickname),
      }));

      PROFILES.set_str(&nickname, "status", "Соединение...");

      TASKS.push(&nickname);
      STATES.push(&nickname);
    }
    Event::Spawn => {
      let nickname = bot.username();

      BOT_REGISTRY.register_bot(&nickname, bot.clone());

      BOT_REGISTRY.send_event(BotEvent::LoadPlugins {
        username: nickname.clone(),
      });

      PROFILES.set_str(&nickname, "status", "Онлайн");

      if let Some(options) = get_current_options() {
        let pos = bot.feet_pos();
        let health = bot.get_health();

        let str_pos = format!("{}, {}, {}", pos.x, pos.y, pos.z);

        if options.use_webhook {
          send_webhook(
            options.webhook_settings.url,
            format!(
              "Бот {} заспавнился | Координаты (XYZ): {} | Здоровье: {} / 20",
              &nickname, str_pos, health
            ),
          );
        }

        emit_event(EventType::Log(LogEventPayload {
          name: "info".to_string(),
          message: format!(
            "Бот {} заспавнился, координаты (XYZ): {}",
            &nickname, str_pos
          ),
        }));

        emit_message(
          "Система",
          format!(
            "Бот {} заспавнился<br><br>Координаты (XYZ): {}<br>Здоровье: {} / 20",
            &nickname, str_pos, health
          ),
        );

        default_authorize(&bot).await;
      }

      if let Some(profile) = PROFILES.get(&nickname) {
        if !profile.skin_is_set {
          if let Some(opts) = get_current_options() {
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

                emit_event(EventType::Log(LogEventPayload {
                  name: "system".to_string(),
                  message: format!("Бот {} успешно установил скин: {}", nickname, command),
                }));

                bot.disconnect();

                BOT_REGISTRY.remove_bot(&nickname);
                PROFILES.set_bool(&nickname, "skin_is_set", true);
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

                  emit_event(EventType::Log(LogEventPayload {
                    name: "system".to_string(),
                    message: format!("Бот {} успешно установил скин: {}", nickname, command),
                  }));

                  bot.disconnect();

                  BOT_REGISTRY.remove_bot(&nickname);
                  PROFILES.set_bool(&nickname, "skin_is_set", true);
                }
              }
              _ => {}
            }
          }
        }
      }
    }
    Event::Disconnect(packet) => {
      let nickname = bot.username();

      if let Some(tasks) = TASKS.get(&nickname) {
        tasks.write().unwrap().kill_all_tasks();
      }

      BOT_REGISTRY.remove_bot(&nickname);

      PROFILES.set_bool(&nickname, "logined", false);
      PROFILES.set_bool(&nickname, "captcha_caught", false);
      STATES.reset(&nickname);
      TASKS.remove(&nickname);

      PROFILES.set_str(&nickname, "status", "Оффлайн");

      if let Some(text) = packet {
        if let Some(options) = get_current_options() {
          if options.use_webhook {
            send_webhook(
              options.webhook_settings.url,
              format!("Бот {} отключился: {}", &nickname, text.to_html()),
            );
          }
        }

        emit_event(EventType::Log(LogEventPayload {
          name: "info".to_string(),
          message: format!("Бот {} отключился: {}", &nickname, text.to_html()),
        }));

        emit_message(
          "Система",
          format!("Бот {} отключился: {}", &nickname, text.to_string()),
        );
      }
    }
    Event::Chat(packet) => {
      let nickname = bot.username();

      if let Some(options) = get_current_options() {
        if options.use_chat_monitoring {
          let sender = packet.sender().unwrap_or("unknown".to_string());

          let mut message = packet.message().to_html();

          for (username, _) in PROFILES.get_all() {
            if username == sender {
              message = format!("%hbБот%sc ~ {}", message);
              break;
            }
          }

          emit_event(EventType::Chat(ChatEventPayload {
            receiver: nickname.clone(),
            message: message,
          }));
        }

        if options.use_anti_captcha {
          let opts = options.anti_captcha_settings.options.web;

          if options.anti_captcha_settings.captcha_type.as_str() == "web" {
            if let Some(url) = ANTI_WEB_CAPTCHA.catch_url_from_message(
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

                  emit_event(EventType::Log(LogEventPayload {
                    name: "info".to_string(),
                    message: format!("[ Анти-Капча ]: Бот {} получил ссылку на капчу", nickname),
                  }));

                  emit_event(EventType::AntiWebCaptcha(AntiWebCaptchaEventPayload {
                    captcha_url: url,
                    nickname: nickname,
                  }));
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

          let ping = if let Some(tab) = bot.get_component::<TabList>() {
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

        if let Some(options) = get_current_options() {
          if options.use_anti_captcha {
            if options.anti_captcha_settings.captcha_type.as_str() == "map" {
              if let Some(map_patch) = &data.color_patch.0 {
                if let Some(profile) = PROFILES.get(&nickname) {
                  if !profile.captcha_caught {
                    PROFILES.set_bool(&nickname, "captcha_caught", true);

                    let base64_code = ANTI_MAP_CAPTCHA.create_png_image(&map_patch.map_colors);

                    if options.use_webhook && options.webhook_settings.information {
                      send_webhook(
                        options.webhook_settings.url,
                        format!("Бот {} получил капчу с карты: {}", nickname, base64_code),
                      );
                    }

                    emit_event(EventType::Log(LogEventPayload {
                      name: "info".to_string(),
                      message: format!("[ Анти-Капча ]: Бот {} получил капчу с карты", nickname),
                    }));

                    emit_event(EventType::AntiMapCaptcha(AntiMapCaptchaEventPayload {
                      base64_code: base64_code,
                      nickname: nickname.to_string(),
                    }));
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
