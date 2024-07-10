use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Comment {
    pub id: i64,
    pub body: String,
    pub author_id: i64,
    pub edited: bool,
}
