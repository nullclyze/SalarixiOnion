use serde::{Serialize, Deserialize};
use tauri::Emitter;

use crate::base::get_flow_manager;


#[derive(Debug, Clone)]
pub enum EventType {
  Log(LogEventPayload),
  Chat(ChatEventPayload),
  AntiMapCaptcha(AntiMapCaptchaEventPayload)
}

// Структура данных в log-payload
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogEventPayload {
  pub name: String,
  pub message: String
}

// Структура данных в chat-payload
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatEventPayload {
  pub receiver: String,
  pub message: String
}

// Структура данных в anti-map-captcha-payload
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AntiMapCaptchaEventPayload {
  pub base64_code: String,
  pub nickname: String
}

// Функция отправки события
pub fn emit_event(event: EventType) {
  if let Some(arc) = get_flow_manager() {
    let fm = arc.read();

    if fm.app_handle.is_some() {
      match event {
        EventType::Log(payload) => {
          let _ = fm.app_handle.as_ref().unwrap().emit("log", payload);
        },
        EventType::Chat(payload) => {
          let _ = fm.app_handle.as_ref().unwrap().emit("chat", payload);
        },
        EventType::AntiMapCaptcha(payload) => {
          let _ = fm.app_handle.as_ref().unwrap().emit("anti-map-captcha", payload);
        }
      }
    }
  } 
}