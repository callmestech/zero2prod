use axum::{http::StatusCode, routing::get, Router};

async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub fn build_server(
) -> hyper::Server<hyper::server::conn::AddrIncoming, axum::routing::IntoMakeService<Router>> {
    let app = Router::new().route("/health_check", get(health_check));
    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap()).serve(app.into_make_service())
}
