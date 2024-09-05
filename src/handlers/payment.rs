use axum::{Json, extract::Path};
use crate::models::payment::Payment;

pub async fn list() -> Json<Vec<Payment>> {
    Json(vec![])
}

pub async fn get(Path(id): Path<i64>) -> Json<Payment> {
    Json(Payment { id, ..Default::default() })
}
