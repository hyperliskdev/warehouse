// Locations contain Pieces

use std::{sync::Arc, collections::HashMap};

use async_graphql::{Object, Context};
use sqlx::{PgPool, Pool, Postgres};
use uuid::Uuid;

#[derive(Clone, Debug, Default)]
pub struct Location {
    id: i32,
    location_code: Uuid,
    title: String,
}

#[Object]
impl Location {
    async fn id(&self, ctx: &Context<'_>) -> Result<i32, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (id,): (i32,) = sqlx::query_as("SELECT id FROM locations WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(id)
    }

    async fn location_code(&self, ctx: &Context<'_>) -> Result<Uuid, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (location_code,): (Uuid,) = sqlx::query_as("SELECT location_code FROM locations WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(location_code)
    }

    async fn title(&self, ctx: &Context<'_>) -> Result<String, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (title,): (String,) = sqlx::query_as("SELECT title FROM locations WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(title)
    }
}



