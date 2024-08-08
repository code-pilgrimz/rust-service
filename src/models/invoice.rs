use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Invoice {
    pub id: i64,
    pub number: String,
    pub amount: f64,
    pub currency: String,
    pub paid: bool,
}
