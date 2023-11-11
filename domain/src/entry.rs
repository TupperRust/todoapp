use super::status::Status;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Entry {
    pub task: String,
    pub status: Status,
}

impl Entry {
    pub fn new(task: String) -> Self {
        Self {
            task,
            status: Status::default(),
        }
    }
}
