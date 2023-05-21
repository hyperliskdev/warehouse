use std::collections::HashMap;

use async_graphql::{Object, Context, FieldError, dataloader::{DataLoader, Loader}, InputObject, async_trait, futures_util::TryStreamExt};



#[derive(Clone, Debug, Default, sqlx::FromRow)]
pub struct Order {
    pub id: i32,
    pub code: i32,
    pub title: String,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[Object]
impl Order {

    async fn id(&self, ctx: &Context<'_>) -> Result<i32, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<OrderLoader>>();
        let order = loader.load_one(self.id).await?;

        if let Some(order) = order {
            Ok(order.id)
        } else {
            Err(FieldError::new("Order not found"))
        }
    }

    async fn code(&self, ctx: &Context<'_>) -> Result<i32, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<OrderLoader>>();
        let order = loader.load_one(self.id).await?;

        if let Some(order) = order {
            Ok(order.code)
        } else {
            Err(FieldError::new("Order not found"))
        }
    }

    async fn title(&self, ctx: &Context<'_>) -> Result<String, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<OrderLoader>>();
        let order = loader.load_one(self.id).await?;

        if let Some(order) = order {
            Ok(order.title)
        } else {
            Err(FieldError::new("Order not found"))
        }
        
    }

    async fn description(&self, ctx: &Context<'_>) -> Result<Option<String>, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<OrderLoader>>();
        let order = loader.load_one(self.id).await?;

        if let Some(order) = order {
            Ok(order.description)
        } else {
            Err(FieldError::new("Order not found"))
        }
    }

    async fn created_at(&self, ctx: &Context<'_>) -> Result<chrono::NaiveDateTime, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<OrderLoader>>();
        let order = loader.load_one(self.id).await?;

        if let Some(order) = order {
            Ok(order.created_at)
        } else {
            Err(FieldError::new("Order not found"))
        }
    }

    async fn updated_at(&self, ctx: &Context<'_>) -> Result<chrono::NaiveDateTime, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<OrderLoader>>();
        let order = loader.load_one(self.id).await?;

        if let Some(order) = order {
            Ok(order.updated_at)
        } else {
            Err(FieldError::new("Order not found"))
        }
    }

} 


#[derive(Clone, Debug, Default, InputObject)]
pub struct InputOrder {
    pub code: i32,
    pub title: String,
    pub description: Option<String>,
}

pub struct OrderLoader {
    pub pool: sqlx::PgPool,
}

impl OrderLoader {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<i32> for OrderLoader {
    type Value = Order;
    type Error = FieldError;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        Ok(
            sqlx::query_as("SELECT * FROM oms.orders WHERE id = ANY($1)")
                .bind(keys)
                .fetch(&self.pool)
                .map_ok(|ord: Order| (ord.id, ord))
                .try_collect()
                .await?
        )
    }
}