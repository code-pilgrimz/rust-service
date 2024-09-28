use axum::{Json, extract::Path};
use crate::models::webhook::Webhook;

pub async fn list() -> Json<Vec<Webhook>> {
    Json(vec![])
}

pub async fn get(Path(id): Path<i64>) -> Json<Webhook> {
    Json(Webhook { id, ..Default::default() })
}
