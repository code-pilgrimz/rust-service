mod models;
mod handlers;

use axum::{Router, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(|| async { "ok" }))
        .route("/users", get(handlers::user::list))
        .route("/users/:id", get(handlers::user::get))
        .route("/organizations", get(handlers::organization::list))
        .route("/organizations/:id", get(handlers::organization::get))
        .route("/projects", get(handlers::project::list))
        .route("/projects/:id", get(handlers::project::get))
        .route("/tasks", get(handlers::task::list))
        .route("/tasks/:id", get(handlers::task::get))
        .route("/comments", get(handlers::comment::list))
        .route("/comments/:id", get(handlers::comment::get))
        .route("/tags", get(handlers::tag::list))
        .route("/tags/:id", get(handlers::tag::get))
        .route("/invoices", get(handlers::invoice::list))
        .route("/invoices/:id", get(handlers::invoice::get))
        .route("/payments", get(handlers::payment::list))
        .route("/payments/:id", get(handlers::payment::get))
        .route("/notifications", get(handlers::notification::list))
        .route("/notifications/:id", get(handlers::notification::get))
        .route("/webhooks", get(handlers::webhook::list))
        .route("/webhooks/:id", get(handlers::webhook::get))
        .route("/api_keys", get(handlers::api_key::list))
        .route("/api_keys/:id", get(handlers::api_key::get))
        .route("/audit_logs", get(handlers::audit_log::list))
        .route("/audit_logs/:id", get(handlers::audit_log::get));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
