use crate::application::commands::AppState;
use crate::application::history_service::add_history_item;
use crate::domain::sanitizer::sanitize_name;
use crate::domain::types::{DownloadItem, HistoryItem, HistoryState, ProcessManagerState, Status};
use crate::infrastructure::file_manager;
use crate::infrastructure::steam_client::execute_steamcmd_with_progress;
use std::fs;
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter, Manager};

pub fn start_worker(app: AppHandle) {
    thread::spawn(move || worker_loop(app));
}

fn worker_loop(app: AppHandle) {
    loop {
        if let Some(item) = pick_pending_item(&app) {
            let _ = app.emit("queue-update", ());
            let result = process_item(&app, item.clone());
            finalize_item(&app, &item.id, result);
            let _ = app.emit("queue-update", ());
        } else {
            thread::sleep(Duration::from_secs(1));
        }
    }
}

fn pick_pending_item(app: &AppHandle) -> Option<DownloadItem> {
    let state = app.try_state::<AppState>()?;
    let mut q = state.0.lock().ok()?;
    let index = q
        .items
        .iter()
        .position(|i| matches!(i.status, Status::Pending))?;
    q.items[index].status = Status::Downloading { progress: 0.0 };
    Some(q.items[index].clone())
}

fn finalize_item(app: &AppHandle, id: &str, result: Result<String, String>) {
    let state = match app.try_state::<AppState>() {
        Some(s) => s,
        None => return,
    };
    let mut q = match state.0.lock() {
        Ok(guard) => guard,
        Err(_) => return,
    };
    if let Some(item) = q.items.iter_mut().find(|i| i.id == id) {
        match result {
            Ok(path) => {
                item.status = Status::Completed;
                item.install_path = Some(path.clone());

                let history_state = app.state::<HistoryState>();
                let history_item = HistoryItem {
                    id: item.id.clone(),
                    steam_id: item.steam_id.clone(),
                    name: item.name.clone(),
                    install_path: path,
                    timestamp: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_secs(),
                };
                let _ = add_history_item(app, &history_state, history_item);
            }
            Err(e) => item.status = Status::Failed(e),
        }
    }
}

fn process_item(app: &AppHandle, item: DownloadItem) -> Result<String, String> {
    let process_manager_state = app.state::<ProcessManagerState>();
    let process_manager = process_manager_state.0.lock().map_err(|_| "Lock error")?;

    let (game_id, file_id) = parse_ids(&item.steam_id);
    let root_dl = app
        .path()
        .app_data_dir()
        .map_err(|e| e.to_string())?
        .join("download");
    let temp_dir = root_dl.join(&file_id);
    let final_dir = root_dl.join(sanitize_name(&item.name));

    let mut commands = vec![
        format!("force_install_dir \"{}\"", temp_dir.to_string_lossy()),
        "login anonymous".to_string(),
    ];

    if let Some(gid) = game_id.as_deref() {
        commands.push(format!("workshop_download_item {} {}", gid, file_id));
    } else {
        commands.push(format!("app_update {} validate", file_id));
    }

    let rx = execute_steamcmd_with_progress(app, commands, &process_manager, item.id.clone())?;

    let success = rx.recv().map_err(|_| "Process crashed")?;

    if !success {
        return Err("Download failed".to_string());
    }

    if let Some(gid) = game_id {
        let content_path = temp_dir
            .join("steamapps/workshop/content")
            .join(gid)
            .join(&file_id);
        if content_path.exists() {
            file_manager::move_recursive(&content_path, &final_dir).map_err(|e| e.to_string())?;
            let _ = fs::remove_dir_all(&temp_dir);
            return Ok(final_dir.to_string_lossy().to_string());
        }
    }
    Ok(temp_dir.to_string_lossy().to_string())
}

fn parse_ids(steam_id: &str) -> (Option<String>, String) {
    match steam_id.split_once(':') {
        Some((g, f)) => (Some(g.to_string()), f.to_string()),
        None => (None, steam_id.to_string()),
    }
}
