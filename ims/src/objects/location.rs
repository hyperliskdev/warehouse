// Locations contain Pieces

use std::{sync::Arc, collections::HashMap};

use async_graphql::{Object,
Context, InputObject, async_trait, futures_util::TryStreamExt, dataloader::{Loader, DataLoader}, FieldError, parser::types::Field};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Clone, Debug, Default, sqlx::FromRow)]
pub struct Location {
    pub id: i32,
    pub code: Uuid,
    pub title: String,
    pub description: Option<String>,
    
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[Object]
impl Location {
    async fn id(&self, ctx: &Context<'_>) -> Result<i32, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<LocationLoader>>();
        let loc = loader.load_one(self.id).await?;

        if let Some(loc) = loc {
            Ok(loc.id)
        } else {
            Err(FieldError::new("Location not found"))
        }
    }

    async fn code(&self, ctx: &Context<'_>) -> Result<Uuid, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<LocationLoader>>();
        let loc = loader.load_one(self.id).await?;

        if let Some(loc) = loc {
            Ok(loc.code)
        } else {
            Err(FieldError::new("Location not found"))
        }
    }

    async fn title(&self, ctx: &Context<'_>) -> Result<String, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<LocationLoader>>();
        let loc = loader.load_one(self.id).await?;

        if let Some(loc) = loc {
            Ok(loc.title)
        } else {
            Err(FieldError::new("Location not found"))
        }
        
    }

    async fn description(&self, ctx: &Context<'_>) -> Result<Option<String>, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<LocationLoader>>();
        let loc = loader.load_one(self.id).await?;

        if let Some(loc) = loc {
            Ok(loc.description)
        } else {
            Err(FieldError::new("Location not found"))
        }
    }

    async fn created_at(&self, ctx: &Context<'_>) -> Result<chrono::NaiveDateTime, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<LocationLoader>>();
        let loc = loader.load_one(self.id).await?;

        if let Some(loc) = loc {
            Ok(loc.created_at)
        } else {
            Err(FieldError::new("Location not found"))
        }
    }

    async fn updated_at(&self, ctx: &Context<'_>) -> Result<chrono::NaiveDateTime, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<LocationLoader>>();
        let loc = loader.load_one(self.id).await?;

        if let Some(loc) = loc {
            Ok(loc.updated_at)
        } else {
            Err(FieldError::new("Location not found"))
        }
    }

}

#[derive(Clone, Debug, Default, InputObject)]
pub struct InputLocation {
    pub title: String,
    pub description: Option<String>,
}

pub struct LocationLoader {
    pool: sqlx::PgPool,
}

impl LocationLoader {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<i32> for LocationLoader {
    type Value = Location;
    type Error = FieldError;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {


        Ok(
            sqlx::query_as("SELECT * FROM ims.locations WHERE id = ANY($1)")
                .bind(keys)
                .fetch(&self.pool)
                .map_ok(|loc: Location| (loc.id, loc))
                .try_collect()
                .await?
        )
    }
}
