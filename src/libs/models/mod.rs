pub mod math;
pub mod string;

use async_graphql::{ 
    Schema, 
    MergedObject, 
    EmptyMutation, 
    EmptySubscription, 
    extensions::ApolloTracing 
};

use self::math::MathQuery;
use self::string::StringQuery;

#[derive(MergedObject, Default)]
pub struct Query(MathQuery, StringQuery);

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub fn create_schema() -> AppSchema {
    Schema::build(
        Query::default(),
        EmptyMutation,
        EmptySubscription
    )
    .extension(ApolloTracing)
    .finish()
}