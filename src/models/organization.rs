use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Organization {
    pub id: i64,
    pub name: String,
    pub slug: String,
    pub plan: String,
    pub seats: i64,
}
