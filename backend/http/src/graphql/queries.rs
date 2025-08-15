use juniper::graphql_object;

use crate::{config::db::get_db_pool, model::user::User, sql::users::DbUserQueries};

#[derive(Clone)]
pub struct Human {
    pub name: String,
    pub age: i32,
}

#[graphql_object]
impl Human {
    fn name(&self) -> &str {
        &self.name
    }

    fn age(&self) -> i32 {
        self.age
    }
}

pub struct Query;

#[graphql_object]
impl Query {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    fn humans() -> Vec<Human> {
        vec![
            Human {
                name: "Josh".to_string(),
                age: 33,
            },
            Human {
                name: "Jenifer".to_string(),
                age: 25,
            },
        ]
    }

    async fn users() -> Result<Vec<User>, juniper::FieldError> {
        let users = DbUserQueries::get_users(get_db_pool()).await?;
        Ok(users)
    }
}
