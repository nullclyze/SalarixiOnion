use once_cell::sync::Lazy;
use rhai::Engine;
use std::sync::{Arc, RwLock};
use tokio::task::JoinHandle;

use crate::script::functions::BasicScriptFunctions;

pub static SCRIPT_EXECUTOR: Lazy<Arc<RwLock<ScriptExecutor>>> =
  Lazy::new(|| Arc::new(RwLock::new(ScriptExecutor::new())));

pub struct ScriptExecutor {
  pub worker: Option<JoinHandle<()>>,
}

impl ScriptExecutor {
  pub fn new() -> Self {
    Self { worker: None }
  }

  pub fn execute(&self, script: String) {
    tokio::spawn(async move {
      let mut engine = Engine::new();

      engine
        .register_fn("print", |str: &str| BasicScriptFunctions::print(str))
        .register_fn("log", |text: &str, name: &str| {
          BasicScriptFunctions::log(text, name)
        })
        .register_fn("message", |content: &str, name: &str| {
          BasicScriptFunctions::message(content, name)
        })
        .register_fn("webhook", |content: &str| {
          BasicScriptFunctions::webhook(content)
        });

      let _ = engine.eval::<()>(script.as_str());
    });
  }

  pub fn stop(&mut self) {
    if let Some(worker) = &self.worker {
      worker.abort();
      self.worker = None;
    }
  }
}
