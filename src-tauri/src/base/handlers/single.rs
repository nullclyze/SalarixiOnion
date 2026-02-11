use azalea::entity::metadata::Health;
use azalea::entity::HumanoidArm;
use azalea::entity::Position;
use azalea::local_player::TabList;
use azalea::prelude::*;
use azalea::protocol::common::client_information::ParticleStatus;
use azalea::protocol::packets::game::ClientboundGamePacket;
use azalea::Vec3;
use azalea::{ClientInformation, NoState};
use std::time::Duration;
use tokio::time::sleep;

use crate::base::*;
use crate::common::{get_health, get_satiety};
use crate::emit::*;
use crate::tools::*;
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

      if let Some(arc) = get_flow_manager() {
        let mut fm = arc.write();
        fm.bots.insert(nickname.clone(), bot.clone());

        if let Some(opts) = fm.options.clone() {
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
      }

      emit_event(EventType::Log(LogEventPayload {
        name: "system".to_string(),
        message: format!("Бот {} подключается...", nickname),
      }));

      PROFILES.set_str(&nickname, "status", "Соединение...");
    }
    Event::Spawn => {
      let nickname = bot.username();

      if let Some(arc) = get_flow_manager() {
        if let Some(options) = &arc.read().options {
          PLUGIN_MANAGER.load(&bot, &options.plugins);
        }
      }

      PROFILES.set_str(&nickname, "status", "Онлайн");

      if let Some(options) = get_current_options() {
        let pos = if let Some(p) = bot.get_component::<Position>() {
          Vec3::new(p.x, p.y, p.z)
        } else {
          Vec3::new(0.0, 0.0, 0.0)
        };

        let health = if let Some(h) = bot.get_component::<Health>() {
          h.0.to_string()
        } else {
          "?".to_string()
        };

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

        let min_delay;
        let max_delay;

        let c;
        let template;

        let action;

        if let Some(profile) = PROFILES.get(&nickname) {
          if !profile.registered {
            c = options.register_command.as_str().trim();
            template = options.register_template.trim().to_string();
            min_delay = options.register_min_delay;
            max_delay = options.register_max_delay;

            PROFILES.set_bool(&nickname, "registered", true);

            action = "зарегистрировался".to_string();
          } else {
            c = options.login_command.as_str().trim();
            template = options.login_template.trim().to_string();
            min_delay = options.login_min_delay;
            max_delay = options.login_max_delay;

            action = "залогинился".to_string();
          }

          sleep(Duration::from_millis(randuint(min_delay, max_delay))).await;

          let cmd = template
            .clone()
            .replace("@cmd", c)
            .replace("@pass", &profile.password);

          bot.chat(&cmd);

          emit_event(EventType::Log(LogEventPayload {
            name: "info".to_string(),
            message: format!("Бот {} {}: {}", &nickname, action, &cmd),
          }));
        }
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

      if let Some(arc) = get_flow_manager() {
        let mut fm = arc.write();
        fm.bots.remove(&nickname);
      }

      if let Some(tasks) = TASKS.get(&nickname) {
        tasks.write().unwrap().kill_all_tasks();
      }

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

          if let Some(arc) = get_flow_manager() {
            for (nickname, _) in arc.write().bots.iter() {
              if *nickname == sender {
                message = format!("%hbБот%sc ~ {}", message);
                break;
              }
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
    }
    Event::Tick => {
      let nickname = bot.username();

      if let Some(profile) = PROFILES.get(&nickname) {
        if profile.status.as_str() == "Онлайн" {
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
          PROFILES.set_num(&nickname, "health", get_health(&bot));
          PROFILES.set_num(&nickname, "satiety", get_satiety(&bot));
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
