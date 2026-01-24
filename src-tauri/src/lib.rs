mod queue;
mod steamcmd;

use queue::{DownloadItem, DownloadStatus, QueueState};
use std::fs; // Added for file system operations
use std::thread;
use std::time::Duration;
use steamcmd::download_app;
use tauri::{AppHandle, Emitter, Manager, State};

fn sanitize(name: &str) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric() || *c == ' ' || *c == '-' || *c == '_')
        .collect::<String>()
        .trim()
        .to_string()
}

#[tauri::command]
async fn add_download(
    state: State<'_, QueueState>,
    steam_id: String,
    name: String,
) -> Result<String, String> {
    let (final_id_str, display_name_from_meta) =
        if steam_id.contains("steamcommunity.com/sharedfiles") || steam_id.contains("?id=") {
            let file_id = if let Some(start) = steam_id.find("?id=") {
                let rest = &steam_id[start + 4..];
                rest.split('&').next().unwrap_or(rest).to_string()
            } else {
                return Err("Invalid Workshop URL".to_string());
            };

            let output = std::process::Command::new("curl")
                .arg("-s")
                .arg("-L")
                .arg("-A")
                .arg("Mozilla/5.0 (Windows NT 10.0; Win64; x64)")
                .arg(&steam_id)
                .output()
                .map_err(|e| format!("Failed to run curl: {}", e))?;

            let html = String::from_utf8_lossy(&output.stdout);

            let game_id = if let Some(idx) = html.find("data-appid=\"") {
                let rest = &html[idx + 12..];
                if let Some(end) = rest.find("\"") {
                    rest[..end].to_string()
                } else {
                    return Err("Could not parse AppID (quote mismatch)".to_string());
                }
            } else if let Some(idx) = html.find("/app/") {
                let rest = &html[idx + 5..];
                let id_part: String = rest.chars().take_while(|c| c.is_numeric()).collect();
                if !id_part.is_empty() {
                    id_part
                } else {
                    return Err("Could not find Game AppID (data-appid missing)".to_string());
                }
            } else {
                return Err("Could not find Game AppID on Workshop page".to_string());
            };

            // Improved Title Parsing
            let workshop_title = if let Some(idx) = html.find("class=\"workshopItemTitle\"") {
                let rest = &html[idx..];
                if let Some(gt) = rest.find('>') {
                    let content = &rest[gt + 1..];
                    if let Some(end) = content.find("</div>") {
                        content[..end].trim().to_string()
                    } else {
                        "Workshop Item".to_string()
                    }
                } else {
                    "Workshop Item".to_string()
                }
            } else {
                "Workshop Item".to_string()
            };

            (format!("{}:{}", game_id, file_id), workshop_title)
        } else {
            (steam_id.clone(), "Unknown".to_string())
        };

    // Use fetched title if user didn't provide a custom name
    let final_name = if name.starts_with("App http") || name.starts_with("App 365") {
        if display_name_from_meta != "Unknown" && display_name_from_meta != "Workshop Item" {
            display_name_from_meta
        } else {
            name
        }
    } else {
        name
    };

    let id = state.add_item(final_id_str, final_name);
    Ok(id)
}

#[tauri::command]
fn get_queue(state: State<'_, QueueState>) -> Vec<DownloadItem> {
    state.get_queue()
}

#[tauri::command]
fn open_folder(state: State<'_, QueueState>, id: String) {
    let q = state.queue.lock().unwrap();
    if let Some(item) = q.iter().find(|i| i.steam_id == id || i.id == id) {
        if let Some(path) = &item.install_path {
            #[cfg(target_os = "windows")]
            {
                std::process::Command::new("explorer")
                    .arg(path)
                    .spawn()
                    .unwrap();
            }
        } else {
            // Fallback if path not set (legacy)
            let cwd = std::env::current_dir().unwrap();
            let root = if cwd.ends_with("src-tauri") {
                cwd.parent().unwrap()
            } else {
                cwd.as_path()
            };

            // Try to guess: download/SanitizedName or download/ID
            let folder_id = if let Some((_, f)) = id.split_once(':') {
                f
            } else {
                id.as_str()
            };
            let path = root.join("download").join(folder_id);
            #[cfg(target_os = "windows")]
            {
                std::process::Command::new("explorer")
                    .arg(path)
                    .spawn()
                    .unwrap();
            }
        }
    }
}

fn move_or_copy(src: &std::path::Path, dst: &std::path::Path) -> std::io::Result<()> {
    if fs::rename(src, dst).is_ok() {
        return Ok(());
    }

    // Fallback: Recursive copy & delete
    if src.is_dir() {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            move_or_copy(&entry.path(), &dst.join(entry.file_name()))?;
        }
        fs::remove_dir_all(src)?;
    } else {
        fs::copy(src, dst)?;
        fs::remove_file(src)?;
    }
    Ok(())
}

fn spawn_worker(app: AppHandle) {
    let state = app.state::<QueueState>();
    let queue = state.queue.clone();
    let app_handle = app.clone();

    thread::spawn(move || {
        loop {
            let mut current_item: Option<DownloadItem> = None;

            {
                let mut q = queue.lock().unwrap();
                if let Some(index) = q
                    .iter()
                    .position(|i| matches!(i.status, DownloadStatus::Pending))
                {
                    q[index].status = DownloadStatus::Downloading;
                    current_item = Some(q[index].clone());
                }
            }

            if let Some(item) = current_item {
                let _ = app_handle.emit("queue-update", ());

                let cwd = std::env::current_dir().unwrap();
                let executable_path = if cwd.ends_with("src-tauri") {
                    cwd.parent().unwrap().join("steamcmd.exe")
                } else {
                    cwd.join("steamcmd.exe")
                };

                let (real_id, game_id) = if let Some((g, f)) = item.steam_id.split_once(':') {
                    (f, Some(g))
                } else {
                    (item.steam_id.as_str(), None)
                };

                // Temp install dir: download/ID
                let root_dl = if cwd.ends_with("src-tauri") {
                    cwd.parent().unwrap().join("download")
                } else {
                    cwd.join("download")
                };
                let temp_install_dir = root_dl.join(real_id);

                let sanitized_name = sanitize(&item.name);
                let final_install_dir = root_dl.join(&sanitized_name);

                let result = download_app(
                    executable_path.clone(),
                    real_id,
                    temp_install_dir.to_str().unwrap(),
                    game_id,
                );

                // If success, move contents
                let final_status = match result {
                    Ok(_) => {
                        thread::sleep(Duration::from_secs(2));

                        if let Some(gid) = game_id {
                            let deep_path = temp_install_dir
                                .join("steamapps/workshop/content")
                                .join(gid)
                                .join(real_id);

                            if deep_path.exists() {
                                // Ensure final dir exists
                                if let Err(_) = fs::create_dir_all(&final_install_dir) {
                                    // ignore
                                }

                                // Move contents
                                match fs::read_dir(&deep_path) {
                                    Ok(entries) => {
                                        for entry in entries {
                                            if let Ok(entry) = entry {
                                                let file_name = entry.file_name();

                                                if file_name == "mods" && entry.path().is_dir() {
                                                    if let Ok(mod_entries) =
                                                        fs::read_dir(entry.path())
                                                    {
                                                        for mod_entry in mod_entries {
                                                            if let Ok(mod_entry) = mod_entry {
                                                                let dest = final_install_dir
                                                                    .join(mod_entry.file_name());
                                                                let _ = move_or_copy(
                                                                    &mod_entry.path(),
                                                                    &dest,
                                                                );
                                                            }
                                                        }
                                                    }
                                                } else {
                                                    let dest = final_install_dir.join(&file_name);
                                                    let _ = move_or_copy(&entry.path(), &dest);
                                                }
                                            }
                                        }
                                    }
                                    Err(_) => {}
                                }

                                // Cleanup temp
                                let _ = fs::remove_dir_all(&temp_install_dir);

                                DownloadStatus::Completed
                            } else {
                                DownloadStatus::Completed
                            }
                        } else {
                            DownloadStatus::Completed
                        }
                    }
                    Err(e) => DownloadStatus::Failed(e),
                };

                {
                    let mut q = queue.lock().unwrap();
                    if let Some(index) = q.iter().position(|i| i.id == item.id) {
                        q[index].status = final_status.clone();
                        if matches!(final_status, DownloadStatus::Completed) {
                            if game_id.is_some() {
                                q[index].install_path =
                                    Some(final_install_dir.to_string_lossy().to_string());
                            } else {
                                q[index].install_path =
                                    Some(temp_install_dir.to_string_lossy().to_string());
                            }
                        }
                    }
                }

                let _ = app_handle.emit("queue-update", ());
            } else {
                thread::sleep(Duration::from_secs(1));
            }
        }
    });
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(QueueState::new())
        .setup(|app| {
            spawn_worker(app.handle().clone());
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            add_download,
            get_queue,
            open_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
