use crate::routes::{health_check, subscribe};
use std::net::TcpListener;

use axum::{
    routing::{get, post},
    Router,
};

pub fn build_server(
    listener: TcpListener,
) -> Result<
    hyper::Server<hyper::server::conn::AddrIncoming, axum::routing::IntoMakeService<Router>>,
    hyper::Error,
> {
    let app = Router::new()
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscribe));

    axum::Server::from_tcp(listener).map(|builder| builder.serve(app.into_make_service()))
}
