use axum::{Json, extract::Path};
use crate::models::project::Project;

pub async fn list() -> Json<Vec<Project>> {
    Json(vec![])
}

pub async fn get(Path(id): Path<i64>) -> Json<Project> {
    Json(Project { id, ..Default::default() })
}
// wip: auth-tokens
