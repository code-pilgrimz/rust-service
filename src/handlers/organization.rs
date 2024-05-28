use axum::{Json, extract::Path};
use crate::models::organization::Organization;

pub async fn list() -> Json<Vec<Organization>> {
    Json(vec![])
}

pub async fn get(Path(id): Path<i64>) -> Json<Organization> {
    Json(Organization { id, ..Default::default() })
}
