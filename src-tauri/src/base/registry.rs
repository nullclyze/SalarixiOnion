use azalea::prelude::*;
use azalea::swarm::Swarm;
use dashmap::DashMap;
use once_cell::sync::Lazy;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

use crate::base::{current_options, MODULE_MANAGER, PLUGIN_MANAGER};
use crate::emit::send_log;
use crate::quick::QUICK_TASK_MANAGER;

pub static BOT_REGISTRY: Lazy<Arc<BotRegistry>> = Lazy::new(|| Arc::new(BotRegistry::new()));

/// Реестр роя, всех ботов и событий.
/// Он выполняет роль основного хранилища всех ботов.
/// Так же данный реестр содержит в себе канал, по которому передаются все основные события для ботов (загрузить плагины, выполнить быструю задачу и так далее).
pub struct BotRegistry {
  pub swarm: Arc<RwLock<Option<Swarm>>>,
  pub bots: Arc<DashMap<String, Arc<RwLock<Option<Client>>>>>,
  pub events: broadcast::Sender<RegistryEvent>,
}

#[derive(Clone)]
pub enum RegistryEvent {
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
  DestroySwarm,
  StopProcessing,
}

impl BotRegistry {
  pub fn new() -> Self {
    let (tx, _) = broadcast::channel(1000);

    Self {
      swarm: Arc::new(RwLock::new(None)),
      bots: Arc::new(DashMap::new()),
      events: tx,
    }
  }

  pub async fn set_swarm(&self, swarm: Swarm) {
    let mut guard = self.swarm.write().await;
    *guard = Some(swarm);
  }

  pub async fn destroy_swarm(&self) {
    if let Some(swarm) = self.swarm.write().await.take() {
      swarm.ecs_lock.lock().write_message(AppExit::Success);
    }
  }

  pub fn register_bot(&self, username: &str, bot: Client) {
    self
      .bots
      .insert(username.to_string(), Arc::new(RwLock::new(Some(bot))));
  }

  pub async fn take_bot(&self, username: &str) -> Option<Client> {
    let (_, cell) = self.bots.remove(username)?;
    let mut guard = cell.write().await;
    guard.take()
  }

  pub fn remove_bot(&self, username: &str) {
    self.bots.remove(username);
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

  pub fn send_event(&self, event: RegistryEvent) {
    let _ = self.events.send(event);
  }

  pub fn clear(&self) {
    self.bots.clear();
    self.send_event(RegistryEvent::DestroySwarm);
    self.send_event(RegistryEvent::StopProcessing);

    send_log(format!("Реестр ботов очищен"), "extended");
  }
}

/// Функция процессинга всех событий из `RegistryEvent`
pub async fn registry_event_loop() {
  let mut rx = BOT_REGISTRY.events.subscribe();

  while let Ok(event) = rx.recv().await {
    match event {
      RegistryEvent::LoadPlugins { username } => {
        if let Some(opts) = current_options() {
          let _ = BOT_REGISTRY
            .get_bot(&username, async |_| {
              PLUGIN_MANAGER.load(&username, &opts.plugins);
            })
            .await;
        }
      }
      RegistryEvent::ControlModules {
        name,
        options,
        group,
      } => {
        MODULE_MANAGER.control(name, options, group).await;
      }
      RegistryEvent::QuickTask { name } => {
        QUICK_TASK_MANAGER.execute(name);
      }
      RegistryEvent::DestroySwarm => {
        BOT_REGISTRY.destroy_swarm().await;
      }
      RegistryEvent::StopProcessing => {
        return;
      }
    }
  }
}
