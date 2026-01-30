pub mod application;
pub mod domain;
pub mod infrastructure;

use crate::application::commands::{
    add_download, check_path_exists, clear_history, get_history, get_queue, open_folder, remove_history_item, retry_download, AppState,
};
use crate::application::history_service::init_history;
use crate::application::queue_manager::start_worker;
use crate::domain::types::{HistoryState, QueueState, ProcessManagerState};
use crate::infrastructure::process_manager::ProcessManager;
use std::sync::{Arc, Mutex};
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState(Mutex::new(QueueState::new())))
        .manage(HistoryState::default())
        .manage(ProcessManagerState(Arc::new(Mutex::new(ProcessManager::new()))))
        .setup(|app| {
            start_worker(app.handle().clone());
            let history_state = app.state::<HistoryState>();
            if let Err(e) = init_history(app.handle(), &history_state) {
                eprintln!("Failed to init history: {}", e);
            }
            Ok(())
        })
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::Destroyed = event {
                if let Some(state) = window.app_handle().try_state::<ProcessManagerState>() {
                    if let Ok(manager) = state.0.lock() {
                        manager.kill_all();
                    }
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            add_download,
            get_queue,
            open_folder,
            get_history,
            clear_history,
            remove_history_item,
            check_path_exists,
            retry_download
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
