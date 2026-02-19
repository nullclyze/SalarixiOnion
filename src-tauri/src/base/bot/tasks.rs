use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::task::JoinHandle;

pub static TASKS: Lazy<Arc<TaskManager>> = Lazy::new(|| Arc::new(TaskManager::new()));

pub struct Tasks {
  pub tasks: HashMap<String, Option<JoinHandle<()>>>,
  pub activity: HashMap<String, bool>,
}

impl Tasks {
  pub fn new() -> Self {
    let names = vec![
      "spamming",
      "movement",
      "jumping",
      "shifting",
      "waving",
      "anti-afk",
      "flight",
      "killaura",
      "scaffold",
      "anti-fall",
      "bow-aim",
      "stealer",
      "miner",
      "farmer",
    ];

    let mut tasks = HashMap::new();
    let mut activity = HashMap::new();

    for name in names {
      tasks.insert(name.to_string(), None);
      activity.insert(name.to_string(), false);
    }

    Self {
      tasks: tasks,
      activity: activity,
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

  pub fn kill_task(&mut self, name: &str) {
    if let Some(handle) = self.tasks.get(name) {
      if let Some(task) = handle {
        task.abort();
        self.activity.insert(name.to_string(), false);
      }
    }
  }

  pub fn kill_all_tasks(&mut self) {
    for (name, handle) in self.tasks.iter() {
      if let Some(task) = handle {
        task.abort();
        self.activity.insert(name.clone(), false);
      }
    }
  }
}

pub struct TaskManager {
  pub map: RwLock<HashMap<String, Arc<RwLock<Tasks>>>>,
}

impl TaskManager {
  pub fn new() -> Self {
    Self {
      map: RwLock::new(HashMap::new()),
    }
  }

  pub fn push(&self, username: &str) {
    if let Some(arc) = self.map.write().unwrap().get(username) {
      arc.write().unwrap().kill_all_tasks();
    }

    self
      .map
      .write()
      .unwrap()
      .insert(username.to_string(), Arc::new(RwLock::new(Tasks::new())));
  }

  pub fn remove(&self, username: &str) {
    self.map.write().unwrap().remove(username);
  }

  pub fn reset(&self, username: &str) {
    for (name, tasks) in self.map.write().unwrap().iter() {
      if name.as_str() == username {
        tasks.write().unwrap().kill_all_tasks();
      }
    }
  }

  pub fn clear(&self) {
    for (_, tasks) in self.map.write().unwrap().iter() {
      tasks.write().unwrap().kill_all_tasks();
    }

    self.map.write().unwrap().clear();
  }

  pub fn get(&self, username: &str) -> Option<Arc<RwLock<Tasks>>> {
    let map = self.map.read().unwrap();
    map.get(username).cloned()
  }

  pub fn get_task_activity(&self, username: &str, task: &str) -> bool {
    let map = self.map.read().unwrap();

    for (name, tasks) in map.iter() {
      if name.as_str() == username {
        return tasks.read().unwrap().get_task_activity(task);
      }
    }

    false
  }
}

// Функция запуска задачи
pub fn run_task(username: &str, task: &str, handle: JoinHandle<()>) {
  if let Some(tasks) = TASKS.get(username) {
    tasks.write().unwrap().run_task(task, handle);
  }
}

// Функция остановки задачи
pub fn kill_task(username: &str, task: &str) {
  if let Some(tasks) = TASKS.get(username) {
    tasks.write().unwrap().kill_task(task);
  }
}
