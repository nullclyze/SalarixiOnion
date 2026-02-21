use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::thread;

mod base;
mod common;
mod emit;
mod generators;
mod methods;
mod quick;
mod script;
mod tools;
mod webhook;

use crate::base::*;
use crate::emit::*;
use crate::script::*;
use crate::tools::*;
use crate::webhook::*;

/// Функция запуска ботов
#[tauri::command]
async fn launch_bots(options: LaunchOptions) -> bool {
  launch_bots_on_server(options)
}

/// Функция остановки ботов
#[tauri::command]
async fn stop_bots() -> bool {
  stop_bots_and_destroy_data()
}

/// Функция получения профилей ботов
#[tauri::command]
async fn get_bot_profiles() -> HashMap<String, Profile> {
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

/// Функция отправки команд в FlowManager
#[tauri::command]
async fn send_command(command: String, options: serde_json::Value) {
  match command.as_str() {
    "send_message" => {
      let opts: SendMessageCommand = serde_json::from_value(options)
        .map_err(|e| format!("Ошибка парсинга опций: {}", e))
        .unwrap();

      send_message_from_bot(opts.username, opts.message);
    }
    "reset_bot" => {
      let opts: ResetBotCommand = serde_json::from_value(options)
        .map_err(|e| format!("Ошибка парсинга опций: {}", e))
        .unwrap();

      reset_bot(opts.username);
    }
    "disconnect_bot" => {
      let opts: DisconnectBotCommand = serde_json::from_value(options)
        .map_err(|e| format!("Ошибка парсинга опций: {}", e))
        .unwrap();

      disconnect_bot(opts.username);
    }
    _ => {}
  }
}

/// Функция изменения группы бота
#[tauri::command]
async fn set_group(nickname: String, group: String) {
  PROFILES.set_str(&nickname, "group", &group);
}

/// Функция получения radar-данных
#[tauri::command]
async fn get_radar_data(target: String) -> Option<RadarInfo> {
  RADAR_MANAGER.find_target(target).await
}

/// Функция сохранения radar-данных
#[tauri::command]
async fn save_radar_data(target: String, path: String, filename: String, x: f64, y: f64, z: f64) {
  RADAR_MANAGER.save_data(target, path, filename, x, y, z);
}

/// Функция получения количества активных ботов
#[tauri::command]
async fn get_active_bots_count() -> i32 {
  active_bots_count()
}

/// Функция получения используемой памяти
#[tauri::command]
async fn get_memory_usage() -> f64 {
  if let Some(usage) = memory_stats::memory_stats() {
    return usage.physical_mem as f64 / 1_000_000.0;
  }

  0.0
}

/// Функция управления ботами
#[tauri::command]
async fn control(name: String, options: serde_json::Value, group: String) {
  if let Some(opts) = current_options() {
    if opts.use_webhook && opts.webhook_settings.actions {
      send_webhook(
        opts.webhook_settings.url,
        format!(
          "Группа ботов с названием '{}' приняла команду '{}'. Полученные опции: {}",
          group, name, options
        ),
      );
    }
  }

  send_log(
    format!(
      "Группа ботов с названием '{}' приняла команду '{}'. Полученные опции: {}",
      group, name, options
    ),
    "extended",
  );

  send_message(
    "Управление",
    format!(
      "Группа ботов с названием '{}' приняла команду '{}'",
      group, name
    ),
  );

  BOT_REGISTRY.send_event(RegistryEvent::ControlModules {
    name,
    options,
    group,
  });
}

/// Функция выполнения быстрых задач
#[tauri::command]
async fn quick_task(name: String) {
  if let Some(opts) = current_options() {
    if opts.use_webhook && opts.webhook_settings.actions {
      send_webhook(
        opts.webhook_settings.url,
        format!("Быстрая задача '{}'", name),
      );
    }
  }

  send_log(format!("Быстрая задача '{}'", name), "extended");

  send_message(
    "Быстрая задача",
    format!(
      "{} ботов получили быструю задачу '{}'",
      get_active_bots_count().await,
      name
    ),
  );

  BOT_REGISTRY.send_event(RegistryEvent::QuickTask { name });
}

/// Функция выполнения пользовательского скрипта
#[tauri::command]
async fn execute_script(script: String) {
  SCRIPT_EXECUTOR.read().unwrap().execute(script);
}

/// Функция остановки пользовательского скрипта
#[tauri::command]
async fn stop_script() {
  SCRIPT_EXECUTOR.write().unwrap().stop();
}

/// Функция рендеринга карты
#[tauri::command]
async fn render_map(nickname: String) -> Option<String> {
  let base64_code = BOT_REGISTRY
    .get_bot(&nickname, async |bot| MAP_RENDERER.render(bot))
    .await;

  base64_code
}

/// Функция сохранения карты
#[tauri::command]
async fn save_map(nickname: String, path: Option<String>, base64code: String) {
  MAP_RENDERER.save_map(nickname, path, base64code);
}

/// Функция пингования сервера
#[tauri::command]
async fn get_server_info(address: String) -> ServerInformation {
  ping_server(address).await
}

/// Функция открытия URL в браузере
#[tauri::command]
async fn open_url(url: String) {
  let _ = open::that(url);
}

/// Функция остановки главного процесса
#[tauri::command]
fn exit() {
  std::process::exit(0x0);
}

/// Функция запуска
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      let handle = app.handle().clone();

      thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(emit_event_loop(handle));
      });
      
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      exit,
      launch_bots,
      stop_bots,
      get_bot_profiles,
      send_command,
      get_radar_data,
      save_radar_data,
      set_group,
      get_active_bots_count,
      get_memory_usage,
      control,
      quick_task,
      execute_script,
      stop_script,
      render_map,
      save_map,
      get_server_info,
      open_url
    ])
    .run(tauri::generate_context!())
    .expect("Не удалось запустить клиент");
}
