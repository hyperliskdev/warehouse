// Piece's go inside Locations

use std::collections::HashMap;

use async_graphql::{Object, Context, InputObject, futures_util::TryStreamExt, async_trait, FieldError, dataloader::DataLoader};
use uuid::Uuid;

#[derive(Clone, Debug, Default, sqlx::FromRow)]
pub struct Piece {
    pub id: i32,
    pub code: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub category: Option<i32>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}


#[Object]
impl Piece {
    async fn id(&self, ctx: &Context<'_>) -> Result<i32, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<PieceLoader>>();
        let p = loader.load_one(self.id).await?;

        

        if let Some(p) = p {
            Ok(p.id)
        } else {
            Err(FieldError::new("Piece not found"))
        }
    }

    async fn code(&self, ctx: &Context<'_>) -> Result<Uuid, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<PieceLoader>>();

        let p = loader.load_one(self.id).await?;

        if let Some(p) = p {
            Ok(p.code)
        } else {
            Err(FieldError::new("Piece not found"))
        }
    }

    async fn name(&self, ctx: &Context<'_>) -> Result<String, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<PieceLoader>>();
        let p = loader.load_one(self.id).await?;

        if let Some(p) = p {
            Ok(p.name)
        } else {
            Err(FieldError::new("Piece not found"))
        }
    }

    async fn description(&self, ctx: &Context<'_>) -> Result<String, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<PieceLoader>>();
        let p = loader.load_one(self.id).await?;

        if let Some(p) = p {
            if let Some(desc) = p.description {
                Ok(desc)
            } else {
                Ok("This piece has no description".to_string())
            }
        } else {
            Err(FieldError::new("Piece not found"))
        }
    }

    async fn category(&self, ctx: &Context<'_>) -> Result<i32, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<PieceLoader>>();
        let p = loader.load_one(self.id).await?;

        if let Some(p) = p {
            if let Some(category) = p.category {
                Ok(category)
            } else {
                Err(FieldError::new("This piece has no category"))
            }
        } else {
            Err(FieldError::new("Piece not found"))
        }
    }

    async fn created_at(&self, ctx: &Context<'_>) -> Result<chrono::NaiveDateTime, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<PieceLoader>>();
        let p = loader.load_one(self.id).await?;

        if let Some(p) = p {
            Ok(p.created_at)
        } else {
            Err(FieldError::new("Piece not found"))
        }
    }

    async fn updated_at(&self, ctx: &Context<'_>) -> Result<chrono::NaiveDateTime, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<PieceLoader>>();
        let p = loader.load_one(self.id).await?;

        if let Some(p) = p {
            Ok(p.updated_at)
        } else {
            Err(FieldError::new("Piece not found"))
        }
    }
}


#[derive(Clone, Debug, Default, InputObject)]
pub struct InputPiece {
    pub name: String,
    pub description: Option<String>,
    pub category: Option<i32>,
}

pub struct PieceLoader {
    pool: sqlx::PgPool,
}

impl PieceLoader {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<i32> for PieceLoader {
    type Value = Piece;
    type Error = FieldError;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        Ok(
            sqlx::query_as("SELECT * FROM ims.pieces WHERE id = ANY($1)")
                .bind(keys)
                .fetch(&self.pool)
                .map_ok(|p: Piece| (p.id, p))
                .try_collect()
                .await?
        )
    }
}