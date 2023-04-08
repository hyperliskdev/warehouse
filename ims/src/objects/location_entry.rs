use async_graphql::{Context, Object};
use sqlx::{Postgres, Pool};




#[derive(Clone, Debug, Default)]
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

    async fn id(&self, ctx: &Context<'_>) -> Result<i32, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (id,): (i32,) = sqlx::query_as("SELECT id FROM ims.location_entries WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(id)
    }

    async fn location_id(&self, ctx: &Context<'_>) -> Result<i32, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (location_id,): (i32,) = sqlx::query_as("SELECT location_id FROM ims.location_entries WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(location_id)
    }

    async fn piece_id(&self, ctx: &Context<'_>) -> Result<i32, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (piece_id,): (i32,) = sqlx::query_as("SELECT piece_id FROM ims.location_entries WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(piece_id)
    }

    async fn quantity(&self, ctx: &Context<'_>) -> Result<i32, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (quantity,): (i32,) = sqlx::query_as("SELECT quantity FROM ims.location_entries WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(quantity)
    }

}