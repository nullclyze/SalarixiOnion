use serde::{Serialize, Deserialize};
use tauri::Emitter;

use crate::base::get_flow_manager;


#[derive(Debug, Clone)]
pub enum EventType {
  Log(LogEventPayload),
  Chat(ChatEventPayload)
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

// Функция отправки события
pub fn emit_event(event: EventType) {
  if let Some(arc) = get_flow_manager() {
    let fm = arc.read();

    if fm.app_handle.is_some() {
      match event {
        EventType::Log(payload) => {
          fm.app_handle.as_ref().unwrap().emit("log", payload).unwrap();
        },
        EventType::Chat(payload) => {
          fm.app_handle.as_ref().unwrap().emit("chat", payload).unwrap();
        }
      }
    }
  } 
}