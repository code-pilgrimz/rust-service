use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Payment {
    pub id: i64,
    pub amount: f64,
    pub provider: String,
    pub status: String,
    pub reference: String,
}
