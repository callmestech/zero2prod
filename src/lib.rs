use std::net::TcpListener;

use axum::{http::StatusCode, routing::get, Router};

async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub fn build_server(
    listener: TcpListener,
) -> Result<
    hyper::Server<hyper::server::conn::AddrIncoming, axum::routing::IntoMakeService<Router>>,
    hyper::Error,
> {
    let app = Router::new().route("/health_check", get(health_check));
    axum::Server::from_tcp(listener).map(|builder| builder.serve(app.into_make_service()))
}
