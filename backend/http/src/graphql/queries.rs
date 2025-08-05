use juniper::graphql_object;


pub struct Query;

#[graphql_object]
impl Query {
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }
}