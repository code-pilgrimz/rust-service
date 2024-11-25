use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Task {
    pub id: i64,
    pub title: String,
    pub priority: i64,
    pub done: bool,
    pub due_date: String,
}
// wip: race-condition
