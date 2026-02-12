use crate::base::{get_current_options, BotEvent, BOT_REGISTRY, MODULE_MANAGER, PLUGIN_MANAGER};
use crate::quick::QUICK_TASK_MANAGER;

pub async fn event_processor() {
  let mut rx = BOT_REGISTRY.events.subscribe();

  while let Ok(event) = rx.recv().await {
    match event {
      BotEvent::LoadPlugins { username } => {
        if let Some(opts) = get_current_options() {
          tokio::spawn(async move {
            let _ = BOT_REGISTRY
              .get_bot(&username, |bot| {
                PLUGIN_MANAGER.load(bot, &opts.plugins);
              })
              .await;
          });
        }
      }

      BotEvent::ControlModules {
        name,
        options,
        group,
      } => {
        MODULE_MANAGER.control(name, options, group);
      }

      BotEvent::QuickTask { name } => {
        QUICK_TASK_MANAGER.execute(name);
      }
    }
  }
}
