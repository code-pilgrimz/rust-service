use axum::{Json, extract::Path};
use crate::models::comment::Comment;

pub async fn list() -> Json<Vec<Comment>> {
    Json(vec![])
}

pub async fn get(Path(id): Path<i64>) -> Json<Comment> {
    Json(Comment { id, ..Default::default() })
}
