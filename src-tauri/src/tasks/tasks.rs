use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use once_cell::sync::Lazy;
use tokio::task::JoinHandle;


pub static TASKS: Lazy<Arc<BotTaskManager>> = Lazy::new(|| Arc::new(BotTaskManager::new()));

pub struct BotTasks {
  pub tasks: HashMap<String, Option<JoinHandle<()>>>
}

impl BotTasks {
  pub fn new() -> Self {
    let tasks_vec = vec![
      "spamming", "movement", "jumping", 
      "shifting", "waving", "anti-afk",
      "flight", "killaura", "scaffold",
      "anti-fall", "bow-aim", "stealer",
      "miner"
    ];
    
    let mut tasks = HashMap::new();

    for task in tasks_vec {
      tasks.insert(task.to_string(), None);
    }

    Self {
      tasks: tasks,
    }
  }

  pub fn set_task(&mut self, task: &str, handle: JoinHandle<()>) {
    self.tasks.insert(task.to_string(), Some(handle));
  }

  pub fn stop_task(&mut self, task: &str) {
    if let Some(o) = self.tasks.get(task) {
      if let Some(task) = o {
        task.abort();
      }
    }
  }

  pub fn stop_all_tasks(&mut self) {
    for element in self.tasks.iter().clone() {
      if let Some(task) = element.1 {
        task.abort();
      }
    }
  }
}

pub struct BotTaskManager {
  pub map: RwLock<HashMap<String, Arc<RwLock<BotTasks>>>>
}

impl BotTaskManager {
  pub fn new() -> Self {
    Self {
      map: RwLock::new(HashMap::new())
    }
  }

  pub fn add(&self, nickname: &String) {
    if let Some(arc) = self.map.write().unwrap().get(nickname) {
      arc.write().unwrap().stop_all_tasks();
    }

    self.map.write().unwrap().insert(nickname.clone(), Arc::new(RwLock::new(BotTasks::new())));
  }

  pub fn remove(&self, nickname: &String) {
    self.map.write().unwrap().remove(nickname);
  }

  pub fn get(&self, nickname: &String) -> Option<Arc<RwLock<BotTasks>>> {
    let map = self.map.read().unwrap();
    map.get(nickname).cloned()
  }

  pub fn clear(&self) {
    for element in self.map.write().unwrap().iter().clone() {
      element.1.write().unwrap().stop_all_tasks();
    }

    self.map.write().unwrap().clear();
  }
}