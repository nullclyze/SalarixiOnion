use discord_presence::Client;
use once_cell::sync::Lazy;
use std::sync::{Arc, RwLock};

use crate::api::emit::send_log;

pub static DISCORD_RPC_MANAGER: Lazy<Arc<RwLock<DiscordRpcManager>>> = Lazy::new(|| Arc::new(RwLock::new(DiscordRpcManager::new())));

pub struct DiscordRpcManager {
  client: Option<Client>,
}

impl DiscordRpcManager {
  pub fn new() -> Self {
    Self { client: None }
  }

  pub fn enable(&mut self, version: String) {
    self.client = Some(Client::new(1477312950271213729));

    if let Some(client) = self.client.as_mut() {
      client.start();
      let _ = client.block_until_event(discord_presence::Event::Ready);

      match client.set_activity(|act| act.details(format!("Версия: {}", version)).state("Сайт: https://salarixi.wuaze.com/")) {
        Ok(_) => {}
        Err(err) => {
          send_log(format!("Ошибка включения Discord RPC: {}", err), "error");
        }
      }
    }
  }

  pub fn disable(&mut self) {
    if let Some(client) = self.client.take() {
      match client.shutdown() {
        Ok(_) => {}
        Err(err) => {
          send_log(format!("Ошибка выключения Discord RPC: {}", err), "error");
        }
      }
    }
  }
}
