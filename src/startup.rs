use crate::{
    email_client::EmailClient,
    routes::{get_metrics, health_check, subscribe},
};
use std::sync::Arc;

use axum::{
    extract::Request,
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use uuid::Uuid;

pub struct AppState {
    pg_pool: sqlx::PgPool,
    email_client: EmailClient,
}

impl AppState {
    pub fn pg_pool(&self) -> &sqlx::PgPool {
        &self.pg_pool
    }

    pub fn email_client(&self) -> &EmailClient {
        &self.email_client
    }
}

pub async fn build_server(
    listener: TcpListener,
    pg_pool: sqlx::PgPool,
    email_client: EmailClient,
) -> Result<(), std::io::Error> {
    let state = Arc::new(AppState {
        pg_pool,
        email_client,
    });
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .route("/metrics", get(get_metrics))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request| {
                let request_id = match request.headers().get("x-request-id") {
                    Some(value) => value.to_str().unwrap_or_default().to_string(),
                    None =>
                        Uuid::new_v4().to_string(),
                }; 
                let path = request.uri().path().to_string();
                if path == "/metrics" || path == "/health_check" {
                    tracing::trace_span!("http-request", "method" = ?request.method(), "request_id" = ?request_id, "uri" = ?request.uri())
                } else {
                tracing::info_span!("http-request", "method" = ?request.method(), "request_id" = ?request_id, "uri" = ?request.uri())}
            }),
        )
        .with_state(state);

    axum::serve(listener, app.into_make_service()).await
}
