use crate::gql_types::Quote;
use async_graphql::*;
use async_graphql::extensions::Logger;

pub type Server1Schema = async_graphql::Schema<Query, EmptyMutation, EmptySubscription>;

pub struct Query;

#[Object]
impl Query {
    /// Returns the sum of a and b
    async fn add(&self, a: i32, b: i32) -> i32 {
        //std::thread::sleep(std::time::Duration::from_millis(500));
        a + b
    }

    async fn quotes(&self, ids: Vec<ID>) -> Vec<Quote> {
        ids.into_iter().map(|id| {
            let plate_number = format!("AA{}BB", &id.to_string());
            Quote {
                id: id.clone(),
                plate_number,
                price: 200 + id.parse::<i32>().unwrap(),
            }
        }).collect()
    }

    async fn quote(&self, id: ID) -> Quote {
        Quote {
            id,
            plate_number: "AA000BB".to_string(),
            price: 200,
        }
    }

    #[graphql(entity)]
    async fn find_quote_by_id(&self, id: ID) -> Quote {
        Quote {
            id,
            plate_number: "AA000BB".to_string(),
            price: 200,
        }
    }
}

pub fn get_schema() -> Server1Schema {
    Schema::build(Query, EmptyMutation, EmptySubscription)
        .extension(Logger)
        .enable_federation()
        .finish()
}
