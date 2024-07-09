use axum::{Json, extract::Path};
use crate::models::task::Task;

pub async fn list() -> Json<Vec<Task>> {
    Json(vec![])
}

pub async fn get(Path(id): Path<i64>) -> Json<Task> {
    Json(Task { id, ..Default::default() })
}
