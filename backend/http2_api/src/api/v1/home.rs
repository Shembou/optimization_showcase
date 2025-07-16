use std::collections::HashMap;

use axum::{extract::Query, response::IntoResponse};
use tracing::info;

#[utoipa::path(
    get,
    path = "/v1/",
    responses(
        (status = 200, description = "Plain text greeting", body = String)
    ),
    params(
        ("id" = u64, Query, description = "Get the hello world with message id")
    )
)]
pub async fn hello(Query(params): Query<HashMap<String, String>>) -> impl IntoResponse {
    if let Some(id) = params.get("id") {
        info!("Serving GET with query param: id:{} at route /v1/", id);
        return format!("Hello world, id:{}", id);
    }
    info!("Serving GET at route /v1/");
    format!("Hello world!")
}
