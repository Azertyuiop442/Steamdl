use std::process::Command;

pub fn download_app(
    executable_path: std::path::PathBuf,
    app_id: &str,
    install_dir: &str,
    workshop_game_id: Option<&str>,
) -> Result<(), String> {
    if !executable_path.exists() {
        return Err(format!("SteamCMD not found at: {:?}", executable_path));
    }

    std::fs::create_dir_all(install_dir).map_err(|e| e.to_string())?;

    let script_path = executable_path.parent().unwrap().join("runscript.txt");
    let mut script_content = String::new();

    script_content.push_str(&format!("force_install_dir \"{}\"\n", install_dir));
    script_content.push_str("login anonymous\n");

    if let Some(game_id) = workshop_game_id {
        script_content.push_str(&format!("workshop_download_item {} {}\n", game_id, app_id));
    } else {
        script_content.push_str(&format!("app_update {} validate\n", app_id));
    }

    script_content.push_str("quit\n");

    std::fs::write(&script_path, script_content)
        .map_err(|e| format!("Failed to write script: {}", e))?;

    let mut cmd = Command::new(&executable_path);

    cmd.current_dir(executable_path.parent().unwrap())
        .stdin(std::process::Stdio::null())
        .arg("+runscript")
        .arg("runscript.txt");

    let status = cmd
        .status()
        .map_err(|e| format!("Failed to start steamcmd: {}", e))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!(
            "SteamCMD exited with error code: {:?}",
            status.code()
        ))
    }
}
