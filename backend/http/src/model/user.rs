use juniper::graphql_object;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub language: Option<String>,
    pub bio: Option<String>,
    pub version: Option<i32>,
}

#[graphql_object]
impl User {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &str {
        &self.name
    }

    fn language(&self) -> Option<&str> {
        self.language.as_deref()
    }

    fn bio(&self) -> Option<&str> {
        self.bio.as_deref()
    }

    fn version(&self) -> Option<i32> {
        self.version
    }
}