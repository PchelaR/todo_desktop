use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: usize,
    pub description: String,
    pub done: bool,
}
