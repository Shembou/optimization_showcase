use crate::controller::metrics::HTTP_REQUEST_DURATION;
use axum::{body::Body, extract::MatchedPath, http::Request, middleware::Next, response::Response};
use std::time::Instant;

pub async fn track_metrics(req: Request<Body>, next: Next) -> Response {
    let path = req
        .extensions()
        .get::<MatchedPath>()
        .map(|p| p.as_str())
        .unwrap_or("unknown")
        .to_owned();

    let start = Instant::now();
    let response = next.run(req).await;
    let duration = start.elapsed().as_secs_f64();

    HTTP_REQUEST_DURATION
        .with_label_values(&[&path])
        .observe(duration);

    response
}