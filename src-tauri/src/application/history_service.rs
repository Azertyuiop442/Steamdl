use crate::domain::types::{HistoryItem, HistoryState};
use crate::infrastructure::persistence::{load_history, save_history};
use std::fs;
use std::path::Path;
use tauri::AppHandle;

pub fn check_file_exists(path: &str) -> bool {
    Path::new(path).exists()
}

pub fn init_history(app: &AppHandle, state: &HistoryState) -> Result<(), String> {
    let loaded = load_history(app)?;
    let mut history = state.0.lock().map_err(|_| "Failed to lock mutex")?;
    *history = loaded;
    Ok(())
}

pub fn add_history_item(
    app: &AppHandle,
    state: &HistoryState,
    item: HistoryItem,
) -> Result<(), String> {
    let mut history = state.0.lock().map_err(|_| "Failed to lock mutex")?;
    // Remove existing item with same id to avoid duplicates (update logic)
    if let Some(pos) = history.iter().position(|x| x.id == item.id) {
        history.remove(pos);
    }
    history.push(item);

    save_history(app, &history)?;
    Ok(())
}

pub fn get_all_history(state: &HistoryState) -> Result<Vec<HistoryItem>, String> {
    let history = state.0.lock().map_err(|_| "Failed to lock mutex")?;
    // Return copy
    Ok(history.clone())
}

pub fn clear_all_history(app: &AppHandle, state: &HistoryState) -> Result<(), String> {
    let mut history = state.0.lock().map_err(|_| "Failed to lock mutex")?;
    history.clear();
    save_history(app, &history)?;
    Ok(())
}

pub fn remove_item(app: &AppHandle, state: &HistoryState, id: &str) -> Result<(), String> {
    let mut history = state.0.lock().map_err(|_| "Failed to lock mutex")?;
    if let Some(pos) = history.iter().position(|x| x.id == id) {
        let item = history.remove(pos);

        if Path::new(&item.install_path).exists() {
            let _ = fs::remove_dir_all(&item.install_path);
        }

        save_history(app, &history)?;
    }
    Ok(())
}
