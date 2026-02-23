use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use crate::emit::send_log;

pub static STATES: Lazy<Arc<StateManager>> = Lazy::new(|| Arc::new(StateManager::new()));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct States {
  pub can_walking: bool,
  pub can_sprinting: bool,
  pub can_eating: bool,
  pub can_drinking: bool,
  pub can_attacking: bool,
  pub can_looking: bool,
  pub can_interacting: bool,
  pub is_walking: bool,
  pub is_sprinting: bool,
  pub is_eating: bool,
  pub is_drinking: bool,
  pub is_attacking: bool,
  pub is_looking: bool,
  pub is_interacting: bool,
}

impl States {
  pub fn new() -> Self {
    Self {
      can_walking: true,
      can_sprinting: true,
      can_eating: true,
      can_drinking: true,
      can_attacking: true,
      can_looking: true,
      can_interacting: true,
      is_walking: false,
      is_sprinting: false,
      is_eating: false,
      is_drinking: false,
      is_attacking: false,
      is_looking: false,
      is_interacting: false,
    }
  }

  pub fn get(&self, field: &str) -> bool {
    match field {
      "can_walking" => {
        return self.can_walking;
      }
      "can_sprinting" => {
        return self.can_sprinting;
      }
      "can_eating" => {
        return self.can_eating;
      }
      "can_drinking" => {
        return self.can_drinking;
      }
      "can_attacking" => {
        return self.can_attacking;
      }
      "can_looking" => {
        return self.can_looking;
      }
      "can_interacting" => {
        return self.can_interacting;
      }
      "is_walking" => {
        return self.is_walking;
      }
      "is_sprinting" => {
        return self.is_sprinting;
      }
      "is_eating" => {
        return self.is_eating;
      }
      "is_drinking" => {
        return self.is_drinking;
      }
      "is_attacking" => {
        return self.is_attacking;
      }
      "is_looking" => {
        return self.is_looking;
      }
      "is_interacting" => {
        return self.is_interacting;
      }
      _ => {}
    }

    false
  }

  pub fn set(&mut self, field: &str, value: bool) {
    match field {
      "can_walking" => {
        self.can_walking = value;
      }
      "can_sprinting" => {
        self.can_sprinting = value;
      }
      "can_eating" => {
        self.can_eating = value;
      }
      "can_drinking" => {
        self.can_drinking = value;
      }
      "can_attacking" => {
        self.can_attacking = value;
      }
      "can_looking" => {
        self.can_looking = value;
      }
      "can_interacting" => {
        self.can_interacting = value;
      }
      "is_walking" => {
        self.is_walking = value;
      }
      "is_sprinting" => {
        self.is_sprinting = value;
      }
      "is_eating" => {
        self.is_eating = value;
      }
      "is_drinking" => {
        self.is_drinking = value;
      }
      "is_attacking" => {
        self.is_attacking = value;
      }
      "is_looking" => {
        self.is_looking = value;
      }
      "is_interacting" => {
        self.is_interacting = value;
      }
      _ => {}
    }
  }
}

pub struct StateManager {
  pub map: RwLock<HashMap<String, Arc<RwLock<States>>>>,
}

impl StateManager {
  pub fn new() -> Self {
    Self {
      map: RwLock::new(HashMap::new()),
    }
  }

  pub fn push(&self, username: &str) {
    let mut states = self.map.write().unwrap();
    if !states.contains_key(username) {
      states.insert(username.to_string(), Arc::new(RwLock::new(States::new())));
    }
  }

  pub fn clear(&self) {
    let mut states = self.map.write().unwrap();
    states.clear();

    send_log(format!("Состояния ботов очищены"), "extended");
  }

  pub fn reset(&self, username: &str) {
    if let Some(arc) = self.map.write().unwrap().get(username) {
      let mut states = arc.write().unwrap();
      states.can_walking = true;
      states.can_sprinting = true;
      states.can_eating = true;
      states.can_drinking = true;
      states.can_attacking = true;
      states.can_looking = true;
      states.can_interacting = true;
      states.is_walking = false;
      states.is_sprinting = false;
      states.is_eating = false;
      states.is_drinking = false;
      states.is_attacking = false;
      states.is_looking = false;
      states.is_interacting = false;
    }
  }

  pub fn set_state(&self, username: &str, field: &str, value: bool) {
    if let Some(arc) = self.map.write().unwrap().get(username) {
      let mut states = arc.write().unwrap();
      states.set(field, value);
    }
  }

  pub fn set_mutual_states(&self, username: &str, name: &str, current_value: bool) {
    if let Some(arc) = self.map.write().unwrap().get(username) {
      let mut states = arc.write().unwrap();
      states.set(format!("is_{}", name).as_str(), current_value);
      states.set(format!("can_{}", name).as_str(), !current_value);
    }
  }

  pub fn get_state(&self, username: &str, field: &str) -> bool {
    let map = self.map.read().unwrap();

    if let Some(states) = map.get(username) {
      return states.read().unwrap().get(field);
    }

    false
  }
}
