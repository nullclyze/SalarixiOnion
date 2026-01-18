use azalea::swarm::*;
use azalea::prelude::*;
use azalea::entity::HumanoidArm;
use azalea::{ClientInformation, NoState};
use tokio::time::sleep;
use std::time::Duration;

use crate::tools::{randuint};
use crate::state::{STATES, BotState};
use crate::tasks::TASKS;
use crate::base::get_flow_manager;
use crate::base::generate_nickname_or_password;
use crate::base::update_bots_count;
use crate::extract_link_from_message;
use crate::emit::*;


// Swarm-обработчик
pub async fn swarm_handler(swarm: Swarm, event: SwarmEvent, _state: NoSwarmState) {
  match event {
    SwarmEvent::Init => {
      if let Some(arc) = get_flow_manager() {
        let mut fm = arc.write();

        fm.swarm = Some(swarm);
      }
    },
    _ => {}
  }
}

// Single-обработчик
pub async fn single_handler(bot: Client, event: Event, _: NoState) -> anyhow::Result<()> {
  match event {
    Event::Login => {
      let nickname = bot.username();

      if let Some(arc) = get_flow_manager() {
        let mut fm = arc.write();
        fm.bots.insert(nickname.clone(), bot);
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
            if state.read().unwrap().password.is_empty() {
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

      TASKS.add(&nickname);

      emit_event(EventType::Log(LogEventPayload { 
        name: "info".to_string(), 
        message: format!("Бот {} заспавнился", &nickname)
      }));

      if let Some(arc) = get_flow_manager() {
        STATES.set(&nickname, "status", "Онлайн".to_string());

        let options = arc.write().options.clone();

        if let Some(opts) = options {
          bot.set_client_information(ClientInformation {
            view_distance: opts.view_distance,
            language: opts.language,
            chat_colors: opts.chat_colors,
            main_hand: if opts.humanoid_arm.to_lowercase().as_str() == "left" { HumanoidArm::Left } else { HumanoidArm::Right },
            ..Default::default()
          });

          let min_delay;
          let max_delay;

          let c;
          let template;

          let action;

          if let Some(state) = STATES.get(&nickname) {
            if !state.read().unwrap().registered {
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

            sleep(Duration::from_millis(randuint(min_delay, max_delay))).await;
              
            let cmd = template.clone()
              .replace("@cmd",c)
              .replace("@pass", &state.read().unwrap().password.clone());

            bot.chat(&cmd);

            emit_event(EventType::Log(LogEventPayload { 
              name: "info".to_string(),
              message: format!("Бот {} {}: {}", &nickname, action, &cmd)
            }));
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

      if let Some(link) = extract_link_from_message(packet.message().to_string()) {
        if let Some(state) = STATES.get(&nickname) {
          if state.read().unwrap().captcha_url.is_none() {
            STATES.set(&nickname, "captcha_url", link);
          }
        }
      }
    },
    _ => {}
  }

  Ok(())
}