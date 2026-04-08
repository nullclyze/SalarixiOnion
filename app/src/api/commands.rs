use hashbrown::HashMap;
use serde::{Deserialize, Serialize};

use crate::api::emit::{send_log, send_message};
use crate::service::core::bot::{Profile, PROFILES};
use crate::service::core::footing::{active_bots_count, current_options, launch_bots_on_server, process_is_active, stop_bots_and_destroy_data, LaunchOptions};
use crate::service::core::functions::{disconnect_bot, reset_bot, send_message_from_bot};
use crate::service::core::registry::{RegistryEvent, BOT_REGISTRY};
use crate::service::rpc::DISCORD_RPC_MANAGER;
use crate::service::script::SCRIPT_EXECUTOR;
use crate::service::tools::ping::{ping_server, ServerInformation};
use crate::service::tools::radar::{RadarInfo, RADAR_MANAGER};
use crate::service::webhook::send_webhook;

/// Функция запуска ботов
#[tauri::command]
pub async fn launch_bots(options: LaunchOptions) -> bool {
  launch_bots_on_server(options)
}

/// Функция остановки ботов
#[tauri::command]
pub async fn stop_bots() -> bool {
  stop_bots_and_destroy_data()
}

/// Функция получения профилей ботов
#[tauri::command]
pub async fn get_bot_profiles() -> HashMap<String, Profile> {
  PROFILES.get_all()
}

#[derive(Serialize, Deserialize)]
struct SendMessageCommand {
  username: String,
  message: String,
}

#[derive(Serialize, Deserialize)]
struct ResetBotCommand {
  username: String,
}

#[derive(Serialize, Deserialize)]
struct DisconnectBotCommand {
  username: String,
}

/// Функция отправки быстрой команды
#[tauri::command]
pub async fn send_command(command: String, options: serde_json::Value) {
  match command.as_str() {
    "send_message" => {
      let opts: SendMessageCommand = serde_json::from_value(options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

      send_message_from_bot(opts.username, opts.message);
    }
    "reset_bot" => {
      let opts: ResetBotCommand = serde_json::from_value(options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

      reset_bot(opts.username);
    }
    "disconnect_bot" => {
      let opts: DisconnectBotCommand = serde_json::from_value(options).map_err(|e| format!("Ошибка парсинга опций: {}", e)).unwrap();

      disconnect_bot(opts.username);
    }
    _ => {}
  }
}

/// Функция изменения группы бота
#[tauri::command]
pub async fn set_group(nickname: String, group: String) {
  PROFILES.set_str(&nickname, "group", &group);
}

/// Функция получения radar-данных
#[tauri::command]
pub async fn get_radar_data(target: String) -> Option<RadarInfo> {
  RADAR_MANAGER.find_target(target).await
}

/// Функция сохранения radar-данных
#[tauri::command]
pub async fn save_radar_data(target: String, path: String, filename: String, x: f64, y: f64, z: f64) {
  RADAR_MANAGER.save_data(target, path, filename, x, y, z);
}

/// Функция преследования radar-цели
#[tauri::command]
pub async fn follow_radar_target(x: i32, z: i32) {
  RADAR_MANAGER.follow(x, z);
}

/// Функция получения количества активных ботов
#[tauri::command]
pub async fn get_active_bots_count() -> i32 {
  active_bots_count()
}

/// Функция получения используемой памяти
#[tauri::command]
pub async fn get_memory_usage() -> f64 {
  if let Some(usage) = memory_stats::memory_stats() {
    return usage.physical_mem as f64 / 1_000_000.0;
  }

  0.0
}

/// Функция управления ботами
#[tauri::command]
pub async fn control_bots(name: String, options: serde_json::Value, group: String) {
  if process_is_active() {
    if let Some(opts) = current_options() {
      if opts.basic.use_webhook && opts.webhook.send_actions {
        send_webhook(
          opts.webhook.url,
          format!("Группа ботов с названием '{}' приняла команду '{}'. Полученные опции: {}", group, name, options),
        );
      }
    }

    send_log(
      format!("Группа ботов с названием '{}' приняла команду '{}'. Полученные опции: {}", group, name, options),
      "extended",
    );

    send_message("Управление", format!("Группа ботов с названием '{}' приняла команду '{}'", group, name));

    BOT_REGISTRY.send_event(RegistryEvent::ControlModules { name, options, group });
  }
}

/// Функция выполнения быстрых задач
#[tauri::command]
pub async fn quick_task(name: String) {
  if let Some(opts) = current_options() {
    if opts.basic.use_webhook && opts.webhook.send_actions {
      send_webhook(opts.webhook.url, format!("Быстрая задача '{}'", name));
    }
  }

  send_log(format!("Быстрая задача '{}'", name), "extended");

  send_message(
    "Быстрая задача",
    format!("{} ботов получили быструю задачу '{}'", get_active_bots_count().await, name),
  );

  BOT_REGISTRY.send_event(RegistryEvent::QuickTask { name });
}

/// Функция выполнения пользовательского скрипта
#[tauri::command]
pub async fn execute_script(script: String) {
  SCRIPT_EXECUTOR.execute("#".to_string(), script);
}

/// Функция остановки пользовательского скрипта
#[tauri::command]
pub async fn stop_script() {
  SCRIPT_EXECUTOR.stop();
}

/// Функция пингования сервера
#[tauri::command]
pub async fn get_server_info(address: String) -> ServerInformation {
  ping_server(address).await
}

/// Функция открытия URL в браузере
#[tauri::command]
pub async fn open_url(url: String) {
  let _ = open::that(url);
}

/// Функция остановки главного процесса
#[tauri::command]
pub fn exit() {
  std::process::exit(0x0);
}

/// Функция задания статуса Discord RPC
#[tauri::command]
pub async fn set_discord_rpc(version: String, state: bool) {
  tokio::spawn(async move {
    if state {
      DISCORD_RPC_MANAGER.write().unwrap().enable(version);
    } else {
      DISCORD_RPC_MANAGER.write().unwrap().disable();
    }
  });
}
