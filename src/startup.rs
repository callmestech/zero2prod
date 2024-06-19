use crate::routes::{health_check, subscribe};
use std::{net::TcpListener, sync::Arc};

use axum::{
    routing::{get, post},
    Router,
};

pub struct AppState {
    pg_pool: sqlx::PgPool,
}

impl AppState {
    pub fn pg_pool(&self) -> &sqlx::PgPool {
        &self.pg_pool
    }
}

pub fn build_server(
    listener: TcpListener,
    pg_pool: sqlx::PgPool,
) -> Result<
    hyper::Server<hyper::server::conn::AddrIncoming, axum::routing::IntoMakeService<Router>>,
    hyper::Error,
> {
    let state = Arc::new(AppState { pg_pool });
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe))
        .with_state(state);

    axum::Server::from_tcp(listener).map(|builder| builder.serve(app.into_make_service()))
}
