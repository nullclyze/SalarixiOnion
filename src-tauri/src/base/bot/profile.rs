use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

pub static PROFILES: Lazy<Arc<ProfileManager>> = Lazy::new(|| Arc::new(ProfileManager::new()));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
  pub status: String,
  pub nickname: String,
  pub version: String,
  pub password: String,
  pub proxy: String,
  pub ping: u32,
  pub health: u32,
  pub satiety: u32,
  pub registered: bool,
  pub skin_is_set: bool,
  pub captcha_caught: bool,
  pub group: String,
}

impl Profile {
  pub fn new(nickname: &String, password: &String, version: &String) -> Self {
    Self {
      status: "Соединение...".to_string(),
      nickname: nickname.to_string(),
      version: version.to_string(),
      password: password.to_string(),
      proxy: "-".to_string(),
      ping: 0,
      health: 0,
      satiety: 0,
      registered: false,
      skin_is_set: false,
      captcha_caught: false,
      group: "global".to_string(),
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

  pub fn set_ping(&mut self, ping: u32) {
    self.ping = ping;
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

  pub fn set_group(&mut self, group: &str) {
    self.group = group.to_string();
  }
}

pub struct ProfileManager {
  pub map: RwLock<HashMap<String, Arc<RwLock<Profile>>>>,
}

impl ProfileManager {
  pub fn new() -> Self {
    Self {
      map: RwLock::new(HashMap::new()),
    }
  }

  pub fn push(&self, nickname: &String, password: String, version: String) {
    let arc = Arc::new(RwLock::new(Profile::new(nickname, &password, &version)));
    let mut profiles = self.map.write().unwrap();
    profiles.insert(nickname.to_string(), arc.clone());
  }

  pub fn clear(&self) {
    let mut profiles = self.map.write().unwrap();
    profiles.clear();
  }

  pub fn set_str(&self, nickname: &String, field: &str, value: &str) {
    if let Some(arc) = self.map.write().unwrap().get(nickname) {
      let mut profile = arc.write().unwrap();

      match field {
        "status" => profile.set_status(value),
        "password" => profile.set_password(value),
        "proxy" => profile.set_proxy(value),
        "group" => profile.set_group(value),
        _ => {}
      }
    }
  }

  pub fn set_num(&self, nickname: &String, field: &str, value: u32) {
    if let Some(arc) = self.map.write().unwrap().get(nickname) {
      let mut profile = arc.write().unwrap();

      match field {
        "ping" => profile.set_ping(value),
        "health" => profile.set_health(value),
        "satiety" => profile.set_satiety(value),
        _ => {}
      }
    }
  }

  pub fn set_bool(&self, nickname: &String, field: &str, value: bool) {
    if let Some(arc) = self.map.write().unwrap().get(nickname) {
      let mut profile = arc.write().unwrap();

      match field {
        "registered" => profile.set_registered(value),
        "skin_is_set" => profile.set_skin_is_set(value),
        "captcha_caught" => profile.set_captcha_caught(value),
        _ => {}
      }
    }
  }

  pub fn get(&self, nickname: &String) -> Option<Profile> {
    let map = self.map.read().unwrap();

    if let Some(profile) = map.get(nickname) {
      return Some(profile.write().unwrap().clone());
    }

    None
  }

  pub fn get_all(&self) -> HashMap<String, Profile> {
    let profiles = self.map.read().unwrap();
    let mut result = HashMap::new();

    for (nickname, profile) in profiles.iter() {
      if let Ok(p) = profile.read() {
        result.insert(nickname.to_string(), p.clone());
      }
    }

    result
  }
}
