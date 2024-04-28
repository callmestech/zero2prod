pub async fn health_check() -> hyper::StatusCode {
    hyper::StatusCode::OK
}
