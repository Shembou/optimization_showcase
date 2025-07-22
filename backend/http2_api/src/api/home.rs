use std::collections::HashMap;

use axum::{extract::Query, response::Json};
use serde_json::{Value, json};
use tracing::info;

#[utoipa::path(
    get,
    path = "/api",
    responses(
        (status = 200, description = "Plain text greeting", body = String)
    ),
    params(
        ("id" = u64, Query, description = "Get the hello world with message id")
    )
)]
pub async fn hello(Query(params): Query<HashMap<String, String>>) -> Json<Value> {
    if let Some(id) = params.get("id") {
        info!("Serving GET with query param: id:{} at route /api", id);
        return Json(json!({ "data": format!("Hello world, id:{}", id)}));
    }
    info!("Serving GET at route /api/");
    Json(json!({ "data": "Hello world"}))
}
