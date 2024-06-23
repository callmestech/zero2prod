use crate::routes::{health_check, subscribe};
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
}

impl AppState {
    pub fn pg_pool(&self) -> &sqlx::PgPool {
        &self.pg_pool
    }
}

pub async fn build_server(
    listener: TcpListener,
    pg_pool: sqlx::PgPool,
) -> Result<(), std::io::Error> {
    let state = Arc::new(AppState { pg_pool });
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request| {
                let request_id = match request.headers().get("x-request-id") {
                    Some(value) => value.to_str().unwrap_or_default().to_string(),
                    None =>
                        Uuid::new_v4().to_string(),
                };
                tracing::info_span!("http-request", "method" = ?request.method(), "request_id" = ?request_id, "uri" = ?request.uri())
            }),
        )
        .with_state(state);

    axum::serve(listener, app.into_make_service()).await
}
