use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub struct ProcessManagerState(
    pub Arc<Mutex<crate::infrastructure::process_manager::ProcessManager>>,
);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Status {
    Pending,
    Downloading { progress: f32 },
    Completed,
    Failed(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DownloadItem {
    pub id: String,
    pub steam_id: String,
    pub name: String,
    pub status: Status,
    pub install_path: Option<String>,
    pub created_at: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QueueState {
    pub items: Vec<DownloadItem>,
}

impl QueueState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_item(mut self, item: DownloadItem) -> Self {
        self.items.push(item);
        self
    }

    pub fn update_status(mut self, id: &str, status: Status) -> Self {
        if let Some(item) = self.items.iter_mut().find(|i| i.id == id) {
            item.status = status;
        }
        self
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryItem {
    pub id: String,
    pub steam_id: String,
    pub name: String,
    pub install_path: String,
    pub timestamp: u64,
}

#[derive(Clone, Default)]
pub struct HistoryState(pub Arc<Mutex<Vec<HistoryItem>>>);
