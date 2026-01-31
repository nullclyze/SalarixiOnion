use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use once_cell::sync::Lazy;
use tokio::task::JoinHandle;


pub static TASKS: Lazy<Arc<BotTaskManager>> = Lazy::new(|| Arc::new(BotTaskManager::new()));

pub struct BotTasks {
  pub tasks: HashMap<String, Option<JoinHandle<()>>>,
  pub activity: HashMap<String, bool>
}

impl BotTasks {
  pub fn new() -> Self {
    let names = vec![
      "spamming", "movement", "jumping", 
      "shifting", "waving", "anti-afk",
      "flight", "killaura", "scaffold",
      "anti-fall", "bow-aim", "stealer", 
      "miner", "farmer"
    ];
    
    let mut tasks = HashMap::new();
    let mut activity = HashMap::new();

    for name in names {
      tasks.insert(name.to_string(), None);
      activity.insert(name.to_string(), false);
    }

    Self {
      tasks: tasks,
      activity: activity
    }
  }

  pub fn get_task_activity(&self, name: &str) -> bool {
    if let Some(activity) = self.activity.get(name) {
      return *activity;
    }

    false
  }

  pub fn run_task(&mut self, name: &str, handle: JoinHandle<()>) {
    self.tasks.insert(name.to_string(), Some(handle));
    self.activity.insert(name.to_string(), true);
  }

  pub fn stop_task(&mut self, name: &str) {
    if let Some(handle) = self.tasks.get(name) {
      if let Some(task) = handle {
        task.abort();
        self.activity.insert(name.to_string(), false);
      }
    }
  }

  pub fn stop_all_tasks(&mut self) {
    for (name, handle) in self.tasks.iter() {
      if let Some(task) = handle {
        task.abort();
        self.activity.insert(name.clone(), false);
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

  pub fn get_task_activity(&self, nickname: &String, task: &str) -> bool {
    let map = self.map.read().unwrap();

    for el in map.iter() {
      if el.0 == nickname {
        return el.1.read().unwrap().get_task_activity(task);
      } 
    }

    false
  }
 
  pub fn clear(&self) {
    for element in self.map.write().unwrap().iter().clone() {
      element.1.write().unwrap().stop_all_tasks();
    }

    self.map.write().unwrap().clear();
  }
}