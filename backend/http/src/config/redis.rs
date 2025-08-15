use std::env;

use deadpool_redis::{Config, Pool, Runtime};

pub fn get_redis_pool() -> Pool {
    let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set");
    let cfg = Config::from_url(redis_url);
    cfg.create_pool(Some(Runtime::Tokio1)).expect("Cannot create Redis pool")
}
