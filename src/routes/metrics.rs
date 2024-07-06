use autometrics::prometheus_exporter;
use axum::response::{IntoResponse, Response};

pub async fn get_metrics() -> Response {
    prometheus_exporter::encode_http_response().into_response()
}
