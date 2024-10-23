use crate::gql_types::{Claim, Quote, Commission, all_commissions};
use async_graphql::extensions::Logger;
use async_graphql::*;

pub type Server2Schema = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;

pub struct Query;

#[Object]
impl Query {
    /// Returns the sum of a and b
    async fn subtract(&self, a: i32, b: i32) -> i32 {
        //std::thread::sleep(std::time::Duration::from_millis(500));
        a - b
    }

    async fn commissions(&self) -> Vec<Commission> {
        all_commissions()
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
