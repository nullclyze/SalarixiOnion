mod base;
mod common;
mod emit;
mod quick;
mod script;
mod tools;
mod webhook;

use crate::base::*;
use crate::emit::*;
use crate::script::*;
use crate::tools::*;
use crate::webhook::*;

// Функция запуска ботов
#[tauri::command(async)]
async fn launch_bots(options: LaunchOptions) -> (String, String) {
  if let Some(arc) = get_flow_manager() {
    let mut fm = arc.write();

    if fm.active {
      return (
        "warning".to_string(),
        format!("Запуск невозможен, существуют активные боты"),
      );
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
    emit_message(
      "Система",
      format!("Остановка {} ботов...", get_active_bots_count().await),
    );

    return arc.write().stop();
  } else {
    return (
      "error".to_string(),
      format!("FlowManager не инициализирован"),
    );
  }
}

// Функция получения профилей ботов
#[tauri::command]
async fn get_bot_profiles() -> Option<std::collections::HashMap<String, Profile>> {
  Some(PROFILES.get_all())
}

// Функция отправки сообщения от бота
#[tauri::command]
async fn send_message(nickname: String, message: String) {
  if let Some(arc) = get_flow_manager() {
    arc.read().send_message(nickname, message);
  }
}

// Функция сброса всех задач и состояний бота
#[tauri::command]
async fn reset_bot(nickname: String) {
  if let Some(arc) = get_flow_manager() {
    arc.read().reset_bot(nickname);
  }
}

// Функция отключения бота
#[tauri::command]
async fn disconnect_bot(nickname: String) {
  if let Some(arc) = get_flow_manager() {
    arc.read().disconnect_bot(nickname);
  }
}

// Функция изменения группы бота
#[tauri::command]
async fn set_group(nickname: String, group: String) {
  PROFILES.set_str(&nickname, "group", &group);
}

// Функция получения radar-данных
#[tauri::command]
async fn get_radar_data(target: String) -> Option<RadarInfo> {
  RADAR_MANAGER.find_target(target).await
}

// Функция сохранения radar-данных
#[tauri::command]
async fn save_radar_data(target: String, path: String, filename: String, x: f64, y: f64, z: f64) {
  RADAR_MANAGER.save_data(target, path, filename, x, y, z);
}

// Функция получения количества активных ботов
#[tauri::command]
async fn get_active_bots_count() -> i32 {
  let mut count = 0;

  for (_, profile) in PROFILES.get_all() {
    if profile.status.to_lowercase().as_str() == "онлайн" {
      count += 1;
    }
  }

  count
}

// Функция получения используемой памяти
#[tauri::command]
async fn get_memory_usage() -> f64 {
  if let Some(usage) = memory_stats::memory_stats() {
    return usage.physical_mem as f64 / 1_000_000.0;
  }

  0.0
}

// Функция управления ботами
#[tauri::command]
async fn control(name: String, options: serde_json::Value, group: String) {
  if let Some(opts) = get_current_options() {
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

  emit_event(EventType::Log(LogEventPayload {
    name: "extended".to_string(),
    message: format!(
      "Группа ботов с названием '{}' приняла команду '{}'. Полученные опции: {}",
      group, name, options
    ),
  }));

  emit_message(
    "Управление",
    format!(
      "Группа ботов с названием '{}' приняла команду '{}'",
      group, name
    ),
  );

  BOT_REGISTRY.send_event(BotEvent::ControlModules {
    name,
    options,
    group,
  });
}

// Функция выполнения быстрых задач
#[tauri::command]
async fn quick_task(name: String) {
  if let Some(opts) = get_current_options() {
    if opts.use_webhook && opts.webhook_settings.actions {
      send_webhook(
        opts.webhook_settings.url,
        format!("Быстрая задача '{}'", name),
      );
    }
  }

  emit_event(EventType::Log(LogEventPayload {
    name: "extended".to_string(),
    message: format!("Быстрая задача '{}'", name),
  }));

  emit_message(
    "Быстрая задача",
    format!(
      "{} ботов получили быструю задачу '{}'",
      get_active_bots_count().await,
      name
    ),
  );

  BOT_REGISTRY.send_event(BotEvent::QuickTask { name });
}

// Функция выполнения пользовательского скрипта
#[tauri::command]
async fn execute_script(script: String) {
  SCRIPT_EXECUTOR.read().unwrap().execute(script);
}

// Функция остановки пользовательского скрипта
#[tauri::command]
async fn stop_script() {
  SCRIPT_EXECUTOR.write().unwrap().stop();
}

// Функция рендеринга карты
#[tauri::command]
async fn render_map(nickname: String) -> Option<String> {
  let base64_code = BOT_REGISTRY
    .get_bot(&nickname, async |bot| MAP_RENDERER.render(bot))
    .await;

  base64_code
}

// Функция сохранения карты
#[tauri::command]
async fn save_map(nickname: String, path: Option<String>, base64code: String) {
  MAP_RENDERER.save_map(nickname, path, base64code);
}

// Функция пингования сервера
#[tauri::command]
async fn get_server_info(address: String) -> ServerInformation {
  ping_server(address).await
}

// Функция открытия URL в браузере
#[tauri::command]
async fn open_url(url: String) {
  let _ = open::that(url);
}

// Функция остановки главного процесса
#[tauri::command]
async fn exit() {
  std::process::exit(0x0);
}

// Функция запуска
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .setup(|app| {
      init_flow_manager(FlowManager::new(app.handle().clone()));
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      exit,
      launch_bots,
      stop_bots,
      get_bot_profiles,
      send_message,
      reset_bot,
      disconnect_bot,
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
