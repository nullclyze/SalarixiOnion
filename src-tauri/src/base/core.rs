use azalea::app::AppExit;
use azalea::app::PluginGroup;
use azalea::auto_reconnect::AutoReconnectDelay;
use azalea::prelude::*;
use azalea::protocol::connect::Proxy;
use azalea::swarm::*;
use azalea::JoinOpts;
use azalea_viaversion::ViaVersionPlugin;
use once_cell::sync::Lazy;
use parking_lot::RwLock;
use serde::{Deserialize, Serialize};
use socks5_impl::protocol::UserKey;
use tokio::task::JoinHandle;
use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

use crate::base::bot::*;
use crate::base::handlers::*;
use crate::base::modules::*;
use crate::base::*;
use crate::emit::*;
use crate::generators::mutate_text;
use crate::generators::randchance;
use crate::generators::randelem;
use crate::generators::randstr;
use crate::generators::randuint;
use crate::generators::Classes;
use crate::webhook::send_webhook;

pub static FLOW_MANAGER: Lazy<Arc<RwLock<Option<Arc<RwLock<FlowManager>>>>>> =
  Lazy::new(|| Arc::new(RwLock::new(None)));
pub static MODULE_MANAGER: Lazy<Arc<ModuleManager>> = Lazy::new(|| Arc::new(ModuleManager::new()));
pub static PLUGIN_MANAGER: Lazy<Arc<PluginManager>> = Lazy::new(|| Arc::new(PluginManager::new()));

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
  "Apple",
  "Boss",
  "Crazy",
  "Dancer",
  "Eagle",
  "Flyer",
  "Game",
  "Hunter",
  "Insider",
  "Joker",
  "King",
  "Lancer",
  "Master",
  "Ninja",
  "Ocean",
  "Phoenix",
  "Queen",
  "Ranger",
  "Savage",
  "Tornado",
  "Ultra",
  "Viking",
  "Warrior",
  "Xenon",
  "Yoda",
  "Zodiac",
  "Ace",
  "Bold",
  "Clever",
  "Dreams",
  "Salarixi",
  "Epic",
  "Frost",
  "Gigant",
  "Hero",
  "Iceberg",
  "Jester",
  "Knight",
  "Lethal",
  "Meteor",
  "Nova",
  "Protector",
  "Orange",
  "Pioneer",
  "Quasar",
  "Rampage",
  "Solar",
  "Titan",
  "Unstoppable",
  "Vandal",
  "Wizard",
  "Xray",
  "Yellow",
  "Zeus",
  "Awesome",
  "Brilliant",
  "Courage",
  "Dynamite",
  "Elegant",
  "Fearless",
  "Glory",
  "Hysteria",
  "Alpha",
  "Bionic",
  "Cosmic",
  "Dominator",
  "Electric",
  "Fury",
  "Gigabyte",
  "Heroic",
  "Ice",
  "Jolt",
  "Killer",
  "Laser",
  "Meteorite",
  "Nemesis",
  "Oceanic",
  "Paradox",
  "Quake",
  "Rampart",
  "Specter",
  "Thunder",
  "Ultimate",
  "Vapor",
  "Wizardry",
  "Xenonix",
  "Yellowstone",
  "Zigzag",
  "Apex",
  "Blitz",
  "Catalyst",
  "Dynamo",
  "Eon",
  "Flux",
  "Ghost",
  "Hawk",
  "Infinity",
  "Jolted",
  "Kaleidoscope",
  "Lumina",
  "Maelstrom",
  "Nebel",
  "Onyx",
  "Pulsar",
  "Quixote",
  "Renegade",
  "Skybolt",
  "Tornado",
  "Umbra",
  "Vagabond",
  "Wildfire",
  "Xylophone",
];

const LEGIT_PASSWORDS: &[&str] = &[
  "password",
  "invisible",
  "ytrewq",
  "mypass",
  "drowssap",
  "asdfghjkl",
  "stealth",
  "unreadable",
  "possible",
];

// Функция генерации никнейма или пароля
pub fn generate_nickname_or_password(item: &str, t: String, template: String) -> String {
  match t.as_str() {
    "legit" => {
      let mut value = randelem(if item == "nickname" {
        LEGIT_NICKNAMES
      } else {
        LEGIT_PASSWORDS
      })
      .unwrap()
      .to_string();

      value = value
        .chars()
        .map(|c| {
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
              _ => c,
            }
          } else {
            c
          }
        })
        .collect();

      value = value
        .chars()
        .map(|c| {
          if randchance(0.4) && c.is_ascii_alphabetic() {
            c.to_ascii_uppercase()
          } else {
            c
          }
        })
        .collect();

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
    }
    "random" => {
      let templates = vec![
        "#m#m#m#m", "#n#n#n#n", "#l#l#l#l", "#m#l#n#m", "#l#m#n#n", "#m#m", "#n#n", "#m#n#l",
      ];
      let chosen_template = randelem(&templates).unwrap();

      mutate_text(chosen_template.to_string())
    }
    "custom" => mutate_text(template),
    _ => String::new(),
  }
}

/// Функция получения количества активных ботов
pub fn active_bots_count() -> i32 {
  let mut count = 0;

  for (_, profile) in PROFILES.get_all() {
    if profile.status.to_lowercase().as_str() == "онлайн" {
      count += 1;
    }
  }

  count
}

/// Функция получения текущих опций
pub fn current_options() -> Option<LaunchOptions> {
  if let Some(arc) = get_flow_manager() {
    return arc.read().options.clone();
  }

  None
}

/// Структура опций запуска
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
  pub register_mode: String,
  pub register_command: String,
  pub register_template: String,
  pub register_min_delay: u64,
  pub register_max_delay: u64,
  pub register_trigger: String,
  pub login_mode: String,
  pub login_command: String,
  pub login_template: String,
  pub login_min_delay: u64,
  pub login_max_delay: u64,
  pub login_trigger: String,
  pub rejoin_delay: u64,
  pub view_distance: u8,
  pub language: String,
  pub chat_colors: bool,
  pub humanoid_arm: Option<String>,
  pub use_auto_rejoin: bool,
  pub proxy_list: Option<String>,
  pub use_auto_register: bool,
  pub use_auto_login: bool,
  pub use_proxy: bool,
  pub use_anti_captcha: bool,
  pub use_webhook: bool,
  pub use_chat_signing: bool,
  pub use_chat_monitoring: bool,
  pub skin_settings: SkinSettings,
  pub anti_captcha_settings: AntiCaptchaSettings,
  pub webhook_settings: WebhookSettings,
  pub plugins: Plugins,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SkinSettings {
  pub skin_type: String,
  pub set_skin_command: Option<String>,
  pub custom_skin_by_nickname: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AntiCaptchaSettings {
  pub captcha_type: String,
  pub options: AntiCaptchaOptions,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AntiCaptchaOptions {
  pub web: AntiWebCaptchaOptions,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AntiWebCaptchaOptions {
  pub regex: Option<String>,
  pub required_url_part: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebhookSettings {
  pub url: Option<String>,
  pub information: bool,
  pub data: bool,
  pub actions: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Plugins {
  pub auto_armor: bool,
  pub auto_totem: bool,
  pub auto_eat: bool,
  pub auto_potion: bool,
  pub auto_look: bool,
  pub auto_shield: bool,
  pub auto_repair: bool,
}

/// Потоковый менеджер.
/// Он выполняет базовые функции (запуск / остановка ботов).
pub struct FlowManager {
  pub active: bool,
  pub options: Option<LaunchOptions>,
  pub swarm: Option<Swarm>,
  pub app_handle: Option<tauri::AppHandle>,
}

impl FlowManager {
  pub fn new(app_handle: tauri::AppHandle) -> Self {
    Self {
      active: false,
      options: None,
      swarm: None,
      app_handle: Some(app_handle),
    }
  }

  pub fn launch(&mut self, options: LaunchOptions) -> anyhow::Result<()> {
    self.options = Some(options.clone());
    self.active = true;

    tokio::spawn(registry_event_loop()); // $%#@$*@$1 &!@*$ @#%$&&!^%^ #!@%*!*#^

    if options.use_webhook {
      send_webhook(
        options.webhook_settings.url.clone(),
        format!(
          "Запуск {} ботов на {}...",
          options.bots_count, options.address
        ),
      );
    }

    if options.use_webhook && options.webhook_settings.data {
      send_webhook(
        options.webhook_settings.url.clone(),
        format!("Опции запуска: {:#?}", options),
      );
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
          send_log("Подготовка...".to_string(), "extended");

          let mut flow = SwarmBuilder::new_without_plugins()
            .add_plugins(plugins)
            .add_plugins(azalea::bot::DefaultBotPlugins)
            .add_plugins(azalea::swarm::DefaultSwarmPlugins)
            .join_delay(Duration::from_millis(options.join_delay))
            .set_swarm_handler(swarm_handler)
            .set_handler(single_handler);

          let mut accounts = Vec::new();

          for _ in 0..options.bots_count {
            let nickname = generate_nickname_or_password(
              "nickname",
              options.nickname_type.clone(),
              options.nickname_template.clone(),
            );

            let password = generate_nickname_or_password(
              "password",
              options.password_type.clone(),
              options.password_template.clone(),
            );

            accounts.push(Account::offline(&nickname));

            PROFILES.push(&nickname, password, options.version.clone());
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

                    let Some(address) = proxy_address.get(0) else {
                      continue;
                    };

                    if let Ok(addr) = address.parse::<SocketAddr>() {
                      let mut proxy = Proxy::new(addr, None);

                      if let Some(auth) = proxy_address.get(1) {
                        let split_auth: Vec<&str> = auth.split(":").collect();

                        let Some(username) = split_auth.get(0) else {
                          continue;
                        };

                        let Some(password) = split_auth.get(1) else {
                          continue;
                        };

                        proxy.auth = Some(UserKey {
                          username: username.to_string(),
                          password: password.to_string(),
                        });
                      }

                      join_opts = join_opts.proxy(proxy);

                      if let Some(mut profile) = PROFILES.get(&account.username) {
                        let split_address: Vec<&str> = address.split(":").collect();

                        let Some(ip_address) = split_address.get(0) else {
                          continue;
                        };

                        profile.set_proxy(*ip_address);
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
            flow = flow.add_accounts(accounts);

            if options.version.as_str() != "1.21.11" {
              flow = flow.add_plugins(ViaVersionPlugin::start(options.version).await);
            }
          }

          send_log("Подготовка окончена".to_string(), "extended");

          let _ = flow.start(options.address).await;
        });

        local_set.await;
      });
    });

    Ok(())
  }

  pub fn stop(&mut self) -> (String, String) {
    if !self.active {
      return (
        "warning".to_string(),
        format!("Нет активных ботов для остановки"),
      );
    }

    PLUGIN_MANAGER.clear();
    PROFILES.clear();
    TASKS.clear();
    STATES.clear();
    BOT_REGISTRY.destroy();

    if let Some(swarm) = &self.swarm {
      swarm.ecs_lock.lock().write_message(AppExit::Success);
    }

    self.swarm.take();
    self.active = false;

    if let Some(options) = &self.options {
      if options.use_webhook {
        send_webhook(
          options.webhook_settings.url.clone(),
          format!("{} ботов было остановлено", active_bots_count()),
        );
      }
    }

    ("info".to_string(), format!("Остановка ботов завершена"))
  }
}

/// Менеджер модулей.
/// Изначально он хранит в себе объекты всех существующих модулей.
/// Позволяет конвертировать JSON-данные в Rust структуры, распределять группы, включать / выключать определённые модули.
pub struct ModuleManager {
  chat: ChatModule,
  action: ActionModule,
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
  farmer: FarmerModule,
}

impl ModuleManager {
  pub fn new() -> Self {
    Self {
      chat: ChatModule::new(),
      action: ActionModule::new(),
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
      farmer: FarmerModule::new(),
    }
  }

  pub async fn control(&'static self, name: String, options: serde_json::Value, group: String) {
    for username in PROFILES.get_all().into_keys() {
      let current_options = options.clone();

      if let Some(profile) = PROFILES.get(&username) {
        if profile.group != group {
          continue;
        }
      }

      match name.as_str() {
        "chat" => {
          let options: ChatOptions = serde_json::from_value(current_options)
            .map_err(|e| format!("Ошибка парсинга опций: {}", e))
            .unwrap();

          let mode = options.clone().mode;

          if mode.as_str() == "spamming" {
            self.chat.stop(&username);
          }

          if options.state {
            let nickname = username.clone();

            let task = tokio::spawn(async move {
              self.chat.enable(&nickname, &options).await;
            });

            if mode.as_str() == "spamming" {
              run_task(&username, "spamming", task);
            }
          } else {
            if mode.as_str() == "spamming" {
              self.chat.stop(&username);
            }
          }
        }
        "action" => {
          let options: ActionOptions = serde_json::from_value(current_options)
            .map_err(|e| format!("Ошибка парсинга опций: {}", e))
            .unwrap();

          let action = options.action.clone();

          self.action.stop(&username, action.as_str()).await;

          if options.state {
            let nickname = username.clone();

            let task = tokio::spawn(async move {
              self.action.enable(&nickname, &options).await;
            });

            run_task(&username, action.as_str(), task);
          } else {
            self.action.stop(&username, action.as_str()).await;
          }
        }
        "inventory" => {
          let options: InventoryOptions = serde_json::from_value(current_options)
            .map_err(|e| format!("Ошибка парсинга опций: {}", e))
            .unwrap();

          self.inventory.interact(&username, &options).await;
        }
        "movement" => {
          let options: MovementOptions = serde_json::from_value(current_options)
            .map_err(|e| format!("Ошибка парсинга опций: {}", e))
            .unwrap();

          self.movement.stop(&username).await;

          if options.state {
            let nickname = username.clone();

            let task = tokio::spawn(async move {
              self.movement.enable(&nickname, &options).await;
            });

            run_task(&username, "movement", task);
          } else {
            self.movement.stop(&username).await;
          }
        }
        "anti-afk" => {
          let options: AntiAfkOptions = serde_json::from_value(current_options)
            .map_err(|e| format!("Ошибка парсинга опций: {}", e))
            .unwrap();

          self.anti_afk.stop(&username).await;

          if options.state {
            let nickname = username.clone();

            let task = tokio::spawn(async move {
              self.anti_afk.enable(&nickname, &options).await;
            });

            run_task(&username, "anti-afk", task);
          } else {
            self.anti_afk.stop(&username).await;
          }
        }
        "flight" => {
          let options: FlightOptions = serde_json::from_value(current_options)
            .map_err(|e| format!("Ошибка парсинга опций: {}", e))
            .unwrap();

          self.flight.stop(&username);

          if options.state {
            let nickname = username.clone();

            let task = tokio::spawn(async move {
              self.flight.enable(&nickname, &options).await;
            });

            run_task(&username, "flight", task);
          } else {
            self.flight.stop(&username);
          }
        }
        "killaura" => {
          let options: KillauraOptions = serde_json::from_value(current_options)
            .map_err(|e| format!("Ошибка парсинга опций: {}", e))
            .unwrap();

          self.killaura.stop(&username).await;

          if options.state {
            let nickname = username.clone();

            let task = tokio::spawn(async move {
              self.killaura.enable(&nickname, &options).await;
            });

            run_task(&username, "killaura", task);
          } else {
            self.killaura.stop(&username).await;
          }
        }
        "scaffold" => {
          let options: ScaffoldOptions = serde_json::from_value(current_options)
            .map_err(|e| format!("Ошибка парсинга опций: {}", e))
            .unwrap();

          self.scaffold.stop(&username).await;

          if options.state {
            let nickname = username.clone();

            let task = tokio::spawn(async move {
              self.scaffold.enable(&nickname, &options).await;
            });

            run_task(&username, "scaffold", task);
          } else {
            self.scaffold.stop(&username).await;
          }
        }
        "anti-fall" => {
          let options: AntiFallOptions = serde_json::from_value(current_options)
            .map_err(|e| format!("Ошибка парсинга опций: {}", e))
            .unwrap();

          self.anti_fall.stop(&username);

          if options.state {
            let nickname = username.clone();

            let task = tokio::spawn(async move {
              self.anti_fall.enable(&nickname, &options).await;
            });

            run_task(&username, "anti-fall", task);
          } else {
            self.anti_fall.stop(&username);
          }
        }
        "bow-aim" => {
          let options: BowAimOptions = serde_json::from_value(current_options)
            .map_err(|e| format!("Ошибка парсинга опций: {}", e))
            .unwrap();

          self.bow_aim.stop(&username).await;

          if options.state {
            let nickname = username.clone();

            let task = tokio::spawn(async move {
              self.bow_aim.enable(&nickname, &options).await;
            });

            run_task(&username, "bow-aim", task);
          } else {
            self.bow_aim.stop(&username).await;
          }
        }
        "stealer" => {
          let options: StealerOptions = serde_json::from_value(current_options)
            .map_err(|e| format!("Ошибка парсинга опций: {}", e))
            .unwrap();

          self.stealer.stop(&username);

          if options.state {
            let nickname = username.clone();

            let task = tokio::spawn(async move {
              self.stealer.enable(&nickname, &options).await;
            });

            run_task(&username, "stealer", task);
          } else {
            self.stealer.stop(&username);
          }
        }
        "miner" => {
          let options: MinerOptions = serde_json::from_value(current_options)
            .map_err(|e| format!("Ошибка парсинга опций: {}", e))
            .unwrap();

          self.miner.stop(&username).await;

          if options.state {
            let nickname = username.clone();

            let task = tokio::spawn(async move {
              self.miner.enable(&nickname, &options).await;
            });

            run_task(&username, "miner", task);
          } else {
            self.miner.stop(&username).await;
          }
        }
        "farmer" => {
          let options: FarmerOptions = serde_json::from_value(current_options)
            .map_err(|e| format!("Ошибка парсинга опций: {}", e))
            .unwrap();

          self.farmer.stop(&username);

          if options.state {
            let nickname = username.clone();

            let task = tokio::spawn(async move {
              self.farmer.enable(&nickname, &options).await;
            });

            run_task(&username, "farmer", task);
          } else {
            self.farmer.stop(&username);
          }
        }
        _ => {}
      }
    }
  }
}

/// Менеджер плагинов.
/// Изначально он хранит в себе объекты всех существующих плагинов.
/// Данный менеджер выполняет роль загрузчика плагинов для бота.
pub struct PluginManager {
  tasks: RwLock<HashMap<String, HashMap<String, Option<JoinHandle<()>>>>>,
  plugins: PluginList,
}

struct PluginList {
  auto_armor: AutoArmorPlugin,
  auto_totem: AutoTotemPlugin,
  auto_eat: AutoEatPlugin,
  auto_potion: AutoPotionPlugin,
  auto_look: AutoLookPlugin,
  auto_shield: AutoShieldPlugin,
  auto_repair: AutoRepairPlugin,
}

impl PluginManager {
  pub fn new() -> Self {
    Self {
      tasks: RwLock::new(HashMap::new()),
      plugins: PluginList { 
        auto_armor: AutoArmorPlugin::new(),
        auto_totem: AutoTotemPlugin::new(),
        auto_eat: AutoEatPlugin::new(),
        auto_potion: AutoPotionPlugin::new(),
        auto_look: AutoLookPlugin::new(),
        auto_shield: AutoShieldPlugin::new(),
        auto_repair: AutoRepairPlugin::new(),
      },
    }
  }

  pub fn load(&'static self, username: &String, plugins: &Plugins) {
    self.tasks.write().insert(username.clone(), HashMap::new());

    if plugins.auto_armor {
      self.plugins.auto_armor.enable(username.clone());
    }

    if plugins.auto_totem {
      self.plugins.auto_totem.enable(username.clone());
    }

    if plugins.auto_eat {
      self.plugins.auto_eat.enable(username.clone());
    }

    if plugins.auto_potion {
      self.plugins.auto_potion.enable(username.clone());
    }

    if plugins.auto_look {
      self.plugins.auto_look.enable(username.clone());
    }

    if plugins.auto_shield {
      self.plugins.auto_shield.enable(username.clone());
    }

    if plugins.auto_repair {
      self.plugins.auto_repair.enable(username.clone());
    }
  }

  pub fn push_task(&self, username: &str, plugin: &str, task: JoinHandle<()>) {
    if let Some(tasks) = self.tasks.write().get_mut(username) {
      if let Some(old_task) = tasks.get(plugin) {
        if let Some(active_old_task) = old_task {
          active_old_task.abort();
        }
      }

      tasks.insert(plugin.to_string(), Some(task));
    }
  }

  pub fn destroy_all_tasks(&self, username: &str) {
    if let Some(tasks) = self.tasks.write().get(username) {
      for (_, task) in tasks {
        if let Some(handle) = task {
          handle.abort();
        }
      }
    }
  }

  pub fn clear(&self) {
    for (username, _) in PROFILES.get_all() {
      self.destroy_all_tasks(&username);
    } 
  }
}
