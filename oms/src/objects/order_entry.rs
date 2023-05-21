use std::collections::HashMap;
use async_graphql::{InputObject, async_trait, dataloader::{Loader, DataLoader}, FieldError, Context, Object, futures_util::TryStreamExt};

#[derive(Clone, Debug, Default, sqlx::FromRow)]
pub struct OrderEntry {
    pub id: i32,
    pub order_id: i32,
    pub piece_id: i32,
    pub quantity: i32,
    pub unit: i32,
    pub line_price: f64,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[Object]
impl OrderEntry {

    async fn id(&self, ctx: &Context<'_>) -> Result<i32, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<OrderEntryLoader>>();
        let order_entry = loader.load_one(self.id).await?;

        if let Some(order_entry) = order_entry {
            Ok(order_entry.id)
        } else {
            Err(FieldError::new("OrderEntry not found"))
        }
    }

    async fn order_id(&self, ctx: &Context<'_>) -> Result<i32, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<OrderEntryLoader>>();
        let order_entry = loader.load_one(self.id).await?;

        if let Some(order_entry) = order_entry {
            Ok(order_entry.order_id)
        } else {
            Err(FieldError::new("OrderEntry not found"))
        }
    }

    async fn piece_id(&self, ctx: &Context<'_>) -> Result<i32, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<OrderEntryLoader>>();
        let order_entry = loader.load_one(self.id).await?;

        if let Some(order_entry) = order_entry {
            Ok(order_entry.piece_id)
        } else {
            Err(FieldError::new("OrderEntry not found"))
        }
    }

    async fn quantity(&self, ctx: &Context<'_>) -> Result<i32, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<OrderEntryLoader>>();
        let order_entry = loader.load_one(self.id).await?;

        if let Some(order_entry) = order_entry {
            Ok(order_entry.quantity)
        } else {
            Err(FieldError::new("OrderEntry not found"))
        }
    }

    async fn unit(&self, ctx: &Context<'_>) -> Result<i32, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<OrderEntryLoader>>();
        let order_entry = loader.load_one(self.id).await?;

        if let Some(order_entry) = order_entry {
            Ok(order_entry.unit)
        } else {
            Err(FieldError::new("OrderEntry not found"))
        }
    }

    async fn line_price(&self, ctx: &Context<'_>) -> Result<f64, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<OrderEntryLoader>>();
        let order_entry = loader.load_one(self.id).await?;

        if let Some(order_entry) = order_entry {
            Ok(order_entry.line_price)
        } else {
            Err(FieldError::new("OrderEntry not found"))
        }
    }

    async fn created_at(&self, ctx: &Context<'_>) -> Result<chrono::NaiveDateTime, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<OrderEntryLoader>>();
        let order_entry = loader.load_one(self.id).await?;

        if let Some(order_entry) = order_entry {
            Ok(order_entry.created_at)
        } else {
            Err(FieldError::new("OrderEntry not found"))
        }
    }

    async fn updated_at(&self, ctx: &Context<'_>) -> Result<chrono::NaiveDateTime, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<OrderEntryLoader>>();
        let order_entry = loader.load_one(self.id).await?;

        if let Some(order_entry) = order_entry {
            Ok(order_entry.updated_at)
        } else {
            Err(FieldError::new("OrderEntry not found"))
        }
    }
}


#[derive(Clone, Debug, Default, InputObject)]
pub struct InputOrderEntry {
    pub order_id: i32,
    pub piece_id: i32,
    pub quantity: i32,
}

pub struct OrderEntryLoader {
    pub pool: sqlx::PgPool,
}

impl OrderEntryLoader {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<i32> for OrderEntryLoader {
    type Value = OrderEntry;
    type Error = FieldError;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        Ok(
            sqlx::query_as("SELECT * FROM oms.order_entries WHERE id = ANY($1)")
                .bind(keys)
                .fetch(&self.pool)
                .map_ok(|ord_ent: OrderEntry| (ord_ent.id, ord_ent))
                .try_collect()
                .await?
        )
    }
}