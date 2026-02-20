use crate::{
  base::current_options,
  emit::{send_log, send_message},
  webhook::send_webhook,
};

#[derive(Clone)]
pub struct BasicScriptFunctions;

impl BasicScriptFunctions {
  pub fn print(str: &str) {
    println!("[ script ]: {}", str);
  }

  pub fn log(text: &str, class: &str) {
    send_log(text.to_string(), class);
  }

  pub fn message(name: &str, content: &str) {
    send_message(name, content.to_string());
  }

  pub fn webhook(content: &str) {
    if let Some(opts) = current_options() {
      send_webhook(opts.webhook_settings.url, content.to_string());
    }
  }
}
