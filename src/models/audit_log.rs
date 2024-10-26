use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct AuditLog {
    pub id: i64,
    pub action: String,
    pub actor_id: i64,
    pub target: String,
}
