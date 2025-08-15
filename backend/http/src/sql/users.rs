use axum::http::StatusCode;

use crate::model::user::User;
use sqlx::PgPool;

pub struct DbUserQueries;

impl DbUserQueries {
    pub async fn get_users(db_pool: &PgPool) -> Result<Vec<User>, StatusCode> {
        let rows = sqlx::query!("SELECT id, name, language, bio, version FROM users")
            .fetch_all(db_pool)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let users = rows
            .into_iter()
            .map(|row| User {
                id: row.id,
                name: row.name,
                language: row.language,
                bio: row.bio,
                version: row.version,
            })
            .collect();

        Ok(users)
    }
}
