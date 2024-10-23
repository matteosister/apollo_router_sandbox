use async_graphql::*;

#[derive(SimpleObject)]
pub struct Commission {
    id: ID,
    amount: i32
}

pub fn all_commissions() -> Vec<Commission> {
    vec![
        Commission {id: 1.into(), amount: 0},
        Commission {id: 2.into(), amount: 20}
    ]
}

pub struct Quote {
    pub id: ID,
}

#[Object(extends)]
impl Quote {
    #[graphql(external)]
    async fn id(&self) -> &ID {
        &self.id
    }
    async fn commissions(&self) -> Commission {
        if self.id.parse::<i32>().unwrap() >= 100 {
            Commission {id: 1.into(), amount: 0}
        } else {
            Commission {id: 2.into(), amount: 20}
        }
    }
}

pub struct Claim {
    pub id: ID,
}

#[Object(extends)]
impl Claim {
    #[graphql(external)]
    async fn id(&self) -> &ID {
        &self.id
    }
    async fn lost(&self) -> i32 {
        if self.id.parse::<i32>().unwrap() >= 100 {
            10
        } else {
            40
        }
    }
}
