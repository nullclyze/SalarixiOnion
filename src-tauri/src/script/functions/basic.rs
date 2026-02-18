use crate::{
  base::get_current_options,
  emit::{emit_event, emit_message, EventType, LogEventPayload},
  webhook::send_webhook,
};

#[derive(Clone)]
pub struct BasicScriptFunctions;

impl BasicScriptFunctions {
  pub fn print(str: &str) {
    println!("[ script ]: {}", str);
  }

  pub fn log(text: &str, name: &str) {
    emit_event(EventType::Log(LogEventPayload {
      name: name.to_string(),
      message: text.to_string(),
    }));
  }

  pub fn message(content: &str, name: &str) {
    emit_message(name, content.to_string());
  }

  pub fn webhook(content: &str) {
    if let Some(opts) = get_current_options() {
      send_webhook(opts.webhook_settings.url, content.to_string());
    }
  }
}
