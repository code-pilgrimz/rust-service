use axum::{Json, extract::Path};
use crate::models::api_key::ApiKey;

pub async fn list() -> Json<Vec<ApiKey>> {
    Json(vec![])
}

pub async fn get(Path(id): Path<i64>) -> Json<ApiKey> {
    Json(ApiKey { id, ..Default::default() })
}
