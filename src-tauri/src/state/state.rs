use std::collections::HashMap;
use std::sync::Arc;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use parking_lot::RwLock;


pub static STATES: Lazy<Arc<BotStateManager>> = Lazy::new(|| Arc::new(BotStateManager::new()));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotState {
  status: String,
  nickname: String,
  password: String,
  version: String,
  proxy: String,
  health: u32,
  satiety: u32,
  registered: bool,
  skin_is_set: bool,
  captcha_caught: bool
}

impl BotState {
  pub fn new(nickname: String, password: String, version: String) -> Self {
    Self {
      status: "Соединение...".to_string(),
      nickname: nickname,
      password: password,
      version: version,
      proxy: "-".to_string(),
      health: 0,
      satiety: 0,
      registered: false,
      skin_is_set: false,
      captcha_caught: false
    }
  }

  pub fn get_string(&self, field: &str) -> Option<&String> {
    match field {
      "status" => return Some(&self.status),
      "nickname" => return Some(&self.nickname),
      "password" => return Some(&self.password),
      "version" => return Some(&self.version),
      "proxy" => return Some(&self.proxy),
      _ => {}
    }

    None
  }

  pub fn get_number(&self, field: &str) -> Option<u32> {
    match field {
      "health" => return Some(self.health),
      "satiety" => return Some(self.satiety),
      _ => {}
    }

    None
  }

  pub fn get_bool(&self, field: &str) -> Option<bool> {
    match field {
      "registered" => return Some(self.registered),
      "skin_is_set" => return Some(self.skin_is_set),
      "captcha_caught" => return Some(self.captcha_caught),
      _ => {}
    }

    None
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

  pub fn set_health(&mut self, health: &u32) {
    self.health = *health;
  }

  pub fn set_satiety(&mut self, satiety: &u32) {
    self.satiety = *satiety;
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
    let mut states = self.states.write();
    states.insert(nickname.clone(), arc_state.clone());
    arc_state
  }

  pub fn get(&self, nickname: &String) -> Option<Arc<RwLock<BotState>>> {
    let states = self.states.read();
    states.get(nickname).cloned()
  }

  pub fn set(&self, nickname: &String, field: &str, value: String) {
    if let Some(arc) = self.states.write().get(nickname) {
      let mut state = arc.write();

      match field {
        "status" => state.set_status(&value),
        "password" => state.set_password(&value),
        "proxy" => state.set_proxy(&value),
        "health" => state.set_health(&value.parse().unwrap()),
        "satiety" => state.set_satiety(&value.parse().unwrap()),
        "registered" => state.set_registered(value.parse().unwrap()),
        "skin_is_set" => state.set_skin_is_set(value.parse().unwrap()),
        "captcha_caught" => state.set_captcha_caught(value.parse().unwrap()),
        _ => {}
      }
    }
  }

  pub fn clear(&self) {
    let mut states = self.states.write();
    states.clear();
  }

  pub fn get_all_profiles(&self) -> HashMap<String, BotState> {
    let states = self.states.read();
    let mut profiles = HashMap::new();

    for (nickname, state) in states.iter() {
      profiles.insert(nickname.clone(), state.read().clone());
    }

    profiles
  }
}