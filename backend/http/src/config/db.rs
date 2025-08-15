use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;
use tokio::sync::OnceCell;
use tracing::info;

static DB_POOL: OnceCell<PgPool> = OnceCell::const_new();

pub async fn init_db_pool() {
    info!("Attempting to read DATABASE_URL env");
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    info!("Creating DB_POOL");

    let pool = PgPoolOptions::new()
        .max_connections(100)
        .connect(&db_url)
        .await
        .expect("Failed to create DB Pool");

    DB_POOL.set(pool).expect("DB_POOL already initialized");
}

pub fn get_db_pool() -> &'static PgPool {
    DB_POOL.get().expect("DB_POOL not initialized")
}
