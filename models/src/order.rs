use std::{collections::HashMap, sync::Arc};

use async_graphql::{Object, Context, dataloader::{Loader, DataLoader}, async_trait, Enum, futures_util::{FutureExt, TryFutureExt, TryStreamExt}, FieldError};
use chrono::{DateTime, Utc};
use serde::{Serialize};
use sqlx::{Pool, Postgres, FromRow, postgres::{PgRow, PgDatabaseError}, Row};
use crate::{order_status::OrderStatus};



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

    pub async fn id(&self, ctx: &Context<'_>, id: i32) -> Result<i32, FieldError> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let loader = ctx.data_unchecked::<DataLoader<OrderLoader>>();
        
        let order = loader.load_one(id).await?;

        if let Some(order) = order {
            Ok(order.id)
        } else {

            let order: Order = sqlx::query_as("SELECT * FROM orders WHERE id = $1")
                .bind(id)
                .fetch_one(pool)
                .await?;

            loader.feed_one(id, order.clone()).await;

            Ok(order.id)
        }
    }
    

    pub async fn total(&self, ctx: &Context<'_>, id: i32) -> Result<f64, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<OrderLoader>>();
        
        let order = loader.load_one(id).await?;

        if let Some(order) = order {
            Ok(order.total)
        } else {
            Err("Order not found".into())
        }
    }

    pub async fn status(&self, ctx: &Context<'_>, id: i32) -> Result<OrderStatus, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<OrderLoader>>();
        
        let order = loader.load_one(id).await?;

        if let Some(order) = order {
            Ok(order.status)
        } else {
            Err("Order not found".into())
        }
    }

    pub async fn updated_at(&self, ctx: &Context<'_>, id: i32) -> Result<DateTime<Utc>, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<OrderLoader>>();
        
        let order = loader.load_one(id).await?;

        if let Some(order) = order {
            Ok(order.updated_at)
        } else {
            Err("Order not found".into())
        }
    }

    pub async fn created_at(&self, ctx: &Context<'_>, id: i32) -> Result<DateTime<Utc>, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<OrderLoader>>();
        
        let order = loader.load_one(id).await?;

        if let Some(order) = order {
            Ok(order.created_at)
        } else {
            Err("Order not found".into())
        }
    }
    
}



pub struct OrderLoader {
    pool: sqlx::PgPool,
}

impl OrderLoader {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self {
            pool
        }
    }
}

#[async_trait::async_trait]
impl Loader<i32> for OrderLoader {
    type Value = Order;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        Ok(sqlx::query_as("SELECT * FROM orders WHERE id = ANY($1)")
            .bind(keys)
            .fetch(&self.pool)
            .map_ok(|order: Order| (order.id, order))
            .map_err(Arc::new)
            .try_collect::<HashMap<i32, Order>>()
            .await?
        )
    }
}