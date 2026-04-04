use hashbrown::HashMap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};

use crate::emit::send_log;

pub static PROFILES: Lazy<Arc<ProfileManager>> = Lazy::new(|| Arc::new(ProfileManager::new()));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Profile {
  pub status: ProfileStatus,
  pub username: String,
  pub password: Option<String>,
  pub email: Option<String>,
  pub proxy: ProfileProxy,
  pub ping: u32,
  pub health: u32,
  pub registered: bool,
  pub logined: bool,
  pub skin_is_set: bool,
  pub captcha_caught: bool,
  pub group: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ProfileStatus {
  Preparation,
  Connecting,
  Online,
  Offline,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfileProxy {
  pub ip_address: Option<String>,
  pub proxy: Option<String>,
  pub username: Option<String>,
  pub password: Option<String>,
}

impl Profile {
  pub fn new(username: &str, password: Option<String>, email: Option<String>) -> Self {
    Self {
      status: ProfileStatus::Preparation,
      username: username.to_string(),
      password: password,
      email: email,
      proxy: ProfileProxy {
        ip_address: None,
        proxy: None,
        username: None,
        password: None,
      },
      ping: 0,
      health: 0,
      registered: false,
      logined: false,
      skin_is_set: false,
      captcha_caught: false,
      group: "global".to_string(),
    }
  }

  pub fn set_status(&mut self, status: ProfileStatus) {
    self.status = status;
  }

  pub fn set_password(&mut self, password: &str) {
    self.password = Some(password.to_string());
  }

  pub fn set_email(&mut self, email: &str) {
    self.email = Some(email.to_string());
  }

  pub fn set_proxy(&mut self, proxy: ProfileProxy) {
    self.proxy = proxy;
  }

  pub fn set_ping(&mut self, ping: u32) {
    self.ping = ping;
  }

  pub fn set_health(&mut self, health: u32) {
    self.health = health;
  }

  pub fn set_registered(&mut self, state: bool) {
    self.registered = state;
  }

  pub fn set_logined(&mut self, state: bool) {
    self.logined = state;
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
  pub map: RwLock<HashMap<String, Profile>>,
}

impl ProfileManager {
  pub fn new() -> Self {
    Self {
      map: RwLock::new(HashMap::new()),
    }
  }

  pub fn push(&self, username: &str, password: Option<String>, email: Option<String>) {
    let profile = Profile::new(username, password, email);
    let mut profiles = self.map.write().unwrap();
    profiles.insert(username.to_string(), profile);
  }

  pub fn clear(&self) {
    let mut profiles = self.map.write().unwrap();
    profiles.clear();

    send_log(format!("Профили ботов очищены"), "extended");
  }

  pub fn get_count(&self) -> usize {
    self.map.read().unwrap().len()
  }

  pub fn set_status(&self, username: &str, status: ProfileStatus) {
    if let Some(profile) = self.map.write().unwrap().get_mut(username) {
      profile.set_status(status);
    }
  }

  pub fn set_str(&self, username: &str, field: &str, value: &str) {
    if let Some(profile) = self.map.write().unwrap().get_mut(username) {
      match field {
        "password" => profile.set_password(value),
        "email" => profile.set_email(value),
        "group" => profile.set_group(value),
        _ => {}
      }
    }
  }

  pub fn set_num(&self, username: &str, field: &str, value: u32) {
    if let Some(profile) = self.map.write().unwrap().get_mut(username) {
      match field {
        "ping" => profile.set_ping(value),
        "health" => profile.set_health(value),
        _ => {}
      }
    }
  }

  pub fn set_bool(&self, username: &str, field: &str, value: bool) {
    if let Some(profile) = self.map.write().unwrap().get_mut(username) {
      match field {
        "registered" => profile.set_registered(value),
        "logined" => profile.set_logined(value),
        "skin_is_set" => profile.set_skin_is_set(value),
        "captcha_caught" => profile.set_captcha_caught(value),
        _ => {}
      }
    }
  }

  pub fn set_proxy(&self, username: &str, proxy: ProfileProxy) {
    if let Some(profile) = self.map.write().unwrap().get_mut(username) {
      profile.set_proxy(proxy);
    }
  }

  pub fn get(&self, username: &str) -> Option<Profile> {
    let map = self.map.read().unwrap();

    if let Some(profile) = map.get(username) {
      return Some(profile.clone());
    }

    None
  }

  pub fn get_all(&self) -> HashMap<String, Profile> {
    let profiles = self.map.read().unwrap();
    let mut result = HashMap::new();

    for (username, profile) in profiles.iter() {
      result.insert(username.to_string(), profile.clone());
    }

    result
  }
}
