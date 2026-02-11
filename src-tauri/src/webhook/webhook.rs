use reqwest;
use serde::{Deserialize, Serialize};

use crate::emit::*;

#[derive(Serialize, Deserialize)]
struct Webhook {
  content: String,
}

pub fn send_webhook(webhook_url: Option<String>, content: String) {
  tokio::spawn(async move {
    if let Some(url) = webhook_url {
      let msg = Webhook { content: content };

      let client = reqwest::Client::new();

      let res = client.post(url).json(&msg).send().await;

      match res {
        Ok(r) => {
          if !r.status().is_success() {
            emit_event(EventType::Log(LogEventPayload {
              name: "error".to_string(),
              message: format!("Не удалось отправить webhook: Status code {}", r.status()),
            }));
          }
        }
        Err(err) => {
          emit_event(EventType::Log(LogEventPayload {
            name: "error".to_string(),
            message: format!("Не удалось отправить webhook: {}", err),
          }));
        }
      }
    }
  });
}
