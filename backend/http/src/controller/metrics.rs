use axum::{http::StatusCode, response::IntoResponse};
use lazy_static::lazy_static;
use prometheus::{Encoder, HistogramVec, TextEncoder, register_histogram_vec};

lazy_static! {
    pub static ref HTTP_REQUEST_DURATION: HistogramVec = register_histogram_vec!(
        "http_request_duration_seconds",
        "HTTP request latencies in seconds",
        &["route"]
    )
    .unwrap();
}

pub async fn metrics_handler() -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    let mut buffer = Vec::new();

    if let Err(e) = encoder.encode(&metric_families, &mut buffer) {
        return (StatusCode::INTERNAL_SERVER_ERROR, format!("Error: {}", e));
    }

    let output = String::from_utf8(buffer).unwrap_or_default();
    (StatusCode::OK, output)
}
