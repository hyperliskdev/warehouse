use std::collections::HashMap;

use async_graphql::{Object, Context, FieldError, futures_util::TryStreamExt, async_trait, dataloader::DataLoader};
use sqlx::{Postgres, Pool, FromRow};


#[derive(Clone, Debug, Default, FromRow)]
pub struct PieceCategory {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[Object]
impl PieceCategory {
    async fn id(&self, ctx: &Context<'_>) -> Result<i32, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<PieceCategoryLoader>>();
        let p = loader.load_one(self.id).await?;

        if let Some(p) = p {
            Ok(p.id)
        } else {
            Err(FieldError::new("PieceCategory not found"))
        }
    }
    
    async fn title(&self, ctx: &Context<'_>) -> Result<String, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<PieceCategoryLoader>>();
        let p = loader.load_one(self.id).await?;

        if let Some(p) = p {
            Ok(p.title)
        } else {
            Err(FieldError::new("PieceCategory not found"))
        }
    }

    async fn description(&self, ctx: &Context<'_>) -> Result<String, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<PieceCategoryLoader>>();
        let p = loader.load_one(self.id).await?;

        if let Some(p) = p {
            if let Some(desc) = p.description {
                Ok(desc)
            } else {
                Err(FieldError::new("This piece has no description"))
            }
        } else {
            Err(FieldError::new("PieceCategory not found"))
        }
    }

    async fn created_at(&self, ctx: &Context<'_>) -> Result<chrono::NaiveDateTime, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<PieceCategoryLoader>>();
        let p = loader.load_one(self.id).await?;

        if let Some(p) = p {
            Ok(p.created_at)
        } else {
            Err(FieldError::new("PieceCategory not found"))
        }
    }

    async fn updated_at(&self, ctx: &Context<'_>) -> Result<chrono::NaiveDateTime, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<PieceCategoryLoader>>();
        let p = loader.load_one(self.id).await?;

        if let Some(p) = p {
            Ok(p.updated_at)
        } else {
            Err(FieldError::new("PieceCategory not found"))
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct InputPieceCategory {
    pub title: String,
    pub description: String,
}

pub struct PieceCategoryLoader {
    pool: Pool<Postgres>,
}

impl PieceCategoryLoader {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl async_graphql::dataloader::Loader<i32> for PieceCategoryLoader {
    type Value = PieceCategory;
    type Error = FieldError;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        Ok(
            sqlx::query_as("SELECT * FROM ims.piece_category WHERE id = ANY($1)")
                .bind(keys)
                .fetch(&self.pool)
                .map_ok(|p: PieceCategory| (p.id, p))
                .try_collect()
                .await?
        )
    }
}