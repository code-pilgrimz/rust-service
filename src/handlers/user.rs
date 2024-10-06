use axum::{Json, extract::Path};
use crate::models::user::User;

pub async fn list() -> Json<Vec<User>> {
    Json(vec![])
}

pub async fn get(Path(id): Path<i64>) -> Json<User> {
    Json(User { id, ..Default::default() })
}
// wip: soft-delete
