// Employee

use async_graphql::{Context, FieldError};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

#[derive(Clone, Debug, Default, sqlx::FromRow)]
pub struct Employee {
    pub id: i32,
    pub employee_code: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub password: String,

    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[Object]
impl Employee {
    async fn id(&self, ctx: &Context<'_>) -> Result<i32, FieldError> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let employee_id = sqlx::query!(
            r#"
                SELECT id FROM ems.employees WHERE id = $1
            "#,
            self.id
        ).fetch_one(pool)
        .await?;
        Ok(employee_id)
    }

    async fn employee_code(&self, ctx: &Context<'_>) -> Result<Uuid, FieldError> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let employee_code = sqlx::query!(
            r#"
                SELECT employee_code FROM ems.employees WHERE id = $1
            "#,
            self.id
        ).fetch_one(pool)
        .await?;

        Ok(employee_code)
    }

    async fn first_name(&self, ctx: &Context<'_>) -> Result<String, FieldError> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let first_name = sqlx::query!(
            r#"
                SELECT first_name FROM ems.employees WHERE id = $1
            "#,
            self.id
        ).fetch_one(pool)
        .await?;

        Ok(first_name)
    }

    async fn last_name(&self, ctx: &Context<'_>) -> Result<String, FieldError> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let last_name = sqlx::query!(
            r#"
                SELECT last_name FROM ems.employees WHERE id = $1
            "#,
            self.id
        ).fetch_one(pool)
        .await?;

        Ok(last_name)
    }

    async fn password(&self, ctx: &Context<'_>) -> Result<String, FieldError> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let password = sqlx::query!(
            r#"
                SELECT password FROM ems.employees WHERE id = $1
            "#,
            self.id
        ).fetch_one(pool)
        .await?;

        Ok(password)
    }

    async fn created_at(&self, ctx: &Context<'_>) -> Result<chrono::NaiveDateTime, FieldError> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let created_at = sqlx::query!(
            r#"
                SELECT created_at FROM ems.employees WHERE id = $1
            "#,
            self.id
        ).fetch_one(pool)
        .await?;

        Ok(created_at)
    }

    async fn updated_at(&self, ctx: &Context<'_>) -> Result<chrono::NaiveDateTime, FieldError> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let updated_at = sqlx::query!(
            r#"
                SELECT updated_at FROM ems.employees WHERE id = $1
            "#,
            self.id
        ).fetch_one(pool)
        .await?;

        Ok(updated_at)
    }
}