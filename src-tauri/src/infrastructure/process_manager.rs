use std::process::{Child, Command, Stdio};
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Emitter};

pub struct ProcessManager {
    pub processes: Arc<Mutex<Vec<Child>>>,
}

impl ProcessManager {
    pub fn new() -> Self {
        Self {
            processes: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn spawn_process(&self, cmd: &mut Command, app: &AppHandle) -> Result<u32, String> {
        cmd.stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped());

        #[cfg(target_os = "windows")]
        {
            use std::os::windows::process::CommandExt;
            cmd.creation_flags(0x08000000);
        }

        let child = cmd.spawn().map_err(|e| e.to_string())?;

        let pid = child.id();
        let _ = app.emit("process-spawned", pid);

        if let Ok(mut procs) = self.processes.lock() {
            procs.push(child);
        } else {
            return Err("Failed to lock process list".to_string());
        }

        Ok(pid)
    }

    pub fn kill_all(&self) {
        if let Ok(mut procs) = self.processes.lock() {
            for child in procs.iter_mut() {
                let _ = child.kill();
            }
            procs.clear();
        }
    }

    pub fn remove_process(&self, pid: u32) {
        if let Ok(mut procs) = self.processes.lock() {
            procs.retain(|p| p.id() != pid);
        }
    }
}

impl Default for ProcessManager {
    fn default() -> Self {
        Self::new()
    }
}
