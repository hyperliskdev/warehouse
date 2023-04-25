use std::collections::HashMap;

use async_graphql::{dataloader::{Loader, DataLoader}, async_trait, FieldError, Context, Object, InputObject, futures_util::TryStreamExt};


#[derive(Clone, Debug, Default, sqlx::FromRow)]
pub struct Unit {
    pub id: i32,
    pub title: String,
    pub short: String,
    pub description: Option<String>,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[Object]
impl Unit {
    async fn id(&self, ctx: &Context<'_>) -> Result<i32, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<UnitLoader>>();
        let u = loader.load_one(self.id).await?;

        if let Some(u) = u {
            Ok(u.id)
        } else {
            Err(FieldError::new("Unit not found"))
        }
    }

    async fn title(&self, ctx: &Context<'_>) -> Result<String, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<UnitLoader>>();
        let u = loader.load_one(self.id).await?;

        if let Some(u) = u {
            Ok(u.title)
        } else {
            Err(FieldError::new("Unit not found"))
        }
    }

    async fn short(&self, ctx: &Context<'_>) -> Result<String, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<UnitLoader>>();
        let u = loader.load_one(self.id).await?;

        if let Some(u) = u {
            Ok(u.short)
        } else {
            Err(FieldError::new("Unit not found"))
        }
    }

    async fn description(&self, ctx: &Context<'_>) -> Result<String, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<UnitLoader>>();
        let u = loader.load_one(self.id).await?;

        if let Some(u) = u {
            if let Some(desc) = u.description {
                Ok(desc)
            } else {
                Err(FieldError::new("Unit not found"))
            }
        } else {
            Err(FieldError::new("Unit not found"))
        }
    }
}


#[derive(Clone, Debug, Default, InputObject)]
pub struct InputUnit {
    pub title: String,
    pub short: String,
    pub description: Option<String>,
}

pub struct UnitLoader {
    pool: sqlx::PgPool,
}

impl UnitLoader {
    pub fn new(pool: sqlx::PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Loader<i32> for UnitLoader {
    type Value = Unit;
    type Error = FieldError;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        Ok(
            sqlx::query_as("SELECT * FROM ims.units WHERE id = ANY($1)")
                .bind(keys)
                .fetch(&self.pool)
                .map_ok(|u: Unit| (u.id, u))
                .try_collect()
                .await?
        )
    }
}