use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Clone, Copy, Deserialize, Serialize)]
pub enum Status {
    #[default]
    Todo,
    Done,
}
