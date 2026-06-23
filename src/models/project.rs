use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Project {
    pub id: i64,
    pub name: String,
    pub description: String,
    pub status: String,
    pub archived: bool,
}
// wip: feature-flags
