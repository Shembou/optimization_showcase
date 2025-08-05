use juniper::graphql_object;

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
}
