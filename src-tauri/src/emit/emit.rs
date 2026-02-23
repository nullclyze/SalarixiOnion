use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::{AppHandle, Emitter};
use tokio::sync::broadcast;

pub static EMIT_MANAGER: Lazy<Arc<EmitManager>> = Lazy::new(|| Arc::new(EmitManager::new()));

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LogPayload {
  pub text: String,
  pub class: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MessagePayload {
  pub name: String,
  pub content: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatEventPayload {
  pub receiver: String,
  pub message: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MapRenderProgressEventPayload {
  pub nickname: String,
  pub progress: i32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AntiWebCaptchaEventPayload {
  pub captcha_url: String,
  pub nickname: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AntiMapCaptchaEventPayload {
  pub base64_code: String,
  pub nickname: String,
}

pub struct EmitManager {
  events: broadcast::Sender<EmitEvent>,
}

#[derive(Clone)]
pub enum EmitEvent {
  Log { text: String, class: String },
  Message { name: String, content: String },
  Optional(OptionalEmitEvent),
}

#[derive(Clone)]
pub enum OptionalEmitEvent {
  Chat(ChatEventPayload),
  MapRenderProgress(MapRenderProgressEventPayload),
  AntiWebCaptcha(AntiWebCaptchaEventPayload),
  AntiMapCaptcha(AntiMapCaptchaEventPayload),
}

impl EmitManager {
  pub fn new() -> Self {
    let (tx, _) = broadcast::channel(1000);

    Self { events: tx }
  }

  pub fn send_event(&self, event: EmitEvent) {
    let _ = self.events.send(event);
  }
}

/// Вспомогательная функция отправки лога
pub fn send_log(text: String, class: &str) {
  EMIT_MANAGER.send_event(EmitEvent::Log {
    text,
    class: class.to_string(),
  });
}

/// Вспомогательная функция отправки сообщения
pub fn send_message(name: &str, content: String) {
  EMIT_MANAGER.send_event(EmitEvent::Message {
    name: name.to_string(),
    content,
  });
}

/// Вспомогательная функция отправки дополнительных событий
pub fn send_optional_event(event: OptionalEmitEvent) {
  match event {
    OptionalEmitEvent::Chat(payload) => {
      EMIT_MANAGER.send_event(EmitEvent::Optional(OptionalEmitEvent::Chat(payload)));
    }
    OptionalEmitEvent::MapRenderProgress(payload) => {
      EMIT_MANAGER.send_event(EmitEvent::Optional(OptionalEmitEvent::MapRenderProgress(
        payload,
      )));
    }
    OptionalEmitEvent::AntiWebCaptcha(payload) => {
      EMIT_MANAGER.send_event(EmitEvent::Optional(OptionalEmitEvent::AntiWebCaptcha(
        payload,
      )));
    }
    OptionalEmitEvent::AntiMapCaptcha(payload) => {
      EMIT_MANAGER.send_event(EmitEvent::Optional(OptionalEmitEvent::AntiMapCaptcha(
        payload,
      )));
    }
  }
}

pub async fn emit_event_loop(handle: AppHandle) {
  let mut rx = EMIT_MANAGER.events.subscribe();

  while let Ok(event) = rx.recv().await {
    match event {
      EmitEvent::Log { text, class } => {
        let _ = handle.emit("log", LogPayload { text, class });
      }
      EmitEvent::Message { name, content } => {
        let _ = handle.emit("message", MessagePayload { name, content });
      }
      EmitEvent::Optional(optional_event) => match optional_event {
        OptionalEmitEvent::Chat(payload) => {
          let _ = handle.emit("chat-message", payload);
        }
        OptionalEmitEvent::MapRenderProgress(payload) => {
          let _ = handle.emit("map-render-progress", payload);
        }
        OptionalEmitEvent::AntiWebCaptcha(payload) => {
          let _ = handle.emit("anti-web-captcha", payload);
        }
        OptionalEmitEvent::AntiMapCaptcha(payload) => {
          let _ = handle.emit("anti-map-captcha", payload);
        }
      },
    }
  }
}
