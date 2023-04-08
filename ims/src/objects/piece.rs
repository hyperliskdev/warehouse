// Piece's go inside Locations

use async_graphql::{Object, Context, InputObject};
use sqlx::{Postgres, Pool};
use uuid::Uuid;

#[derive(Clone, Debug, Default)]
pub struct Piece {
    pub id: i32,
    pub code: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub category: i32,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}


#[Object]
impl Piece {
    async fn id(&self, ctx: &Context<'_>) -> Result<i32, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (id,): (i32,) = sqlx::query_as("SELECT id FROM pieces WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(id)
    }

    async fn code(&self, ctx: &Context<'_>) -> Result<Uuid, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (piece_code,): (Uuid,) = sqlx::query_as("SELECT piece_code FROM pieces WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(piece_code)
    }

    async fn name(&self, ctx: &Context<'_>) -> Result<String, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (name,): (String,) = sqlx::query_as("SELECT name FROM pieces WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(name)
    }

    async fn description(&self, ctx: &Context<'_>) -> Result<String, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (description,): (String,) = sqlx::query_as("SELECT description FROM pieces WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(description)
    }

    async fn category(&self, ctx: &Context<'_>) -> Result<i32, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (category,): (i32,) = sqlx::query_as("SELECT category FROM pieces WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(category)
    }

    async fn created_at(&self, ctx: &Context<'_>) -> Result<chrono::NaiveDateTime, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (created_at,): (chrono::NaiveDateTime,) = sqlx::query_as("SELECT created_at FROM pieces WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(created_at)
    }

    async fn updated_at(&self, ctx: &Context<'_>) -> Result<chrono::NaiveDateTime, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (updated_at,): (chrono::NaiveDateTime,) = sqlx::query_as("SELECT updated_at FROM pieces WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(updated_at)
    }
}


#[derive(Clone, Debug, Default, InputObject)]
pub struct InputPiece {
    pub name: String,
    pub description: Option<String>,
    pub category: i32,
}