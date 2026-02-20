use crate::base::{current_options, RegistryEvent, BOT_REGISTRY, MODULE_MANAGER, PLUGIN_MANAGER};
use crate::quick::QUICK_TASK_MANAGER;

/// Процессор всех событий реестра ботов
pub async fn registry_event_loop() {
  let mut rx = BOT_REGISTRY.events.subscribe();

  while let Ok(event) = rx.recv().await {
    match event {
      RegistryEvent::LoadPlugins { username } => {
        if let Some(opts) = current_options() {
          let _ = BOT_REGISTRY
            .get_bot(&username, async |_| {
              PLUGIN_MANAGER.load(&username, &opts.plugins);
            })
            .await;
        }
      }
      RegistryEvent::ControlModules {
        name,
        options,
        group,
      } => {
        MODULE_MANAGER.control(name, options, group).await;
      }
      RegistryEvent::QuickTask { name } => {
        QUICK_TASK_MANAGER.execute(name);
      }
      RegistryEvent::StopProcessing => {
        return;
      }
    }
  }
}
