use std::collections::HashMap;

use axum::{Extension, extract::Query, http::StatusCode, response::Json};
use deadpool_redis::{Pool as RedisPool, redis::AsyncCommands};
use sqlx::PgPool;
use serde_json::{Value, json};
use tracing::info;

use crate::sql::users::DbUserQueries;

pub async fn hello(
    Query(params): Query<HashMap<String, String>>,
    Extension(redis_pool): Extension<RedisPool>,
    Extension(db_pool): Extension<&PgPool>,
) -> Result<Json<Value>, StatusCode> {
    if let Some(id) = params.get("id") {
        info!("Serving GET with query param: id:{} at route /api", id);
        return Ok(Json(json!({ "data": format!("Hello world, id:{}", id)})));
    }

    let mut conn = redis_pool
        .get()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Ok(Some(users_json)) = conn.get::<_, Option<String>>("users_cache").await {
        info!("Serving GET from cache at route /api/");
        let users_value: Value =
            serde_json::from_str(&users_json).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        return Ok(Json(json!({ "data": users_value })));
    }

    let users = DbUserQueries::get_users(db_pool).await?;

    if let Ok(users_json) = serde_json::to_string(&users) {
        let _: Result<(), _> = conn.set_ex("users_cache", users_json, 300).await;
    }

    info!("Serving GET at route /api/");
    Ok(Json(json!({ "data": users})))
}
