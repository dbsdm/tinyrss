use std::path::PathBuf;

use crate::worker::db;

pub enum ToApp {
    WorkerError { error: WorkerError },
    UpdateFeed { items: Vec<db::Item> },
    FeedUpdateProgress { progress: f32 },
    ImportProgress { progress: f32 },
    UpdateChannels { channels: Vec<db::Channel> },
}

pub enum ToWorker {
    Startup,
    Shutdown,
    UpdateFeed,
    AddChannel { link: String },
    EditChannel { id: String, title: String },
    SetDismissed { id: String, dismissed: bool },
    DismissAll,
    Unsubscribe { id: String },
    ImportChannels { path: Option<PathBuf> },
    ExportChannels,
}

pub struct WorkerError {
    pub description: String,
    pub error_message: String,
}

impl WorkerError {
    pub fn new(description: impl Into<String>, error_message: impl Into<String>) -> Self {
        Self {
            description: description.into(),
            error_message: error_message.into(),
        }
    }
}
