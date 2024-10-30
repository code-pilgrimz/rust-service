use axum::{Json, extract::Path};
use crate::models::audit_log::AuditLog;

pub async fn list() -> Json<Vec<AuditLog>> {
    Json(vec![])
}

pub async fn get(Path(id): Path<i64>) -> Json<AuditLog> {
    Json(AuditLog { id, ..Default::default() })
}
