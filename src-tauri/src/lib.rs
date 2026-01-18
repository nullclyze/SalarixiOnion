mod emit;
mod tools;
mod base;
mod radar;
mod state;
mod tasks;
mod quick;

use crate::base::*;
use crate::quick::QuickTaskManager;
use crate::radar::*;
use crate::state::{STATES, BotState};
use crate::tasks::*;
use crate::emit::*;


// Функция запуска ботов
#[tauri::command(async)]
async fn launch_bots(options: LaunchOptions) -> (String, String) {
  if let Some(arc) = get_flow_manager() {
    let mut fm = arc.write();

    if fm.active {
      return ("warning".to_string(), format!("Запуск невозможен, существуют активные боты"));
    }

    let _ = fm.launch(options);

    return ("system".to_string(), format!("Запуск принят"));
  }

  ("error".to_string(), format!("Не удалось запустить ботов"))
}

// Функция остановки ботов
#[tauri::command(async)]
async fn stop_bots() -> (String, String) {
  if let Some(arc) = get_flow_manager() {
    return arc.write().stop();
  } else {
    return ("error".to_string(), format!("FlowManager не инициализирован"));
  }
}

// Функция получения профилей ботов
#[tauri::command]
fn get_bot_profiles() -> Option<std::collections::HashMap<String, BotState>> {
  Some(STATES.get_all_profiles())
}

// Функция отправки сообщения от бота
#[tauri::command]
fn send_message(nickname: String, message: String) -> (String, String) {
  if let Some(arc) = get_flow_manager() {
    if let Some(msg) = arc.write().send_message(&nickname, &message) {
      return ("info".to_string(), msg);
    }
  }

  ("error".to_string(), format!("Бот {} не смог отправить сообщение '{}' в чат", nickname, message))
}

// Функция отключения бота
#[tauri::command]
fn disconnect_bot(nickname: String) -> (String, String) {
  if let Some(arc) = get_flow_manager() {
    if let Some(msg) = arc.write().disconnect_bot(&nickname) {
      return ("info".to_string(), msg);
    }
  }

  ("error".to_string(), format!("Не удалось отключить бота {}", nickname))
}

// Функция получения radar-данных
#[tauri::command]
fn get_radar_data(target: String) -> Option<RadarInfo> {
  RadarManager::find_target(target)
}

// Функция сохранения radar-данных
#[tauri::command]
fn save_radar_data(target: String, path: String, filename: String, x: f64, y: f64, z: f64) {
  RadarManager::save_data(target, path, filename, x, y, z);
}

// Функция получения количества активных ботов
#[tauri::command]
fn get_active_bots_count() -> i32 {
  if let Some(arc) = get_flow_manager() {
    let fm = arc.read();
    return fm.bots_count;
  }

  0
}

// Функция получения используемой памяти
#[tauri::command]
fn get_memory_usage() -> f64 {
  if let Some(usage) = memory_stats::memory_stats() {
    return usage.physical_mem as f64 / 1_000_000.0;
  }

  0.0
}

// Функция управления ботами
#[tauri::command]
async fn control(name: String, options: serde_json::Value) {
  emit_event(EventType::Log(LogEventPayload { 
    name: "extended".to_string(), 
    message: format!("[ debug ] Управление '{}' | Опции: {}", name, options)
  }));

  ModuleManager::control(name, options).await;
}

// Функция выполнения быстрых задач
#[tauri::command]
async fn quick_task(name: String) {
  emit_event(EventType::Log(LogEventPayload { 
    name: "extended".to_string(), 
    message: format!("[ debug ] Быстрая задача '{}'", name)
  }));

  QuickTaskManager::execute(name).await;
}

// Функция открытия URL в браузере
#[tauri::command]
fn open_url(url: String) {
  let _ = open::that(url);
}

// Функция остановки главного процесса
#[tauri::command]
fn exit() {
  std::process::exit(0x0);
}

// Функция запуска
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      let app_handle = app.handle().clone();
            
      init_flow_manager(FlowManager::new(app_handle));
            
      Ok(())
    })
    .plugin(tauri_plugin_opener::init())
    .invoke_handler(tauri::generate_handler![
      exit, launch_bots, stop_bots, 
      get_bot_profiles, send_message, disconnect_bot,
      get_radar_data, save_radar_data,
      get_active_bots_count, get_memory_usage,
      control, quick_task, open_url
    ])
    .run(tauri::generate_context!())
    .expect("Не удалось запустить клиент");
}