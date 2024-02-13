pub mod configuration;
pub mod routes;
pub mod startup;

use std::net::TcpListener;

use axum::{
    http::StatusCode,
    routing::{get, post},
    Form, Router,
};

async fn health_check() -> StatusCode {
    StatusCode::OK
}

#[derive(serde::Deserialize)]
struct FormData {
    name: String,
    email: String,
}

async fn subscribe(Form(data): Form<FormData>) -> StatusCode {
    println!("name {} email {}", data.name, data.email);
    StatusCode::OK
}

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
