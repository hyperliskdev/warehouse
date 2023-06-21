use std::{collections::HashMap, sync::Arc};

use async_graphql::{Object, Context, dataloader::{Loader, DataLoader}, async_trait, Enum};
use serde::{Serialize};
use sqlx::{types::{chrono::{Utc, DateTime}}, Pool, Postgres, FromRow};

use crate::{order_status::OrderStatus, customer::{Customer, self}};



#[derive(Debug, FromRow, Clone)]
pub struct Order {
    pub id: i32,
    pub customer_id: String,
    pub total: f64,
    pub status: OrderStatus,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[Object]
impl Order {

    pub async fn id(&self, ctx: &Context<'_>, id: i32) -> Result<i32, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let loader = ctx.data_unchecked::<DataLoader<OrderLoader>>();
        

        if let Some(order) = loader.load_one(id).await? {
            
            Ok(order.id);

        } else {

            let (order, ) = sqlx::query_as::<Order>
                ("SELECT * FROM orders WHERE id = $1")
                    .bind(id)
                    .fetch_one(pool)
                    .await?;
            
            loader.feed_one(id, order);

            Ok(order.id)
        }  
    }
}



struct OrderLoader {
    pool: sqlx::PgPool,
}

#[async_trait::async_trait]
impl Loader<i32> for OrderLoader {
    type Value = Order;
    type Error = Arc<sqlx::Error>;


    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {

        Ok(sqlx::query("SELECT * FROM orders WHERE id = ANY($1)")
            .bind(keys)
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(|order| (order.0, order))
            .collect()
        )
    }
}