use async_graphql::*;

pub struct Quote {
    pub id: ID,
}

#[Object(extends)]
impl Quote {
    #[graphql(external)]
    async fn id(&self) -> &ID {
        &self.id
    }
    async fn commissions(&self) -> i32 {
        if self.id.parse::<i32>().unwrap() >= 100 {
            0
        } else {
            20
        }
    }
}
