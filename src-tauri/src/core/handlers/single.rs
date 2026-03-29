use azalea::entity::HumanoidArm;
use azalea::entity::LookDirection;
use azalea::prelude::*;
use azalea::protocol::common::client_information::ParticleStatus;
use azalea::protocol::packets::game::ClientboundGamePacket;
use azalea::registry::builtin::EntityKind;
use azalea::{ClientInformation, NoState};
use std::io;
use std::time::Duration;
use tokio::time::sleep;

use crate::core::utils::MAP_ACCUMULATOR;
use crate::core::*;
use crate::emit::*;
use crate::extensions::BotDefaultExt;
use crate::generators::*;
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

pub async fn single_handler(bot: Client, event: Event, _state: NoState) -> io::Result<()> {
  match event {
    Event::Init => {
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
    }
    Event::Login => {
      let username = bot.name();
      send_log(format!("Бот {} подключается...", username), "system");
      PROFILES.set_status(&username, ProfileStatus::Connecting);
    }
    Event::Spawn => {
      let username = bot.name();

      BOT_REGISTRY.register_bot(&username, bot.clone());

      BOT_REGISTRY.send_event(RegistryEvent::LoadPlugins {
        username: username.clone(),
      });

      TASKS.push(&username);
      STATES.push(&username);

      PROFILES.set_status(&username, ProfileStatus::Online);

      if let Some(options) = current_options() {
        let pos = bot.feet_pos();
        let health = bot.get_health();

        let str_pos = format!("{}, {}, {}", pos.x as i32, pos.y as i32, pos.z as i32);

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

      let Some(profile) = PROFILES.get(&username) else {
        return Ok(());
      };

      let Some(opts) = current_options() else {
        return Ok(());
      };

      if !profile.skin_is_set {
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
    Event::Disconnect(packet) => {
      let username = bot.name();

      TASKS.reset(&username);
      PLUGIN_MANAGER.destroy_all_tasks(&username);

      BOT_REGISTRY.take_bot(&username).await;

      PROFILES.set_bool(&username, "logined", false);
      PROFILES.set_bool(&username, "captcha_caught", false);
      STATES.reset(&username);
      TASKS.remove(&username);

      PROFILES.set_status(&username, ProfileStatus::Offline);

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
      let username = bot.name();

      let Some(options) = current_options() else {
        return Ok(());
      };

      send_optional_event(OptionalEmitEvent::Chat(ChatEventPayload {
        receiver: username.clone(),
        message: packet.message().to_html(),
      }));

      if options.basic.use_anti_captcha && options.captcha_bypass.captcha_type.as_str() == "web" {
        if let Some(url) = WEB_CAPTCHA_BYPASS.catch_url_from_message(
          packet.message().to_string(),
          options.captcha_bypass.regex.as_str(),
          options.captcha_bypass.required_url_part,
        ) {
          let Some(profile) = PROFILES.get(&username) else {
            return Ok(());
          };

          if !profile.captcha_caught {
            PROFILES.set_bool(&username, "captcha_caught", true);

            if options.basic.use_webhook && options.webhook.send_information {
              send_webhook(
                options.webhook.url,
                format!("Бот {} получил ссылку на капчу: {}", username, url),
              );
            }

            send_log(
              format!("[ Анти-Капча ]: Бот {} получил ссылку на капчу", username),
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
                  username: username,
                },
              ));
            }
          }
        }
      }

      trigger_authorize(&bot, packet.message().to_string()).await;
    }
    Event::Tick => {
      let username = bot.name();

      let Some(profile) = PROFILES.get(&username) else {
        return Ok(());
      };

      if profile.status == ProfileStatus::Online {
        if !bot.workable() {
          return Ok(());
        }

        PROFILES.set_num(&username, "ping", bot.ping());
        PROFILES.set_num(&username, "health", bot.get_health() as u32);
      }
    }
    Event::Packet(packet) => match &*packet {
      ClientboundGamePacket::AddEntity(entity_packet) => {
        if entity_packet.entity_type == EntityKind::ItemFrame
          || entity_packet.entity_type == EntityKind::GlowItemFrame
        {
          let username = bot.name();
          let pos = entity_packet.position;
          MAP_ACCUMULATOR.add_frame_position(
            &username,
            entity_packet.id.0 as i32,
            pos.x,
            pos.y,
            pos.z,
          );
        }
      }
      ClientboundGamePacket::MapItemData(data) => {
        let username = bot.name();

        if let Some(options) = current_options() {
          if options.basic.use_anti_captcha && options.captcha_bypass.captcha_type.as_str() == "map"
          {
            let Some(map_patch) = &data.color_patch.0 else {
              return Ok(());
            };

            let Some(profile) = PROFILES.get(&username) else {
              return Ok(());
            };

            if !profile.captcha_caught {
              let is_frame = options.captcha_bypass.captcha_subtype.as_str() == "frame";

              if is_frame {
                let pos = bot.feet_pos();
                let yaw = if let Some(look_dir) = bot.get_component::<LookDirection>() {
                  look_dir.y_rot()
                } else {
                  0.0
                };

                MAP_ACCUMULATOR.add_map_data(
                  &username,
                  map_patch.width as u32,
                  map_patch.height as u32,
                  map_patch.map_colors.clone(),
                  pos.x,
                  pos.z,
                  yaw,
                );

                if MAP_ACCUMULATOR.is_processing(&username) {
                  return Ok(());
                }

                let maps = MAP_ACCUMULATOR.get_maps(&username).unwrap_or_default();

                if maps.len()
                  < (options.captcha_bypass.number_of_columns
                    * options.captcha_bypass.number_of_rows) as usize
                {
                  return Ok(());
                }

                MAP_ACCUMULATOR.set_processing(&username, true);

                if let Some(combined_base64) = MAP_ACCUMULATOR.combine_all(&username) {
                  PROFILES.set_bool(&username, "captcha_caught", true);
                  MAP_ACCUMULATOR.update_captcha_time(&username);

                  if options.basic.use_webhook && options.webhook.send_information {
                    send_webhook(
                      options.webhook.url,
                      format!("Бот {} получил капчу с карты", username),
                    );
                  }

                  send_log(
                    format!("[ Анти-Капча ]: Бот {} получил капчу с карты", username),
                    "info",
                  );

                  if options.captcha_bypass.solve_mode == "auto" {
                    MAP_CAPTCHA_BYPASS.solve_captcha(
                      bot.name(),
                      combined_base64,
                      options.captcha_bypass,
                    );
                  } else {
                    send_optional_event(OptionalEmitEvent::AntiMapCaptcha(
                      AntiMapCaptchaEventPayload {
                        base64_code: combined_base64,
                        username: username.to_string(),
                      },
                    ));
                  }

                  MAP_ACCUMULATOR.clear_maps(&username);
                } else {
                  MAP_ACCUMULATOR.clear_maps(&username);
                }
              } else {
                let base64_code = MAP_CAPTCHA_BYPASS.create_png_image(
                  map_patch.width as u32,
                  map_patch.height as u32,
                  &map_patch.map_colors,
                );

                PROFILES.set_bool(&username, "captcha_caught", true);
                MAP_ACCUMULATOR.update_captcha_time(&username);

                if options.basic.use_webhook && options.webhook.send_information {
                  send_webhook(
                    options.webhook.url,
                    format!("Бот {} получил капчу с карты", username),
                  );
                }

                send_log(
                  format!("[ Анти-Капча ]: Бот {} получил капчу с карты", username),
                  "info",
                );

                if options.captcha_bypass.solve_mode == "auto" {
                  MAP_CAPTCHA_BYPASS.solve_captcha(bot.name(), base64_code, options.captcha_bypass);
                } else {
                  send_optional_event(OptionalEmitEvent::AntiMapCaptcha(
                    AntiMapCaptchaEventPayload {
                      base64_code,
                      username: username.to_string(),
                    },
                  ));
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
