use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DownloadStatus {
    Pending,
    Downloading,
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadItem {
    pub id: String,
    pub steam_id: String,
    pub name: String,
    pub status: DownloadStatus,
    pub install_path: Option<String>,
    pub created_at: u64,
}

pub struct QueueState {
    pub queue: Arc<Mutex<VecDeque<DownloadItem>>>,
}

impl QueueState {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn add_item(&self, steam_id: String, name: String) -> String {
        let mut q = self.queue.lock().unwrap();
        let id = Uuid::new_v4().to_string();
        q.push_back(DownloadItem {
            id: id.clone(),
            steam_id,
            name,
            status: DownloadStatus::Pending,
            install_path: None,
            created_at: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });
        id
    }

    pub fn get_queue(&self) -> Vec<DownloadItem> {
        let q = self.queue.lock().unwrap();
        q.iter().cloned().collect()
    }
}
