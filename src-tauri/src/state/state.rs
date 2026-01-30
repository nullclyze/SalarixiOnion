use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};


pub static STATES: Lazy<Arc<BotStateManager>> = Lazy::new(|| Arc::new(BotStateManager::new()));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotState {
  pub status: String,
  pub nickname: String,
  pub version: String,
  pub password: String,
  pub proxy: String,
  pub health: u32,
  pub satiety: u32,
  pub registered: bool,
  pub skin_is_set: bool,
  pub captcha_caught: bool,
  pub group: String,
  pub can_walk: bool,
  pub plugin_activity: PluginActivity
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginActivity {
  pub auto_armor: bool,
  pub auto_totem: bool,
  pub auto_eat: bool,
  pub auto_potion: bool,
  pub auto_look: bool,
  pub auto_shield: bool
}

impl BotState {
  pub fn new(nickname: String, password: String, version: String) -> Self {
    Self {
      status: "Соединение...".to_string(),
      nickname: nickname,
      version: version,
      password: password,
      proxy: "-".to_string(),
      health: 0,
      satiety: 0,
      registered: false,
      skin_is_set: false,
      captcha_caught: false,
      group: "global".to_string(),
      can_walk: true,
      plugin_activity: PluginActivity { 
        auto_armor: false, 
        auto_totem: false, 
        auto_eat: false, 
        auto_potion: false,
        auto_look: false,
        auto_shield: false
      }
    }
  }

  pub fn set_status(&mut self, status: &str) {
    self.status = status.to_string();
  }

  pub fn set_password(&mut self, password: &str) {
    self.password = password.to_string();
  }

  pub fn set_proxy(&mut self, proxy: &str) {
    self.proxy = proxy.to_string();
  }

  pub fn set_health(&mut self, health: u32) {
    self.health = health;
  }

  pub fn set_satiety(&mut self, satiety: u32) {
    self.satiety = satiety;
  }

  pub fn set_registered(&mut self, state: bool) {
    self.registered = state;
  }

  pub fn set_skin_is_set(&mut self, state: bool) {
    self.skin_is_set = state;
  }

  pub fn set_captcha_caught(&mut self, state: bool) {
    self.captcha_caught = state;
  }

  pub fn set_group(&mut self, group: String) {
    self.group = group;
  }

  pub fn set_can_walk(&mut self, state: bool) {
    self.can_walk = state;
  }
}

pub struct BotStateManager {
  pub states: RwLock<HashMap<String, Arc<RwLock<BotState>>>>,
}

impl BotStateManager {
  pub fn new() -> Self {
    Self {
      states: RwLock::new(HashMap::new()),
    }
  }

  pub fn add(&self, nickname: &String, state: BotState) -> Arc<RwLock<BotState>> {
    let arc_state = Arc::new(RwLock::new(state));
    let mut states = self.states.write().unwrap();
    states.insert(nickname.clone(), arc_state.clone());
    arc_state
  }

  pub fn get(&self, nickname: &String) -> Option<Arc<RwLock<BotState>>> {
    let states = self.states.read().unwrap();
    states.get(nickname).cloned()
  }

  pub fn set(&self, nickname: &String, field: &str, value: String) {
    if let Some(arc) = self.states.write().unwrap().get(nickname) {
      let mut state = arc.write().unwrap();

      match field {
        "status" => state.set_status(&value),
        "password" => state.set_password(&value),
        "proxy" => state.set_proxy(&value),
        "health" => state.set_health(value.parse().unwrap()),
        "satiety" => state.set_satiety(value.parse().unwrap()),
        "registered" => state.set_registered(value.parse().unwrap()),
        "skin_is_set" => state.set_skin_is_set(value.parse().unwrap()),
        "captcha_caught" => state.set_captcha_caught(value.parse().unwrap()),
        "group" => state.set_group(value.parse().unwrap()),
        "can_walk" => state.set_can_walk(value.parse().unwrap()),
        _ => {}
      }
    }
  }

  pub fn get_plugin_activity(&self, nickname: &String, plugin: &str) -> bool {
    if let Some(arc) = self.states.read().unwrap().get(nickname) {
      let state = arc.read().unwrap();

      match plugin {
        "auto-armor" => return state.plugin_activity.auto_armor,
        "auto-totem" => return state.plugin_activity.auto_totem,
        "auto-eat" => return state.plugin_activity.auto_eat,
        "auto-potion" => return state.plugin_activity.auto_potion,
        "auto-look" => return state.plugin_activity.auto_look,
        "auto-shield" => return state.plugin_activity.auto_shield,
        _ => {}
      }
    }

    false
  }

  pub fn set_plugin_activity(&self, nickname: &String, plugin: &str, value: bool) {
    if let Some(arc) = self.states.write().unwrap().get(nickname) {
      let mut state = arc.write().unwrap();

      match plugin {
        "auto-armor" => {
          state.plugin_activity.auto_armor = value;
        },
        "auto-totem" => {
          state.plugin_activity.auto_totem = value;
        },
        "auto-eat" => {
          state.plugin_activity.auto_eat = value;
        },
        "auto-potion" => {
          state.plugin_activity.auto_potion = value;
        },
        "auto-look" => {
          state.plugin_activity.auto_look = value;
        },
        "auto-shield" => {
          state.plugin_activity.auto_shield = value;
        },
        _ => {}
      }
    }
  }

  pub fn clear(&self) {
    let mut states = self.states.write().unwrap();
    states.clear();
  }

  pub fn can_walk(&self, nickname: &String) -> bool {
    if let Some(arc) = self.states.read().unwrap().get(nickname) {
      return arc.read().unwrap().can_walk;
    }

    true
  }

  pub fn get_all_profiles(&self) -> HashMap<String, BotState> {
    let states = self.states.read().unwrap();
    let mut profiles = HashMap::new();

    for (nickname, state) in states.iter() {
      if let Ok(state_guard) = state.read() {
        profiles.insert(nickname.clone(), state_guard.clone());
      }
    }

    profiles
  }
}