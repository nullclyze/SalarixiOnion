use azalea::entity::metadata::Health;
use azalea::local_player::Hunger;
use azalea::protocol::common::client_information::ParticleStatus;
use azalea::protocol::packets::game::ClientboundGamePacket;
use azalea::swarm::*;
use azalea::prelude::*;
use azalea::entity::HumanoidArm;
use azalea::{ClientInformation, NoState};
use tokio::time::sleep;
use std::time::Duration;

use crate::tools::{randuint, randelem, randchance};
use crate::state::{STATES, BotState};
use crate::tasks::TASKS;
use crate::base::*;
use crate::emit::*;
use crate::{AntiWebCaptcha, AntiMapCaptcha};


const ACCOUNTS_WITH_SKINS: &[&str] = &[
  "ordunury", "tqyd", "rittes", "Kalmdel", "rlyy", "tusxi", "hrfi", "seree", "NewGuyCri", "UnityForsaken",
  "B0RAA", "cigarist", "fumbled", "_Malrand_"
];


// Swarm-обработчик
pub async fn swarm_handler(swarm: Swarm, event: SwarmEvent, _state: NoSwarmState) {
  match event {
    SwarmEvent::Init => {
      if let Some(arc) = get_flow_manager() {
        let mut fm = arc.write();

        fm.swarm = Some(swarm.clone());
      }
    },
    _ => {}
  }
}

// Single-обработчик
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
        message: format!("Бот {} подключается...", nickname)
      }));
                
      if let Some(state) = STATES.get(&nickname) {
        STATES.set(&nickname, "status", "Соединение...".to_string());

        if let Some(arc) = get_flow_manager() {
          let fm = arc.write();

          if let Some(options) = &fm.options {
            if state.read().get_string("password").is_none() {
              STATES.set(&nickname, "password", generate_nickname_or_password("password", options.password_type.clone(), options.password_template.clone()));
            }
          }
        }  
      } else {
        if let Some(arc) = get_flow_manager() {
          let fm = arc.write();
          
          if let Some(options) = &fm.options {
            STATES.add(&nickname, BotState::new(nickname.clone(), generate_nickname_or_password("password", options.password_type.clone(), options.password_template.clone()), options.version.clone()));
          }
        }
      }
    },
    Event::Spawn => {
      let nickname = bot.username();

      update_bots_count('+');

      if let Some(arc) = get_flow_manager() {
        if let Some(options) = arc.read().options.clone() {
          if options.plugins.auto_armor {
            AutoArmorPlugin::enable(bot.clone());
          }

          if options.plugins.auto_totem {
            AutoTotemPlugin::enable(bot.clone());
          }

          if options.plugins.auto_eat {
            AutoEatPlugin::enable(bot.clone());
          }
        }
      }

      TASKS.add(&nickname);

      emit_event(EventType::Log(LogEventPayload { 
        name: "info".to_string(), 
        message: format!("Бот {} заспавнился", &nickname)
      }));

      if let Some(arc) = get_flow_manager() {
        STATES.set(&nickname, "status", "Онлайн".to_string());

        let options = arc.write().options.clone();

        if let Some(opts) = options {
          let mut min_delay = 2000;
          let mut max_delay = 4000;

          let mut c = "";
          let mut template = "@cmd @pass".to_string();

          let mut action = "".to_string();

          if let Some(state) = STATES.get(&nickname) {
            if let Some(registered) = state.read().get_bool("registered") {
              if !registered {
                c = opts.register_command.as_str().trim();
                template = opts.register_template.trim().to_string();
                min_delay = opts.register_min_delay;
                max_delay = opts.register_max_delay;

                STATES.set(&nickname, "registered", "true".to_string());

                action = "зарегистрировался".to_string();
              } else {
                c = opts.login_command.as_str().trim();
                template = opts.login_template.trim().to_string();
                min_delay = opts.login_min_delay;
                max_delay = opts.login_max_delay;

                action = "залогинился".to_string();
              }
            }

            sleep(Duration::from_millis(randuint(min_delay, max_delay))).await;
              
            let cmd = template.clone()
              .replace("@cmd",c)
              .replace("@pass", state.read().get_string("password").unwrap());

            bot.chat(&cmd);

            emit_event(EventType::Log(LogEventPayload { 
              name: "info".to_string(),
              message: format!("Бот {} {}: {}", &nickname, action, &cmd)
            }));
          }
        }
      }

      if let Some(state) = STATES.get(&nickname) {
        if let Some(skin_is_set) = state.read().get_bool("skin_is_set") {
          if !skin_is_set {
            if let Some(arc) = get_flow_manager() {
              let fm = arc.write();

              if let Some(opts) = fm.options.clone() {
                match opts.skin_settings.skin_type.as_str() {
                  "random" => {
                    let bot_clone = bot.clone();

                    tokio::spawn(async move {
                      let command = format!("{} {}", opts.skin_settings.set_skin_command.unwrap_or("/skin".to_string()), randelem(ACCOUNTS_WITH_SKINS).unwrap());

                      bot_clone.chat(command.clone());

                      sleep(Duration::from_millis(3000)).await;
                      
                      emit_event(EventType::Log(LogEventPayload { 
                        name: "system".to_string(), 
                        message: format!("Бот {} успешно установил скин: {}", bot_clone.username(), command)
                      }));

                      bot_clone.disconnect();
                    });

                    STATES.set(&nickname, "skin_is_set", "true".to_string());
                  },
                  "custom" => {
                    if let Some(n) = opts.skin_settings.custom_skin_by_nickname {
                      let bot_clone = bot.clone();

                      tokio::spawn(async move {
                        let command = format!("{} {}", opts.skin_settings.set_skin_command.unwrap_or("/skin".to_string()), n);

                        bot_clone.chat(command.clone());

                        sleep(Duration::from_millis(3000)).await;
                        
                        emit_event(EventType::Log(LogEventPayload { 
                          name: "system".to_string(), 
                          message: format!("Бот {} успешно установил скин: {}", bot_clone.username(), command)
                        }));

                        bot_clone.disconnect();
                      });

                      STATES.set(&nickname, "skin_is_set", "true".to_string());
                    }
                  },
                  _ => {}
                }
              }
            }
          }
        }
      }
    },
    Event::Disconnect(packet) => {
      let nickname = bot.username();

      if let Some(arc) = get_flow_manager() {
        let mut fm = arc.write();
        fm.bots.remove(&nickname);
      }  

      update_bots_count('-');

      if let Some(tasks) = TASKS.get(&nickname) {
        tasks.write().unwrap().stop_all_tasks();
      }

      if let Some(arc) = STATES.get(&nickname) {
        let mut state = arc.write();
        state.set_captcha_caught(false);
      }

      TASKS.remove(&nickname);

      STATES.set(&nickname, "status", "Оффлайн".to_string());

      if let Some(text) = packet {
        emit_event(EventType::Log(LogEventPayload { 
          name: "info".to_string(),
          message: format!("Бот {} отключился: {}", &nickname, text.to_html())
        }));
      }
    },
    Event::Chat(packet) => {
      let nickname = bot.username();

      let sender = packet.sender().unwrap_or("unknown".to_string());

      let mut message = packet.message().to_html();

      if let Some(arc) = get_flow_manager() {
        for bot in arc.write().bots.clone().into_values() {
          if bot.username() == sender {
            message = format!("%hbБот%sc ~ {}", message);
            break;
          }
        }
      }

      emit_event(EventType::Chat(ChatEventPayload { 
        receiver: nickname.clone(),
        message: message
      }));

      if let Some(arc) = get_flow_manager() {
        if let Some(options) = arc.read().options.clone() {
          if options.use_anti_captcha {
            if let Some(arc) = STATES.get(&nickname) {
              let mut state = arc.write();

              if let Some(captcha_caught) = state.get_bool("captcha_caught") {
                if !captcha_caught {
                  let opts = options.anti_captcha_settings.options.web.clone();

                  if options.anti_captcha_settings.captcha_type.as_str() == "web" {
                    if let Some(url) = AntiWebCaptcha::catch_url_from_message(packet.message().to_string(), opts.regex.as_str(), opts.required_url_part) {
                      state.set_captcha_caught(true);

                      emit_event(EventType::AntiWebCaptcha(AntiWebCaptchaEventPayload {  
                        captcha_url: url,
                        nickname: nickname.to_string()
                      }));
                    }
                  }
                }
              }
            }
          }
        }
      }
    },
    Event::Tick => {
      let nickname = bot.username();

      if let Some(state) = STATES.get(&nickname) {
        if let Some(status) = state.read().get_string("status") {
          if status.to_lowercase() == "онлайн" {
            if let Some(health) = bot.get_component::<Health>() {
              STATES.set(&nickname, "health", health.to_string());
            }

            if let Some(hunger) = bot.get_component::<Hunger>() {
              STATES.set(&nickname, "satiety", hunger.food.to_string());
            }
          }
        }
      }
    },
    Event::Packet(packet) => match &*packet {
      ClientboundGamePacket::MapItemData(data) => {
        let nickname = bot.username();

        if let Some(map_patch) = &data.color_patch.0 {
          if let Some(arc) = STATES.get(&nickname) {
            let mut state = arc.write();

            if let Some(captcha_caught) = state.get_bool("captcha_caught") {
              if !captcha_caught {
                state.set_captcha_caught(true);

                let base64_code = AntiMapCaptcha::create_and_save_png_image(&map_patch.map_colors);

                emit_event(EventType::AntiMapCaptcha(AntiMapCaptchaEventPayload {  
                  base64_code: base64_code,
                  nickname: nickname.to_string()
                }));
              }
            }
          }
        }
      },
      _ => {}
    }
    _ => {}
  }

  Ok(())
}