use crate::domain::types::HistoryItem;
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

fn get_history_file_path(app: &AppHandle) -> Result<PathBuf, String> {
    let path = app.path().app_data_dir().map_err(|e| e.to_string())?;
    if !path.exists() {
        fs::create_dir_all(&path).map_err(|e| e.to_string())?;
    }
    Ok(path.join("history.json"))
}

pub fn save_history(app: &AppHandle, history: &[HistoryItem]) -> Result<(), String> {
    let path = get_history_file_path(app)?;
    let content = serde_json::to_string_pretty(history).map_err(|e| e.to_string())?;
    fs::write(path, content).map_err(|e| e.to_string())?;
    Ok(())
}

pub fn load_history(app: &AppHandle) -> Result<Vec<HistoryItem>, String> {
    let path = get_history_file_path(app)?;
    if !path.exists() {
        return Ok(Vec::new());
    }
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let history = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    Ok(history)
}
