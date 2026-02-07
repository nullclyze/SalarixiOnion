use azalea::auto_reconnect::AutoReconnectDelay;
use azalea::prelude::*;
use azalea::swarm::*;
use azalea::app::AppExit;
use azalea::JoinOpts;
use azalea::app::PluginGroup;
use azalea::protocol::connect::Proxy; 
use azalea_viaversion::ViaVersionPlugin;
use std::collections::HashMap;
use std::net::SocketAddr;  
use std::time::Duration;  
use std::thread;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

use crate::base::*;
use crate::base::modules::*;
use crate::base::handlers::*;
use crate::base::bot::*;
use crate::tools::*;
use crate::emit::*;
use crate::webhook::send_webhook;


pub static FLOW_MANAGER: Lazy<Arc<RwLock<Option<Arc<RwLock<FlowManager>>>>>> = Lazy::new(|| { Arc::new(RwLock::new(None)) });
pub static MODULE_MANAGER: Lazy<Arc<ModuleManager>> = Lazy::new(|| { Arc::new(ModuleManager::new()) });
pub static PLUGIN_MANAGER: Lazy<Arc<PluginManager>> = Lazy::new(|| { Arc::new(PluginManager::new()) });

// Функция инициализации FlowManager
pub fn init_flow_manager(manager: FlowManager) {
  let arc = Arc::new(RwLock::new(manager));
  
  let mut write_guard = FLOW_MANAGER.write();
  *write_guard = Some(arc); 
}

// Функция получения FlowManager
pub fn get_flow_manager() -> Option<Arc<RwLock<FlowManager>>> {
  let read_guard = FLOW_MANAGER.read();
  read_guard.as_ref().cloned()
}

const LEGIT_NICKNAMES: &[&str] = &[
  "Apple", "Boss", "Crazy", "Dancer", "Eagle", "Flyer", "Game", "Hunter", "Insider", "Joker",
  "King", "Lancer", "Master", "Ninja", "Ocean", "Phoenix", "Queen", "Ranger", "Savage", "Tornado",
  "Ultra", "Viking", "Warrior", "Xenon", "Yoda", "Zodiac", "Ace", "Bold", "Clever", "Dreams", "Salarixi",
  "Epic", "Frost", "Gigant", "Hero", "Iceberg", "Jester", "Knight", "Lethal", "Meteor", "Nova", "Protector",
  "Orange", "Pioneer", "Quasar", "Rampage", "Solar", "Titan", "Unstoppable", "Vandal", "Wizard", "Xray",
  "Yellow", "Zeus", "Awesome", "Brilliant", "Courage", "Dynamite", "Elegant", "Fearless", "Glory", "Hysteria",
  "Alpha", "Bionic", "Cosmic", "Dominator", "Electric", "Fury", "Gigabyte", "Heroic", "Ice", "Jolt",
  "Killer", "Laser", "Meteorite", "Nemesis", "Oceanic", "Paradox", "Quake", "Rampart", "Specter", "Thunder",
  "Ultimate", "Vapor", "Wizardry", "Xenonix", "Yellowstone", "Zigzag", "Apex", "Blitz", "Catalyst", "Dynamo",
  "Eon", "Flux", "Ghost", "Hawk", "Infinity", "Jolted", "Kaleidoscope", "Lumina", "Maelstrom", "Nebel",
  "Onyx", "Pulsar", "Quixote", "Renegade", "Skybolt", "Tornado", "Umbra", "Vagabond", "Wildfire", "Xylophone"
];

const LEGIT_PASSWORDS: &[&str] = &[
  "password", "invisible", "ytrewq", "mypass", "drowssap", 
  "asdfghjkl", "stealth", "unreadable", "possible"
];

// Функция генерации никнейма или пароля
pub fn generate_nickname_or_password(item: &str, t: String, template: String) -> String {
  match t.as_str() {
    "legit" => {
      let mut value = randelem(if item == "nickname" { LEGIT_NICKNAMES } else { LEGIT_PASSWORDS }).unwrap().to_string();
            
      value = value.chars().map(|c| {
        if randchance(0.25) {
          match c.to_ascii_lowercase() {
            'o' => '0',
            'a' => '4',
            'z' => '3',
            'e' => '3',
            'i' => '1',
            'l' => '1',
            'p' => '5',
            'v' => '8',
            'b' => '6',
            _ => c
          }
        } else {
          c
        }
      }).collect();
            
      value = value.chars().map(|c| {
        if randchance(0.4) && c.is_ascii_alphabetic() {
          c.to_ascii_uppercase()
        } else {
          c
        }
      }).collect();
            
      if value.len() <= 13 {
        let suffix_len = if value.len() <= 6 {
          randuint(4, 6) as usize
        } else {
          randuint(2, 3) as usize
        };
                
        let suffix = randstr(Classes::Multi, suffix_len as i32);
        let separator = if randchance(0.5) { "_" } else { "" };
                
        value = format!("{}{}{}", value, separator, suffix);
      }
          
      value
    },
    "random" => {
      let templates = vec!["#m#m#m#m", "#n#n#n#n", "#l#l#l#l","#m#l#n#m", "#l#m#n#n", "#m#m", "#n#n", "#m#n#l"];
      let chosen_template = randelem(&templates).unwrap();
        
      Mutator::mutate_text(chosen_template.to_string())
    },
    "custom" => {
      Mutator::mutate_text(template)
    },
    _ => String::new()
  }
}

// Функция получения текущих опций
pub fn get_current_options() -> Option<LaunchOptions> {
  if let Some(arc) = get_flow_manager() {
    return arc.read().options.clone();
  }

  None
}

// Структура опций запуска
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LaunchOptions {
  pub address: String,
  pub version: String,
  pub bots_count: i32,
  pub join_delay: u64,
  pub nickname_type: String,
  pub password_type: String,
  pub nickname_template: String,
  pub password_template: String,
  pub register_command: String,
  pub register_template: String,
  pub register_min_delay: u64,
  pub register_max_delay: u64,
  pub login_command: String,
  pub login_template: String,
  pub login_min_delay: u64,
  pub login_max_delay: u64,
  pub rejoin_delay: u64,
  pub view_distance: u8,
  pub language: String,
  pub chat_colors: bool,
  pub humanoid_arm: Option<String>,
  pub use_auto_rejoin: bool,
  pub proxy_list: Option<String>,
  pub use_proxy: bool,
  pub use_anti_captcha: bool,
  pub use_webhook: bool,
  pub use_chat_signing: bool,
  pub use_chat_monitoring: bool,
  pub skin_settings: SkinSettings,
  pub anti_captcha_settings: AntiCaptchaSettings,
  pub webhook_settings: WebhookSettings,
  pub plugins: Plugins
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkinSettings {
  pub skin_type: String,
  pub set_skin_command: Option<String>,
  pub custom_skin_by_nickname: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AntiCaptchaSettings {
  pub captcha_type: String,
  pub options: AntiCaptchaOptions
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AntiCaptchaOptions {
  pub web: AntiWebCaptchaOptions
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AntiWebCaptchaOptions {
  pub regex: Option<String>,
  pub required_url_part: Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebhookSettings {
  pub url: Option<String>,
  pub information: bool,
  pub data: bool,
  pub actions: bool
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Plugins {
  pub auto_armor: bool,
  pub auto_totem: bool,
  pub auto_eat: bool,
  pub auto_potion: bool,
  pub auto_look: bool,
  pub auto_shield: bool,
  pub auto_repair: bool
}

// Структура FlowManager
pub struct FlowManager {
  pub active: bool,
  pub options: Option<LaunchOptions>,
  pub swarm: Option<Swarm>,
  pub bots: HashMap<String, Client>,
  pub app_handle: Option<tauri::AppHandle>
}

impl FlowManager {
  pub fn new(app_handle: tauri::AppHandle) -> Self {
    Self {
      active: false,
      options: None,
      swarm: None,
      bots: HashMap::new(),
      app_handle: Some(app_handle)
    }
  }

  pub fn launch(&mut self, options: LaunchOptions) -> anyhow::Result<()> {
    self.options = Some(options.clone());
    self.swarm.take();
    self.bots.clear();

    PROFILES.clear();
    STATES.clear();
    TASKS.clear();

    self.active = true;

    if options.use_webhook {
      send_webhook(options.webhook_settings.url.clone(), format!("Запуск {} ботов на {}...", options.bots_count, options.address));
    }

    if options.use_webhook && options.webhook_settings.data {
      send_webhook(options.webhook_settings.url.clone(), format!("Опции запуска: {:#?}", options));
    }

    thread::spawn(move || {
      let rt = tokio::runtime::Runtime::new().unwrap();

      rt.block_on(async move {
        let local_set = tokio::task::LocalSet::new();

        let default_plugins = azalea::DefaultPlugins;
        let mut plugins = default_plugins.build();

        if !options.use_auto_rejoin {
          plugins = plugins.disable::<azalea::auto_reconnect::AutoReconnectPlugin>();
        } else {
          AutoReconnectDelay::new(Duration::from_millis(options.rejoin_delay));
        }

        if !options.use_chat_signing {
          plugins = plugins.disable::<azalea::chat_signing::ChatSigningPlugin>();
        }

        local_set.spawn_local(async move {
          emit_event(EventType::Log(LogEventPayload { 
            name: "extended".to_string(), 
            message: format!("Подготовка...")
          }));
      
          let mut flow =  SwarmBuilder::new_without_plugins()
            .add_plugins(plugins)
            .add_plugins(azalea::bot::DefaultBotPlugins)
            .add_plugins(azalea::swarm::DefaultSwarmPlugins)
            .join_delay(Duration::from_millis(options.join_delay))
            .set_swarm_handler(swarm_handler)
            .set_handler(single_handler);

          let mut accounts = Vec::new();

          for _ in 0..options.bots_count {
            let nickname = generate_nickname_or_password("nickname", options.nickname_type.clone(), options.nickname_template.clone());
            let password = generate_nickname_or_password("password", options.password_type.clone(), options.password_template.clone());
            
            accounts.push(Account::offline(&nickname));

            PROFILES.push(&nickname, password, options.version.clone());
            TASKS.push(&nickname);
            STATES.push(&nickname);
          }

          if options.use_proxy {
            let mut accounts_with_opts = Vec::new();

            for (i, account) in accounts.into_iter().enumerate() { 
              let opts_clone = options.clone(); 

              let mut join_opts = JoinOpts::new();  
                
              if opts_clone.use_proxy {  
                if let Some(proxy_list) = &opts_clone.proxy_list { 
                  let list: Vec<&str> = proxy_list.split("\n").collect();

                  if !list.is_empty() {  
                    let proxy_str = &list[i % list.len()].to_string();  

                    let clean_proxy_str = proxy_str
                      .trim_start_matches("socks5://")  
                      .trim_start_matches("socks4://")
                      .trim_start_matches("https://")
                      .trim_start_matches("http://"); 

                    let proxy_address: Vec<&str> = clean_proxy_str.split("@").collect(); 
                    let address = proxy_address.get(0).unwrap().to_string();

                    if let Ok(addr) = address.parse::<SocketAddr>() {  
                      let proxy = Proxy::new(addr, None);  
                      join_opts = join_opts.proxy(proxy);  

                      if let Some(mut profile) = PROFILES.get(&account.username) {
                        let split_address: Vec<&str> = address.split(":").collect();
                        profile.set_proxy(split_address.get(0).unwrap());
                      }
                    }  
                  }  
                }  
              }  
                  
              accounts_with_opts.push((account, join_opts));  
            }  

            for (account, opts) in accounts_with_opts {  
              flow = flow.add_account_with_opts(account, opts);  
            } 
          } else {
            for account in accounts {  
              flow = flow.add_account(account);  
            } 

            if options.version.as_str() != "1.21.11" {
              flow = flow.add_plugins(ViaVersionPlugin::start(options.version).await);
            }
          }

          emit_event(EventType::Log(LogEventPayload { 
            name: "extended".to_string(), 
            message: format!("Подготовка окончена")
          }));

          let _ = flow.start(options.address).await;
        });

        local_set.await;
      });
    });

    Ok(())
  }

  pub fn stop(&mut self) -> (String, String) {
    if !self.active {
      return ("warning".to_string(), format!("Нет активных ботов для остановки"));
    }

    if let Some(swarm) = self.swarm.clone() {
      swarm.ecs_lock.lock().write_message(AppExit::Success);
    }

    self.active = false;
    self.swarm.take();
    self.bots.clear();

    PROFILES.clear();
    TASKS.clear();

    if let Some(options) = &self.options {
      if options.use_webhook {
        send_webhook(options.webhook_settings.url.clone(), format!("{} ботов было остановлено", options.bots_count));
      }
    }

    ("info".to_string(), format!("Остановка ботов завершена"))
  }

  pub fn send_message(&self, nickname: &String, message: &String) -> Option<String> {
    for (name, bot) in self.bots.iter() {
      if *name == *nickname {
        bot.chat(message);
        return Some(format!("Бот {} успешно отправил сообщение '{}' в чат", nickname, message));
      }
    }

    None
  }

  pub fn reset_bot(&self, nickname: &String) -> Option<String> {
    for (name, _) in self.bots.iter() {
      if *name == *nickname {
        TASKS.reset(nickname);
        STATES.reset(nickname);
        return Some(format!("Все задачи и состояния бота {} сброшены", nickname));
      }
    }

    None
  }

  pub fn disconnect_bot(&self, nickname: &String) -> Option<String> {
    for (name, bot) in self.bots.iter() {
      if *name == *nickname {
        bot.disconnect();
        return Some(format!("Бот {} отключился", nickname));
      }
    }

    None
  }
}

// Структура ModuleManager
pub struct ModuleManager {
  chat: ChatModule,
  actions: ActionModule,
  inventory: InventoryModule,
  movement: MovementModule,
  anti_afk: AntiAfkModule,
  flight: FlightModule,
  killaura: KillauraModule,
  scaffold: ScaffoldModule,
  anti_fall: AntiFallModule,
  bow_aim: BowAimModule,
  stealer: StealerModule,
  miner: MinerModule,
  farmer: FarmerModule
}

impl ModuleManager {
  pub fn new() -> Self {
    Self {
      chat: ChatModule::new(),
      actions: ActionModule::new(),
      inventory: InventoryModule::new(),
      movement: MovementModule::new(),
      anti_afk: AntiAfkModule::new(),
      flight: FlightModule::new(),
      killaura: KillauraModule::new(),
      scaffold: ScaffoldModule::new(),
      anti_fall: AntiFallModule::new(),
      bow_aim: BowAimModule::new(),
      stealer: StealerModule::new(),
      miner: MinerModule::new(),
      farmer: FarmerModule::new()
    }
  }

  pub async fn control(&'static self, name: String, options: serde_json::Value, group: String) {
    if let Some(arc) = get_flow_manager() {
      let fm = arc.write();

      let bots = fm.bots.clone();

      if fm.bots.len() > 0 {
        for (nickname, bot) in bots.into_iter() {
          let current_options = options.clone();

          if let Some(profile) = PROFILES.get(&nickname) {
            if profile.group != group {
              continue;
            }
          }

          match name.as_str() {
            "chat" => {
              let options: ChatOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

              let options_task = options.clone();

              if options.mode.as_str() == "spamming" {
                self.chat.stop(&nickname);
              }

              if options.state {
                let task = tokio::spawn(async move {
                  match options_task.mode.as_str() {
                    "message" => { let _ = self.chat.message(&bot, options_task).await; },
                    "spamming" => { let _ = self.chat.spamming(&bot, options_task).await; },
                    _ => {}
                  }
                });

                if options.mode.as_str() == "spamming" {
                  run_task(&nickname, "spamming", task);
                }
              } else {
                if options.mode.as_str() == "spamming" {
                  self.chat.stop(&nickname);
                }
              }
            },
            "action" => {
              let options: ActionOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

              let options_task = options.clone();

              self.actions.stop(&bot, &options.action);

              if options.state {
                let task = tokio::spawn(async move {
                  match options_task.action.as_str() {
                    "jumping" => { self.actions.jumping(&bot, options_task).await; },
                    "shifting" => { self.actions.shifting(&bot, options_task).await; },
                    "waving" => { self.actions.waving(&bot, options_task).await; },
                    _ => {}
                  }
                });

                run_task(&nickname, &options.action, task);
              } else {
                self.actions.stop(&bot, &options.action);
              }
            },
            "inventory" => {
              let options: InventoryOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

              tokio::spawn(async move {
                self.inventory.interact(&bot, options).await;
              });
            },
            "movement" => {
              let options: MovementOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

              let options_task = options.clone();  

              self.movement.stop(&bot);

              if options.state {
                let task = tokio::spawn(async move {
                  self.movement.enable(&bot, options_task).await;
                });

                run_task(&nickname, "movement", task);
              } else {
                self.movement.stop(&bot);
              }
            },
            "anti-afk" => {
              let options: AntiAfkOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

              let options_task = options.clone(); 

              self.anti_afk.stop(&bot);

              if options.state {
                let task = tokio::spawn(async move {
                  self.anti_afk.enable(&bot, options_task).await;
                });

                run_task(&nickname, "anti-afk", task);
              } else {
                self.anti_afk.stop(&bot);
              }
            },
            "flight" => {
              let options: FlightOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

              let options_task = options.clone();  

              self.flight.stop(&nickname);

              if options.state {
                let task = tokio::spawn(async move {
                  self.flight.enable(&bot, options_task).await;
                });

                run_task(&nickname, "flight", task);
              } else {
                self.flight.stop(&nickname);
              }
            },
            "killaura" => {
              let options: KillauraOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

              let options_task = options.clone();  

              self.killaura.stop(&bot);

              if options.state {
                let task = tokio::spawn(async move {
                  self.killaura.enable(&bot, options_task).await;
                });

                run_task(&nickname, "killaura", task);
              } else {
                self.killaura.stop(&bot);
              }
            },
            "scaffold" => {
              let options: ScaffoldOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

              let options_task = options.clone();

              self.scaffold.stop(&bot);

              if options.state {
                let task = tokio::spawn(async move {
                  self.scaffold.enable(&bot, options_task).await;
                });

                run_task(&nickname, "scaffold", task);
              } else {
                self.scaffold.stop(&bot);
              }
            },
            "anti-fall" => {
              let options: AntiFallOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

              let options_task = options.clone();

              self.anti_fall.stop(&nickname);

              if options.state {
                let task = tokio::spawn(async move {
                  self.anti_fall.enable(&bot, options_task).await;
                });

                run_task(&nickname, "anti-fall", task);
              } else {
                self.anti_fall.stop(&nickname);
              }
            },
            "bow-aim" => {
              let options: BowAimOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

              let options_task = options.clone();  

              self.bow_aim.stop(&bot);

              if options.state {
                let task = tokio::spawn(async move {
                  self.bow_aim.enable(&bot, options_task).await;
                });

                run_task(&nickname, "bow-aim", task);
              } else {
                self.bow_aim.stop(&bot);
              }
            },
            "stealer" => {
              let options: StealerOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

              let options_task = options.clone();  

              self.stealer.stop(&bot);

              if options.state {
                let task = tokio::spawn(async move {
                  self.stealer.enable(&bot, options_task).await;
                });

                run_task(&nickname, "stealer", task);
              } else {
                self.stealer.stop(&bot);
              }
            },
            "miner" => {
              let options: MinerOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();
 
              let options_task = options.clone();  

              self.miner.stop(&bot);

              if options.state {
                let task = tokio::spawn(async move {
                  self.miner.enable(&bot, options_task).await;
                });

                run_task(&nickname, "miner", task);
              } else {
                self.miner.stop(&bot);
              }
            },
            "farmer" => {
              let options: FarmerOptions = serde_json::from_value(current_options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

              let options_task = options.clone();  

              self.farmer.stop(&nickname);

              if options.state {
                let task = tokio::spawn(async move {
                  self.farmer.enable(&bot, options_task).await;
                });

                run_task(&nickname, "farmer", task);
              } else {
                self.farmer.stop(&nickname);
              }
            },
            _ => {}
          }
        }
      }
    }
  }
}

// Структура PluginManager
pub struct PluginManager {
  auto_armor: AutoArmorPlugin,
  auto_totem: AutoTotemPlugin,
  auto_eat: AutoEatPlugin,
  auto_potion: AutoPotionPlugin,
  auto_look: AutoLookPlugin,
  auto_shield: AutoShieldPlugin,
  auto_repair: AutoRepairPlugin
}

impl PluginManager {
  pub fn new() -> Self {
    Self {
      auto_armor: AutoArmorPlugin::new(),
      auto_totem: AutoTotemPlugin::new(),
      auto_eat: AutoEatPlugin::new(),
      auto_potion: AutoPotionPlugin::new(),
      auto_look: AutoLookPlugin::new(),
      auto_shield: AutoShieldPlugin::new(),
      auto_repair: AutoRepairPlugin::new()
    }
  }

  pub fn load(&'static self, bot: &Client, plugins: &Plugins) {
    if plugins.auto_armor {
      self.auto_armor.enable(bot.clone());
    }

    if plugins.auto_totem {
      self.auto_totem.enable(bot.clone());
    }

    if plugins.auto_eat {
      self.auto_eat.enable(bot.clone());
    }

    if plugins.auto_potion {
      self.auto_potion.enable(bot.clone());
    }

    if plugins.auto_look {
      self.auto_look.enable(bot.clone());
    }

    if plugins.auto_shield {
      self.auto_shield.enable(bot.clone());
    }

    if plugins.auto_repair {
      self.auto_repair.enable(bot.clone());
    }
  }
}