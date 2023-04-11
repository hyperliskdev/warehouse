use std::collections::HashMap;

use async_graphql::{Context, Object, async_trait, FieldError, dataloader::{Loader, DataLoader}, futures_util::TryStreamExt, InputObject};
use sqlx::{Postgres, Pool};




#[derive(Clone, Debug, Default, sqlx::FromRow)]
pub struct LocationEntry {
    pub id: i32,
    pub location_id: i32,
    pub piece_id: i32,
    pub quantity: i32,
    pub unit: i32,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[Object]
impl LocationEntry {

    async fn id(&self, ctx: &Context<'_>) -> Result<i32, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<LocationEntryLoader>>();
        let loc_entry = loader.load_one(self.id).await?;

        if let Some(loc_entry) = loc_entry {
            Ok(loc_entry.id)
        } else {
            Err(FieldError::new("LocationEntry not found"))
        }
    }

    async fn location_id(&self, ctx: &Context<'_>) -> Result<i32, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<LocationEntryLoader>>();
        let loc_entry = loader.load_one(self.id).await?;

        if let Some(loc_entry) = loc_entry {
            Ok(loc_entry.location_id)
        } else {
            Err(FieldError::new("LocationEntry not found"))
        }
    }

    async fn piece_id(&self, ctx: &Context<'_>) -> Result<i32, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<LocationEntryLoader>>();
        let loc_entry = loader.load_one(self.id).await?;

        if let Some(loc_entry) = loc_entry {
            Ok(loc_entry.piece_id)
        } else {
            Err(FieldError::new("LocationEntry not found"))
        }
    }

    async fn quantity(&self, ctx: &Context<'_>) -> Result<i32, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<LocationEntryLoader>>();
        let loc_entry = loader.load_one(self.id).await?;

        if let Some(loc_entry) = loc_entry {
            Ok(loc_entry.quantity)
        } else {
            Err(FieldError::new("LocationEntry not found"))
        }
    }

}

#[derive(Clone, Debug, Default, InputObject)]
pub struct InputLocationEntry {
    pub location_id: i32,
    pub piece_id: i32,
    pub quantity: i32,
    pub unit: i32,
}

pub struct LocationEntryLoader {
    pool: Pool<Postgres>,
}

impl LocationEntryLoader {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<i32> for LocationEntryLoader {
    type Value = LocationEntry;
    type Error = FieldError;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        Ok(
            sqlx::query_as("SELECT * FROM ims.location_entries WHERE id = ANY($1)")
                .bind(keys)
                .fetch(&self.pool)
                .map_ok(|loc_entry: LocationEntry| (loc_entry.id, loc_entry))
                .try_collect()
                .await?
        )
    }
}