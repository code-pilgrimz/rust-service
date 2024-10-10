use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct ApiKey {
    pub id: i64,
    pub name: String,
    pub prefix: String,
    pub revoked: bool,
}
