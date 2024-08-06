use axum::{Json, extract::Path};
use crate::models::tag::Tag;

pub async fn list() -> Json<Vec<Tag>> {
    Json(vec![])
}

pub async fn get(Path(id): Path<i64>) -> Json<Tag> {
    Json(Tag { id, ..Default::default() })
}
