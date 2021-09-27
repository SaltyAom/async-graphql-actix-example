use async_graphql::Object;

#[derive(Default)]
pub struct MathQuery;

#[Object]
impl MathQuery {
    pub async fn add(&self, a: i32, b: i32) -> i32 {
        a + b
    }
}
