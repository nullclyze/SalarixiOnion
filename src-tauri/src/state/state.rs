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
  pub registered: bool,
  pub captcha_url: Option<String>
}

impl BotState {
  pub fn new(nickname: String, password: String, version: String) -> Self {
    Self {
      status: "Соединение...".to_string(),
      nickname: nickname,
      version: version,
      password: password,
      proxy: "-".to_string(),
      registered: false,
      captcha_url: None
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

  pub fn set_registered(&mut self, registered: bool) {
    self.registered = registered;
  }

  pub fn set_captcha_url(&mut self, url: String) {
    self.captcha_url = Some(url);
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
        "registered" => state.set_registered(value.parse().unwrap()),
        "captcha_url" => state.set_captcha_url(value.parse().unwrap()),
        _ => {}
      }
    }
  }

  pub fn clear(&self) {
    let mut states = self.states.write().unwrap();
    states.clear();
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