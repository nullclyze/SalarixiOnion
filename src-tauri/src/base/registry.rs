use azalea::prelude::*;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

pub static BOT_REGISTRY: Lazy<Arc<BotRegistry>> = Lazy::new(|| Arc::new(BotRegistry::new()));

// Реестр всех ботов и событий
pub struct BotRegistry {
  pub bots: Arc<DashMap<String, Arc<RwLock<Option<Client>>>>>,
  pub events: broadcast::Sender<BotEvent>,
}

#[derive(Clone)]
pub enum BotEvent {
  LoadPlugins {
    username: String,
  },
  ControlModules {
    name: String,
    options: serde_json::Value,
    group: String,
  },
  QuickTask {
    name: String,
  },
}

impl BotRegistry {
  pub fn new() -> Self {
    let (tx, _) = broadcast::channel(1000);

    Self {
      bots: Arc::new(DashMap::new()),
      events: tx,
    }
  }

  pub fn register_bot(&self, username: &str, bot: Client) {
    self
      .bots
      .insert(username.to_string(), Arc::new(RwLock::new(Some(bot))));
  }
  
  pub async fn remove_bot(&self, username: &str) -> Option<Client> {
    let (_, cell) = self.bots.remove(username)?;
    let mut guard = cell.write().await;
    guard.take()
  }

  pub fn destroy(&self) {
    self.bots.clear();
  }

  pub async fn get_bot<F, T>(&self, username: &str, f: F) -> Option<T>
  where
    F: AsyncFnOnce(&Client) -> T,
  {
    if let Some(reference) = self.bots.get(username) {
      if let Some(bot) = reference.read().await.as_ref() {
        return Some(f(bot).await);
      }
    }

    None
  }

  pub fn send_event(&self, event: BotEvent) {
    let _ = self.events.send(event);
  }
}
