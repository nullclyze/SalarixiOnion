mod api;
mod service;

use api::commands::*;
use api::emit::emit_event_loop;

/// Запуск приложения
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_dialog::init())
    .setup(|app| {
      let handle = app.handle().clone();

      std::thread::spawn(move || {
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
      follow_radar_target,
      set_group,
      get_active_bots_count,
      get_memory_usage,
      control_bots,
      quick_task,
      execute_script,
      stop_script,
      get_server_info,
      open_url,
      set_discord_rpc
    ])
    .run(tauri::generate_context!())
    .expect("Не удалось запустить клиент");
}
