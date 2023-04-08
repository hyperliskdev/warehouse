// Locations contain Pieces

use std::{sync::Arc, collections::HashMap};

use async_graphql::{Object,
Context, InputObject, async_trait, futures_util::TryStreamExt, dataloader::Loader};
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
    async fn id(&self, ctx: &Context<'_>) -> Result<i32, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (id,): (i32,) = sqlx::query_as("SELECT id FROM ims.locations WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(id)
    }

    async fn code(&self, ctx: &Context<'_>) -> Result<Uuid, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (location_code,): (Uuid,) = sqlx::query_as("SELECT code FROM ims.locations WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(location_code)
    }

    async fn title(&self, ctx: &Context<'_>) -> Result<String, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (title,): (String,) = sqlx::query_as("SELECT title FROM ims.locations WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(title)
    }

    async fn description(&self, ctx: &Context<'_>) -> Result<Option<String>, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (description,): (Option<String>,) = sqlx::query_as("SELECT description FROM ims.locations WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(description)
    }

    async fn created_at(&self, ctx: &Context<'_>) -> Result<chrono::NaiveDateTime, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (created_at,): (chrono::NaiveDateTime,) = sqlx::query_as("SELECT created_at FROM ims.locations WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(created_at)
    }

    async fn updated_at(&self, ctx: &Context<'_>) -> Result<chrono::NaiveDateTime, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (updated_at,): (chrono::NaiveDateTime,) = sqlx::query_as("SELECT updated_at FROM ims.locations WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(updated_at)
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

#[async_trait::async_trait]
impl Loader<i32> for LocationLoader {
    type Value = Location;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        Ok(
            sqlx::query_as("SELECT * FROM locations WHERE id = ANY($1)")
                .bind(keys)
                .fetch(&self.pool)
                .map_ok(|loc: Location| (loc.id, loc))
                .try_collect()
                .await?
        )
    }
}
