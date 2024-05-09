use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct User {
    pub id: i64,
    pub email: String,
    pub full_name: String,
    pub is_active: bool,
    pub hashed_password: String,
}
