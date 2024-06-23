#[tracing::instrument(name = "health_check")]
pub async fn health_check() -> hyper::StatusCode {
    hyper::StatusCode::OK
}
