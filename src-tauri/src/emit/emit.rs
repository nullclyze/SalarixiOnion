use serde::{Deserialize, Serialize};
use tauri::Emitter;

use crate::base::get_flow_manager;

#[derive(Debug, Clone)]
pub enum EventType {
  Log(LogEventPayload),
  Chat(ChatEventPayload),
  MapRenderProgress(MapRenderProgressEventPayload),
  AntiWebCaptcha(AntiWebCaptchaEventPayload),
  AntiMapCaptcha(AntiMapCaptchaEventPayload),
}

// Структура данных в log-payload
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogEventPayload {
  pub name: String,
  pub message: String,
}

// Структура данных в chat-payload
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatEventPayload {
  pub receiver: String,
  pub message: String,
}

// Структура данных в chat-payload
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MapRenderProgressEventPayload {
  pub nickname: String,
  pub progress: i32,
}

// Структура данных в anti-web-captcha-payload
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AntiWebCaptchaEventPayload {
  pub captcha_url: String,
  pub nickname: String,
}

// Структура данных в anti-map-captcha-payload
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AntiMapCaptchaEventPayload {
  pub base64_code: String,
  pub nickname: String,
}

// Структура данных в message-payload
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessagePayload {
  pub name: String,
  pub content: String,
}

// Функция отправки события
pub fn emit_event(event: EventType) {
  if let Some(arc) = get_flow_manager() {
    let fm = arc.read();

    if let Some(handle) = fm.app_handle.as_ref() {
      match event {
        EventType::Log(payload) => {
          let _ = handle.emit("log", payload);
        }
        EventType::Chat(payload) => {
          let _ = handle.emit("chat-message", payload);
        }
        EventType::MapRenderProgress(payload) => {
          let _ = handle.emit("map-render-progress", payload);
        }
        EventType::AntiWebCaptcha(payload) => {
          let _ = handle.emit("anti-web-captcha", payload);
        }
        EventType::AntiMapCaptcha(payload) => {
          let _ = handle.emit("anti-map-captcha", payload);
        }
      }
    }
  }
}

// Функция отправки сообщения
pub fn emit_message(name: &str, content: String) {
  if let Some(arc) = get_flow_manager() {
    let fm = arc.read();

    if let Some(handle) = fm.app_handle.as_ref() {
      let _ = handle.emit(
        "message",
        MessagePayload {
          name: name.to_string(),
          content: content,
        },
      );
    }
  }
}
