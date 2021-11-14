use async_graphql::*;

pub struct Quote {
    pub id: ID,
    pub plate_number: String,
    pub price: i32,
}

#[Object]
impl Quote {
    async fn id(&self) -> &ID {
        &self.id
    }

    async fn plate_number(&self) -> &str {
        &self.plate_number
    }

    async fn price(&self) -> i32 {
        self.price
    }
}
