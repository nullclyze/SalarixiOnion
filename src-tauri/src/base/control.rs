use crate::base::*;
use crate::base::modules::*;


// Структура ModuleManager
pub struct ModuleManager;

impl ModuleManager {
  pub async fn control(name: String, options: serde_json::Value, group: String) {
    if let Some(arc) = get_flow_manager() {
      let fm = arc.write();

      let bots = fm.bots.clone();

      if fm.bots.len() > 0 {
        for bot in bots.into_values() {
          let current_options = options.clone();

          if let Some(profile) = PROFILES.get(&bot.username()) {
            if profile.group != group {
              continue;
            }
          }

          let nickname = bot.username();

          match name.as_str() {
            "chat" => {
              let options: ChatOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

              let options_task = options.clone();

              if options.mode.as_str() == "spamming" {
                ChatModule::stop(&nickname);
              }

              if options.state {
                let task = tokio::spawn(async move {
                  match options_task.mode.as_str() {
                    "message" => { let _ = ChatModule::message(&bot, options_task).await; },
                    "spamming" => { let _ = ChatModule::spamming(&bot, options_task).await; },
                    _ => {}
                  }
                });

                if options.mode.as_str() == "spamming" {
                  TASKS.get(&nickname).unwrap().write().unwrap().run_task("spamming", task);
                }
              } else {
                if options.mode.as_str() == "spamming" {
                  ChatModule::stop(&nickname);
                }
              }
            },
            "action" => {
              let options: ActionOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

              let options_task = options.clone();

              ActionModule::stop(&bot, &options.action);

              if options.state {
                let task = tokio::spawn(async move {
                  match options_task.action.as_str() {
                    "jumping" => { ActionModule::jumping(&bot, options_task).await; },
                    "shifting" => { ActionModule::shifting(&bot, options_task).await; },
                    "waving" => { ActionModule::waving(&bot, options_task).await; },
                    _ => {}
                  }
                });

                TASKS.get(&nickname).unwrap().write().unwrap().run_task(&options.action, task);
              } else {
                ActionModule::stop(&bot, &options.action);
              }
            },
            "inventory" => {
              let options: InventoryOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

              tokio::spawn(async move {
                InventoryModule::action(&bot, options).await;
              });
            },
            "movement" => {
              let options: MovementOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

              let options_task = options.clone();  

              MovementModule::stop(&bot);

              if options.state {
                let task = tokio::spawn(async move {
                  MovementModule::enable(&bot, options_task).await;
                });

                TASKS.get(&nickname).unwrap().write().unwrap().run_task("movement", task);
              } else {
                MovementModule::stop(&bot);
              }
            },
            "anti-afk" => {
              let options: AntiAfkOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

              let options_task = options.clone(); 

              AntiAfkModule::stop(&bot);

              if options.state {
                let task = tokio::spawn(async move {
                  AntiAfkModule::enable(&bot, options_task).await;
                });

                TASKS.get(&nickname).unwrap().write().unwrap().run_task("anti-afk", task);
              } else {
                AntiAfkModule::stop(&bot);
              }
            },
            "flight" => {
              let options: FlightOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

              let options_task = options.clone();  

              FlightModule::stop(&nickname);

              if options.state {
                let task = tokio::spawn(async move {
                  FlightModule::enable(&bot, options_task).await;
                });

                TASKS.get(&nickname).unwrap().write().unwrap().run_task("flight", task);
              } else {
                FlightModule::stop(&nickname);
              }
            },
            "killaura" => {
              let options: KillauraOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

              let options_task = options.clone();  

              KillauraModule::stop(&bot);

              if options.state {
                let task = tokio::spawn(async move {
                  KillauraModule::enable(&bot, options_task).await;
                });

                TASKS.get(&nickname).unwrap().write().unwrap().run_task("killaura", task);
              } else {
                KillauraModule::stop(&bot);
              }
            },
            "scaffold" => {
              let options: ScaffoldOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

              let options_task = options.clone();

              ScaffoldModule::stop(&bot);

              if options.state {
                let task = tokio::spawn(async move {
                  ScaffoldModule::enable(&bot, options_task).await;
                });

                TASKS.get(&nickname).unwrap().write().unwrap().run_task("scaffold", task);
              } else {
                ScaffoldModule::stop(&bot);
              }
            },
            "anti-fall" => {
              let options: AntiFallOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

              let options_task = options.clone();

              AntiFallModule::stop(&nickname);

              if options.state {
                let task = tokio::spawn(async move {
                  AntiFallModule::enable(&bot, options_task).await;
                });

                TASKS.get(&nickname).unwrap().write().unwrap().run_task("anti-fall", task);
              } else {
                AntiFallModule::stop(&nickname);
              }
            },
            "bow-aim" => {
              let options: BowAimOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

              let options_task = options.clone();  

              BowAimModule::stop(&bot);

              if options.state {
                let task = tokio::spawn(async move {
                  BowAimModule::enable(&bot, options_task).await;
                });

                TASKS.get(&nickname).unwrap().write().unwrap().run_task("bow-aim", task);
              } else {
                BowAimModule::stop(&bot);
              }
            },
            "stealer" => {
              let options: StealerOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

              let options_task = options.clone();  

              StealerModule::stop(&bot);

              if options.state {
                let task = tokio::spawn(async move {
                  StealerModule::enable(&bot, options_task).await;
                });

                TASKS.get(&nickname).unwrap().write().unwrap().run_task("stealer", task);
              } else {
                StealerModule::stop(&bot);
              }
            },
            "miner" => {
              let options: MinerOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();
 
              let options_task = options.clone();  

              MinerModule::stop(&bot);

              if options.state {
                let task = tokio::spawn(async move {
                  MinerModule::enable(&bot, options_task).await;
                });

                TASKS.get(&nickname).unwrap().write().unwrap().run_task("miner", task);
              } else {
                MinerModule::stop(&bot);
              }
            },
            "farmer" => {
              let options: FarmerOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

              let options_task = options.clone();  

              FarmerModule::stop(&nickname);

              if options.state {
                let task = tokio::spawn(async move {
                  FarmerModule::enable(&bot, options_task).await;
                });

                TASKS.get(&nickname).unwrap().write().unwrap().run_task("farmer", task);
              } else {
                FarmerModule::stop(&nickname);
              }
            },
            _ => {}
          }
        }
      }
    }
  }
}