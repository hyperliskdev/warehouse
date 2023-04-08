use async_graphql::{Object, Context};
use sqlx::{Postgres, Pool};



pub struct PieceCategory {
    pub id: i32,
    pub title: String,
    pub description: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
}

#[Object]
impl PieceCategory {
    async fn id(&self, ctx: &Context<'_>) -> Result<i32, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (id,): (i32,) = sqlx::query_as("SELECT id FROM piece_category WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(id)
    }
    
    async fn title(&self, ctx: &Context<'_>) -> Result<String, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (title,): (String,) = sqlx::query_as("SELECT title FROM piece_category WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(title)
    }

    async fn desc(&self, ctx: &Context<'_>) -> Result<String, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (desc,): (String,) = sqlx::query_as("SELECT desc FROM piece_category WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(desc)
    }
}