use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};

#[cfg(target_os = "windows")]
const STEAMCMD_BYTES: &[u8] = include_bytes!("../../bin/steamcmd-x86_64-pc-windows-msvc.exe");
#[cfg(target_os = "macos")]
const STEAMCMD_BYTES: &[u8] = include_bytes!("../../bin/steamcmd-aarch64-apple-darwin");

pub fn get_steamcmd_path(app: &AppHandle) -> Result<PathBuf, String> {
    let exe_name = if cfg!(target_os = "windows") {
        "steamcmd.exe"
    } else {
        "steamcmd"
    };
    let app_data = app.path().app_data_dir().map_err(|e| e.to_string())?;
    let bin_path = app_data.join("engine").join(exe_name);

    if !bin_path.exists() {
        extract_bin(&bin_path)?;
    }

    if !bin_path.exists() {
        return Err(format!(
            "Binary extraction failed: path does not exist after extraction attempt: {:?}",
            bin_path
        ));
    }

    Ok(bin_path)
}

fn extract_bin(path: &PathBuf) -> Result<(), String> {
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }
    fs::write(path, STEAMCMD_BYTES).map_err(|e| e.to_string())?;

    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(path).map_err(|e| e.to_string())?.permissions();
        perms.set_mode(0o755);
        fs::set_permissions(path, perms).map_err(|e| e.to_string())?;
    }
    Ok(())
}
