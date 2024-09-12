use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Notification {
    pub id: i64,
    pub kind: String,
    pub message: String,
    pub read: bool,
}
