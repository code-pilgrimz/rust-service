use axum::{Json, extract::Path};
use crate::models::notification::Notification;

pub async fn list() -> Json<Vec<Notification>> {
    Json(vec![])
}

pub async fn get(Path(id): Path<i64>) -> Json<Notification> {
    Json(Notification { id, ..Default::default() })
}
