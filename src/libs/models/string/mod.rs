use async_graphql::Object;

#[derive(Default)]
pub struct StringQuery;

#[Object]
impl StringQuery {
    async fn echo(&self, message: String) -> String {
        message
    }
}
