use crate::infrastructure::bin_loader::get_steamcmd_path;
use crate::infrastructure::process_manager::ProcessManager;
use std::io::{BufRead, BufReader, Write};
use std::process::Stdio;
use std::sync::mpsc;
use std::thread;
use tauri::{AppHandle, Emitter};

pub fn execute_steamcmd_with_progress(
    app: &AppHandle,
    commands: Vec<String>,
    process_manager: &ProcessManager,
    item_id: String,
) -> Result<mpsc::Receiver<bool>, String> {
    let steamcmd_path = get_steamcmd_path(app)?;

    let mut cmd = std::process::Command::new(&steamcmd_path);
    cmd.stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        cmd.creation_flags(0x08000000);
    }

    let mut child = cmd.spawn().map_err(|e| e.to_string())?;

    let pid = child.id();
    let _ = app.emit("process-spawned", pid);

    let stdin = child.stdin.take().ok_or("Failed to capture stdin")?;
    let stdout = child.stdout.take().ok_or("Failed to capture stdout")?;
    let stderr = child.stderr.take().ok_or("Failed to capture stderr")?;

    let _app_clone = app.clone();
    let _item_id_clone = item_id.clone();
    thread::spawn(move || {
        let mut writer = stdin;
        for command in commands {
            let _ = writeln!(writer, "{}", command);
        }
        let _ = writeln!(writer, "quit");
    });

    let app_out = app.clone();
    let item_id_out = item_id.clone();
    thread::spawn(move || {
        let reader = BufReader::new(stdout);
        for line in reader.lines() {
            if let Ok(l) = line {
                let _ = app_out.emit("terminal-output", l.clone());

                if let Some(progress) = parse_progress(&l) {
                    let _ = app_out.emit(
                        "download-progress",
                        serde_json::json!({
                            "id": item_id_out,
                            "progress": progress
                        }),
                    );
                }
            }
        }
    });

    let app_err = app.clone();
    thread::spawn(move || {
        let reader = BufReader::new(stderr);
        for line in reader.lines() {
            if let Ok(l) = line {
                let _ = app_err.emit("terminal-output", l);
            }
        }
    });

    let (tx, rx) = mpsc::channel();
    let processes_clone = process_manager.processes.clone();

    if let Ok(mut procs) = process_manager.processes.lock() {
        procs.push(child);
        let idx = procs.len() - 1;

        thread::spawn(move || {
            let success = if let Ok(mut p) = processes_clone.lock() {
                if let Some(child) = p.get_mut(idx) {
                    child.wait().map(|s| s.success()).unwrap_or(false)
                } else {
                    false
                }
            } else {
                false
            };
            let _ = tx.send(success);
        });

        return Ok(rx);
    }

    Err("Failed to lock process list".to_string())
}

fn parse_progress(line: &str) -> Option<f32> {
    if line.contains("progress:") {
        let parts: Vec<&str> = line.split("progress:").collect();
        if parts.len() > 1 {
            let num_part = parts[1].split('%').next()?;
            return num_part.trim().parse::<f32>().ok();
        }
    }
    None
}
