use serde::{Deserialize, Serialize};
use tauri::Emitter;

use crate::base::get_flow_manager;

// Структура данных в log-payload
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogPayload {
  pub text: String,
  pub class: String,
}

// Структура данных в message-payload
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessagePayload {
  pub name: String,
  pub content: String,
}

#[derive(Debug, Clone)]
pub enum PayloadEvent {
  Chat(ChatEventPayload),
  MapRenderProgress(MapRenderProgressEventPayload),
  AntiWebCaptcha(AntiWebCaptchaEventPayload),
  AntiMapCaptcha(AntiMapCaptchaEventPayload),
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

// Функция отправки лога
pub fn send_log(text: String, class: &str) {
  if let Some(arc) = get_flow_manager() {
    let fm = arc.read();

    if let Some(handle) = fm.app_handle.as_ref() {
      let _ = handle.emit(
        "log",
        LogPayload {
          text: text,
          class: class.to_string(),
        },
      );
    }
  }
}

// Функция отправки сообщения
pub fn send_message(name: &str, content: String) {
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

// Функция отправки события
pub fn send_event(event: PayloadEvent) {
  if let Some(arc) = get_flow_manager() {
    let fm = arc.read();

    if let Some(handle) = fm.app_handle.as_ref() {
      match event {
        PayloadEvent::Chat(payload) => {
          let _ = handle.emit("chat-message", payload);
        }
        PayloadEvent::MapRenderProgress(payload) => {
          let _ = handle.emit("map-render-progress", payload);
        }
        PayloadEvent::AntiWebCaptcha(payload) => {
          let _ = handle.emit("anti-web-captcha", payload);
        }
        PayloadEvent::AntiMapCaptcha(payload) => {
          let _ = handle.emit("anti-map-captcha", payload);
        }
      }
    }
  }
}
