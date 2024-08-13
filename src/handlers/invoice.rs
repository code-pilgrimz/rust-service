use axum::{Json, extract::Path};
use crate::models::invoice::Invoice;

pub async fn list() -> Json<Vec<Invoice>> {
    Json(vec![])
}

pub async fn get(Path(id): Path<i64>) -> Json<Invoice> {
    Json(Invoice { id, ..Default::default() })
}
