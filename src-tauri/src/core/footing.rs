use azalea::app::PluginGroup;
use azalea::auto_reconnect::AutoReconnectDelay;
use azalea::prelude::*;
use azalea::protocol::connect::Proxy;
use azalea::swarm::*;
use azalea::JoinOpts;
use azalea_viaversion::ViaVersionPlugin;
use hashbrown::HashMap;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use socks5_impl::protocol::UserKey;
use std::io;
use std::net::SocketAddr;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::RwLock;
use std::thread;
use std::time::Duration;
use tokio::task::JoinHandle;
use tokio::time::sleep;

use crate::core::bot::*;
use crate::core::handlers::*;
use crate::core::modules::*;
use crate::core::*;
use crate::emit::*;
use crate::generators::mutate_text;
use crate::generators::randchance;
use crate::generators::randelem;
use crate::generators::randstr;
use crate::generators::randuint;
use crate::generators::Classes;
use crate::webhook::send_webhook;

pub static ACTIVE: AtomicBool = AtomicBool::new(false);
pub static CURRENT_OPTIONS: Lazy<Arc<RwLock<Option<LaunchOptions>>>> =
  Lazy::new(|| Arc::new(RwLock::new(None)));

pub static MODULE_MANAGER: Lazy<Arc<ModuleManager>> = Lazy::new(|| Arc::new(ModuleManager::new()));
pub static PLUGIN_MANAGER: Lazy<Arc<PluginManager>> = Lazy::new(|| Arc::new(PluginManager::new()));

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

/// Структура опций запуска
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LaunchOptions {
  pub basic: BasicOptions,
  pub accounts: HashMap<String, AccountOptions>,
  pub plugins: PluginOptions,
  pub captcha_bypass: CaptchaBypassOptions,
  pub webhook: WebhookOptions,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct BasicOptions {
  pub address: String,
  pub version: String,
  pub bots_count: i32,
  pub join_delay: u64,
  pub nickname_type: String,
  pub password_type: String,
  pub email_type: String,
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
  pub humanoid_arm: Option<String>,
  pub use_auto_rejoin: bool,
  pub use_auto_register: bool,
  pub use_double_auth: bool,
  pub use_auto_login: bool,
  pub use_chat_signing: bool,
  pub use_auto_respawn: bool,
  pub use_accept_rp: bool,
  pub use_pathfinder: bool,
  pub use_auto_script: bool,
  pub use_proxy: bool,
  pub use_anti_captcha: bool,
  pub use_webhook: bool,
  pub use_accounts: bool,
  pub skin_type: String,
  pub set_skin_command: Option<String>,
  pub custom_skin_by_nickname: Option<String>,
  pub proxy_list: Option<String>,
  pub script: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AccountOptions {
  pub password: Option<String>,
  pub email: Option<String>,
  pub proxy: Option<String>,
  pub proxy_username: Option<String>,
  pub proxy_password: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PluginOptions {
  pub auto_armor: bool,
  pub auto_totem: bool,
  pub auto_eat: bool,
  pub auto_potion: bool,
  pub auto_look: bool,
  pub auto_shield: bool,
  pub auto_repair: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CaptchaBypassOptions {
  pub captcha_type: String,
  pub captcha_subtype: String,
  pub solve_mode: String,
  pub browser: String,
  pub regex: String,
  pub required_url_part: Option<String>,
  pub webdriver_server_url: Option<String>,
  pub number_of_columns: u32,
  pub number_of_rows: u32,
  pub api_key: Option<String>,
  pub api_service: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WebhookOptions {
  pub url: Option<String>,
  pub send_information: bool,
  pub send_data: bool,
  pub send_actions: bool,
}

#[derive(Debug)]
struct CustomAccount {
  object: Account,
  options: Option<AccountOptions>,
}

/// Функция генерации юзернейма или пароля
fn generate_username_or_password(item: &str, class: &str, template: &str) -> Option<String> {
  match class {
    "legit" => {
      let mut value = randelem(if item == "username" {
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

      Some(value)
    }
    "random" => {
      let templates = vec![
        "#m#m#m#m", "#n#n#n#n", "#l#l#l#l", "#m#l#n#m", "#l#m#n#n", "#m#m", "#n#n", "#m#n#l",
      ];
      let chosen_template = randelem(&templates).unwrap();

      Some(mutate_text(chosen_template.to_string()))
    }
    "custom" => Some(mutate_text(template.to_string())),
    _ => None,
  }
}

/// Функция генерации электронной почты
fn generate_email(class: &str) -> Option<String> {
  match class {
    "random" => {
      let templates = vec![
        "#m#m#m#m", "#n#n#n#n", "#l#l#l#l", 
        "#m#l#n#m", "#l#m#n#n", "#m#m", "#n#n", "#m#n#l",
        "#n#n#m#n", "#m#m#m#n#m", "#m#n#m#n#l",
      ];

      let services = vec!["gmail.com", "proton.me", "yandex.ru", "mail.ru"];
      
      let chosen_template = randelem(&templates).unwrap();
      let chosen_service = randelem(&services).unwrap();

      Some(format!("{}@{}", mutate_text(chosen_template.to_string()), chosen_service))
    }
    _ => None,
  }
}

/// Функция генерации уникального юзернейма
fn generate_unique_username(class: &str, template: &str) -> Option<String> {
  for _ in 0..5 {
    let Some(username) = generate_username_or_password("username", class, template) else {
      continue;
    };

    let mut is_unique = true;

    for name in PROFILES.get_all().keys() {
      if username == *name {
        is_unique = false;
        break;
      }
    }

    if is_unique {
      return Some(username);
    }
  }

  None
}

/// Функция получения количества активных ботов
pub fn active_bots_count() -> i32 {
  let mut count = 0;

  for (_, profile) in PROFILES.get_all() {
    if profile.status == ProfileStatus::Online {
      count += 1;
    }
  }

  count
}

/// Функция установки опций
pub fn set_options(options: LaunchOptions) {
  let mut guard = CURRENT_OPTIONS.write().unwrap();
  *guard = Some(options);
}

/// Функция получения текущих опций
pub fn current_options() -> Option<LaunchOptions> {
  CURRENT_OPTIONS.read().unwrap().clone()
}

/// Функция проверки активности основного процесса
pub fn process_is_active() -> bool {
  ACTIVE.load(Ordering::Relaxed)
}

fn clear_proxy(proxy: &str) -> String {
  proxy
    .trim_start_matches("socks5://")
    .trim_start_matches("socks4://")
    .trim_start_matches("https://")
    .trim_start_matches("http://")
    .to_string()
}

/// Функция запуска ботов на сервер
pub fn launch_bots_on_server(options: LaunchOptions) -> bool {
  if process_is_active() {
    send_log(
      format!("Запуск ботов невозможен: The process is already active"),
      "warning",
    );
    return false;
  }

  send_log("Подготовка...".to_string(), "extended");

  set_options(options.clone());

  ACTIVE.store(true, Ordering::Relaxed);

  tokio::spawn(registry_event_loop());

  if options.basic.use_anti_captcha
    && options.captcha_bypass.captcha_type.as_str() == "web"
    && options.captcha_bypass.solve_mode.as_str() == "auto"
  {
    tokio::spawn(WEB_CAPTCHA_BYPASS.webdriver_event_loop());

    tokio::spawn(async {
      sleep(Duration::from_millis(100)).await;
      WEB_CAPTCHA_BYPASS.send_webdriver_event(WebDriverEvent::CreateWebDriver {
        proxy: None,
        username: None,
        password: None,
      });
    });
  }

  if options.basic.use_webhook {
    send_webhook(
      options.webhook.url.clone(),
      format!("Запуск ботов на сервер {}...", options.basic.address),
    );
  }

  if options.basic.use_webhook && options.webhook.send_data {
    send_webhook(
      options.webhook.url.clone(),
      format!("Опции запуска: {:#?}", options),
    );
  }

  thread::spawn(move || {
    let rt = tokio::runtime::Runtime::new().unwrap();

    rt.block_on(async move {
      let local_set = tokio::task::LocalSet::new();

      let mut swarm_plugins = azalea::DefaultPlugins.build();

      if !options.basic.use_auto_rejoin {
        swarm_plugins = swarm_plugins.disable::<azalea::auto_reconnect::AutoReconnectPlugin>();
      } else {
        AutoReconnectDelay::new(Duration::from_millis(options.basic.rejoin_delay));
      }

      if !options.basic.use_chat_signing {
        swarm_plugins = swarm_plugins.disable::<azalea::chat_signing::ChatSigningPlugin>();
      }

      let mut bot_plugins = azalea::bot::DefaultBotPlugins.build();

      if !options.basic.use_auto_respawn {
        bot_plugins = bot_plugins.disable::<azalea::auto_respawn::AutoRespawnPlugin>();
      }

      if !options.basic.use_accept_rp {
        bot_plugins = bot_plugins.disable::<azalea::accept_resource_packs::AcceptResourcePacksPlugin>();
      }

      if !options.basic.use_pathfinder {
        bot_plugins = bot_plugins.disable::<azalea::pathfinder::PathfinderPlugin>();
      }

      local_set.spawn_local(async move {
        let mut flow = SwarmBuilder::new_without_plugins()
          .add_plugins(swarm_plugins)
          .add_plugins(bot_plugins)
          .add_plugins(azalea::swarm::DefaultSwarmPlugins)
          .join_delay(Duration::from_millis(options.basic.join_delay))
          .set_swarm_handler(swarm_handler)
          .set_handler(single_handler);

        let mut accounts = Vec::new();

        if options.basic.use_accounts {
          for (username, opts) in &options.accounts {
            accounts.push(CustomAccount {
              object: Account::offline(username),
              options: Some(opts.clone()),
            });

            PROFILES.push(username, opts.password.clone(), opts.email.clone());
          }
        } else {
          for _ in 0..options.basic.bots_count {
            let Some(username) = generate_unique_username(
              &options.basic.nickname_type,
              &options.basic.nickname_template,
            ) else {
              continue;
            };

            let password = generate_username_or_password(
              "password",
              &options.basic.password_type,
              &options.basic.password_template,
            );

            let email = generate_email(&options.basic.email_type);

            accounts.push(CustomAccount {
              object: Account::offline(&username),
              options: None,
            });

            PROFILES.push(&username, password, email);
          }
        }

        if options.basic.use_proxy || options.basic.use_accounts {
          let mut accounts_with_opts = Vec::new();

          if options.basic.use_accounts {
            for account in accounts.into_iter() {
              let mut join_opts = JoinOpts::new();

              if let Some(account_opts) = account.options.clone() {
                if let Some(proxy) = account_opts.proxy {
                  let mut profile_proxy = ProfileProxy {
                    ip_address: None,
                    proxy: None,
                    username: None,
                    password: None,
                  };

                  let clean_proxy = clear_proxy(&proxy);

                  profile_proxy.proxy = Some(clean_proxy.to_string());

                  if let Ok(addr) = clean_proxy.parse::<SocketAddr>() {
                    let mut proxy = Proxy::new(addr, None);

                    if let Some(username) = account_opts.proxy_username {
                      if let Some(password) = account_opts.proxy_password {
                        profile_proxy.username = Some(username.clone());
                        profile_proxy.password = Some(password.clone());

                        proxy.auth = Some(UserKey {
                          username: username,
                          password: password,
                        });
                      }
                    }

                    join_opts = join_opts.proxy(proxy);

                    let split_address: Vec<&str> = clean_proxy.split(":").collect();

                    let Some(ip_address) = split_address.get(0) else {
                      continue;
                    };

                    profile_proxy.ip_address = Some(ip_address.to_string());

                    PROFILES.set_proxy(&account.object.username, profile_proxy);
                  }
                };
              };

              accounts_with_opts.push((account, join_opts));
            }
          } else {
            for (i, account) in accounts.into_iter().enumerate() {
              let mut join_opts = JoinOpts::new();

              if let Some(proxy_list) = &options.basic.proxy_list {
                let list: Vec<&str> = proxy_list.split("\n").collect();

                if !list.is_empty() {
                  let mut profile_proxy = ProfileProxy {
                    ip_address: None,
                    proxy: None,
                    username: None,
                    password: None,
                  };

                  let proxy = list[i % list.len()];
                  let clean_proxy = clear_proxy(&proxy);

                  let proxy_address: Vec<&str> = clean_proxy.split("@").collect();

                  let Some(address) = proxy_address.get(0) else {
                    continue;
                  };

                  profile_proxy.proxy = Some(address.to_string());

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

                      profile_proxy.username = Some(username.to_string());
                      profile_proxy.password = Some(password.to_string());

                      proxy.auth = Some(UserKey {
                        username: username.to_string(),
                        password: password.to_string(),
                      });
                    }

                    join_opts = join_opts.proxy(proxy);

                    let split_address: Vec<&str> = address.split(":").collect();

                    let Some(ip_address) = split_address.get(0) else {
                      continue;
                    };

                    profile_proxy.ip_address = Some(ip_address.to_string());

                    PROFILES.set_proxy(&account.object.username, profile_proxy);
                  }
                }
              }

              accounts_with_opts.push((account, join_opts));
            }
          }

          for (account, opts) in accounts_with_opts {
            flow = flow.add_account_with_opts(account.object, opts);
          }
        } else {
          for account in accounts {
            flow = flow.add_account(account.object);
          }

          if options.basic.version.as_str() != "1.21.11" {
            flow = flow.add_plugins(ViaVersionPlugin::start(options.basic.version).await);
          }
        }

        send_log("Подготовка окончена".to_string(), "extended");

        let _ = flow.start(options.basic.address).await;
      });

      local_set.await;
    });
  });

  true
}

/// Функция остановки ботов
pub fn stop_bots_and_destroy_data() -> bool {
  let active_bots = active_bots_count();

  send_message("Система", "Остановка ботов...".to_string());

  if !process_is_active() {
    send_log(
      format!("Остановка ботов невозможна: The process is not active now"),
      "warning",
    );
    return false;
  }

  PLUGIN_MANAGER.clear();
  TASKS.clear();
  STATES.clear();
  PROFILES.clear();

  WEB_CAPTCHA_BYPASS.send_webdriver_event(WebDriverEvent::StopProcessing);
  MAP_ACCUMULATOR.clear_all();

  BOT_REGISTRY.clear();

  ACTIVE.store(false, Ordering::Relaxed);

  if let Some(options) = current_options() {
    if options.basic.use_webhook {
      send_webhook(
        options.webhook.url.clone(),
        format!("{} ботов было остановлено", active_bots),
      );
    }
  }

  send_log(format!("Остановка ботов завершена"), "info");

  true
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
  stalker: StalkerModule,
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
      stalker: StalkerModule::new(),
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
        "stalker" => {
          let options: StalkerOptions = serde_json::from_value(current_options)
            .map_err(|e| format!("Ошибка парсинга опций: {}", e))
            .unwrap();

          self.stalker.stop(&username);

          if options.state {
            let nickname = username.clone();

            let task = tokio::spawn(async move {
              self.stalker.enable(&nickname, &options).await;
            });

            run_task(&username, "stalker", task);
          } else {
            self.stalker.stop(&username);
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
/// Изначально он хранит в себе объекты всех существующих плагинов и список их задач.
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

  pub fn activate_for(&'static self, username: String, plugins: PluginOptions) -> io::Result<()> {
    self
      .tasks
      .write()
      .unwrap()
      .insert(username.clone(), HashMap::new());

    if plugins.auto_armor {
      self.plugins.auto_armor.activate(username.clone())?;
    }

    if plugins.auto_totem {
      self.plugins.auto_totem.activate(username.clone())?;
    }

    if plugins.auto_eat {
      self.plugins.auto_eat.activate(username.clone())?;
    }

    if plugins.auto_potion {
      self.plugins.auto_potion.activate(username.clone())?;
    }

    if plugins.auto_look {
      self.plugins.auto_look.activate(username.clone())?;
    }

    if plugins.auto_shield {
      self.plugins.auto_shield.activate(username.clone())?;
    }

    if plugins.auto_repair {
      self.plugins.auto_repair.activate(username.clone())?;
    }

    Ok(())
  }

  pub fn push_task(&self, username: &str, plugin: &str, task: JoinHandle<()>) {
    if let Some(tasks) = self.tasks.write().unwrap().get_mut(username) {
      if let Some(old_task) = tasks.get(plugin) {
        if let Some(active_old_task) = old_task {
          active_old_task.abort();
        }
      }

      tasks.insert(plugin.to_string(), Some(task));
    }
  }

  pub fn destroy_all_tasks(&self, username: &str) {
    if let Some(tasks) = self.tasks.read().unwrap().get(username) {
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

    send_log(format!("Активные задачи плагинов остановлены"), "extended");
  }
}
