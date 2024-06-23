use crate::routes::{health_check, subscribe};
use std::sync::Arc;

use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;

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
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    axum::serve(listener, app.into_make_service()).await
}
