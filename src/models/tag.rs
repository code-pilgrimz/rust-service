use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Tag {
    pub id: i64,
    pub label: String,
    pub color: String,
}
