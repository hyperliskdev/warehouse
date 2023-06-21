use std::sync::Arc;

use async_graphql::{Object, Context, dataloader::{DataLoader, Loader}, async_trait};
use chrono::{DateTime, Utc};
use sqlx::{FromRow, Pool, Postgres};



#[derive(Debug, FromRow, Clone)]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}



#[Object]
impl Customer {

    pub async fn id(&self, ctx: &Context<'_>, id: i32) -> Result<i32, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let loader = ctx.data_unchecked::<DataLoader<CustomerLoader>>();
        let (id, ) = sqlx::query_as::<_, (i32, )>("SELECT id FROM customers WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await?;

        Ok(id)
    }

}


struct CustomerLoader {
    pool: sqlx::PgPool,
}

#[async_trait::async_trait]
impl Loader<i32> for CustomerLoader {
    type Value = Customer;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &[i32]) -> Result<std::collections::HashMap<i32, Self::Value>, Self::Error> {
        let customers = sqlx::query_as::<_, Customer>("SELECT * FROM customers WHERE id = ANY($1)")
            .bind(keys)
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|customer| (customer.id, customer))
            .collect();

        Ok(customers)
    }
    

}