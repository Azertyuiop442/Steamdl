use tauri::{command, State, AppHandle};
use crate::domain::types::{DownloadItem, QueueState, Status, HistoryItem, HistoryState};
use crate::domain::parser::parse_workshop_html;
use crate::infrastructure::file_manager;
use crate::application::history_service;
use std::sync::Mutex;

pub struct AppState(pub Mutex<QueueState>);

#[command]
pub async fn get_history(state: State<'_, HistoryState>) -> Result<Vec<HistoryItem>, String> {
    history_service::get_all_history(&state)
}

#[command]
pub async fn clear_history(app: AppHandle, state: State<'_, HistoryState>) -> Result<(), String> {
    history_service::clear_all_history(&app, &state)
}

#[command]
pub async fn remove_history_item(app: AppHandle, state: State<'_, HistoryState>, id: String) -> Result<(), String> {
    history_service::remove_item(&app, &state, &id)
}

#[command]
pub async fn add_download(
    state: State<'_, AppState>,
    steam_id: String,
    name: String,
) -> Result<String, String> {
    let (final_id, final_name) = if steam_id.contains("steamcommunity.com") {
        let output = std::process::Command::new("curl")
            .args(["-s", "-L", "-A", "Mozilla/5.0", &steam_id])
            .output()
            .map_err(|e| e.to_string())?;
        
        let html = String::from_utf8_lossy(&output.stdout);
        let file_id = steam_id.split("?id=")
            .nth(1)
            .ok_or("Invalid URL")?
            .split('&')
            .next()
            .ok_or("Invalid URL parameters")?;
            
        let meta = parse_workshop_html(&html, file_id)?;
        
        (format!("{}:{}", meta.app_id, meta.file_id), meta.title)
    } else {
        (steam_id, name)
    };

    let id = uuid::Uuid::new_v4().to_string();
    let mut q = state.0.lock().map_err(|_| "Poisoned mutex")?;
    
    let item = DownloadItem {
        id: id.clone(),
        steam_id: final_id,
        name: final_name,
        status: Status::Pending,
        install_path: None,
        created_at: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|e| e.to_string())?
            .as_secs(),
    };
    
    q.items.push(item);
    Ok(id)
}

#[command]
pub fn get_queue(state: State<'_, AppState>) -> Result<Vec<DownloadItem>, String> {
    let q = state.0.lock().map_err(|_| "Poisoned mutex")?;
    Ok(q.items.clone())
}

#[command]
pub fn open_folder(_state: State<'_, AppState>, path: String) -> Result<(), String> {
    file_manager::open_path(&path).map_err(|e| e.to_string())
}

#[command]
pub fn check_path_exists(path: String) -> bool {
    file_manager::check_path_exists(&path)
}

#[command]
pub async fn retry_download(
    state: State<'_, AppState>,
    steam_id: String,
    name: String,
) -> Result<String, String> {
    add_download(state, steam_id, name).await
}
