use crate::gql_types::{Quote, Claim};
use async_graphql::*;
use async_graphql::extensions::Logger;

pub type Server2Schema = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;

pub struct Query;

#[Object]
impl Query {
    /// Returns the sum of a and b
    async fn subtract(&self, a: i32, b: i32) -> i32 {
        a - b
    }

    #[graphql(entity)]
    async fn quote(&self, id: ID) -> Quote {
        Quote { id }
    }

    #[graphql(entity)]
    async fn claim(&self, id: ID) -> Claim {
        Claim { id }
    }
}

pub fn get_schema() -> Server2Schema {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .enable_federation()
        .extension(Logger)
        .finish()
}
