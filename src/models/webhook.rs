use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Webhook {
    pub id: i64,
    pub url: String,
    pub event: String,
    pub active: bool,
    pub secret: String,
}
