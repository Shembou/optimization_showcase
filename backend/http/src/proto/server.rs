use deadpool_redis::{Pool, redis::AsyncCommands};
use tonic::{Request, Response, Status};
use tracing::info;

use crate::{
    config::db::get_db_pool, proto::user::{user_server::User, Empty, UserMessage, UsersList}, sql::users::DbUserQueries
};



impl From<crate::model::user::User> for UserMessage {
    fn from(user: crate::model::user::User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            language: user.language.unwrap_or_default(),
            bio: user.bio.unwrap_or_default(),
            version: user.version.unwrap_or_default(),
        }
    }
}

#[derive(Debug)]
pub struct MyService {
    redis_pool: Pool,
}

impl MyService {
    pub fn new(redis_pool: Pool) -> Self {
        Self { redis_pool }
    }
}

#[tonic::async_trait]
impl User for MyService {
    async fn user_rpc(
        &self,
        request: Request<Empty>,
    ) -> Result<Response<UsersList>, Status> {
        info!("Got a request: {:?}", request);

        let mut conn = self
            .redis_pool
            .get()
            .await
            .map_err(|e| Status::internal(format!("Redis error: {}", e)))?;
        let cache_key = "users_cache";
        let cached: Option<String> = conn.get(&cache_key).await.ok();

        if let Some(json) = cached {
            info!("Cache HIT");
            let response = serde_json::from_str(&json)
                .map_err(|e| Status::internal(format!("Deserialization error: {}", e)))?;
            return Ok(Response::new(response));
        }

        info!("Cache MISS");
        let app_users = DbUserQueries::get_users(get_db_pool())
            .await
            .map_err(|e| Status::internal(format!("DB error: {}", e)))?;

        let proto_users = app_users.into_iter().map(Into::into).collect();

        let response = UsersList {
            users: proto_users,
        };

        let json_response = serde_json::to_string(&response)
            .map_err(|e| Status::internal(format!("Serialization error: {}", e)))?;

        let _: () = conn
            .set_ex(&cache_key, json_response, 300)
            .await
            .map_err(|e| Status::internal(format!("Redis SET error: {}", e)))?;

        Ok(Response::new(response))
    }
}