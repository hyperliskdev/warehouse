use async_graphql::{Context, Object};
use sqlx::{Postgres, Pool};




#[derive(Clone, Debug, Default)]
pub struct LocationEntry {
    id: i32,
    location_id: i32,
    piece_id: i32,
    quantity: i32,
}

#[Object]
impl LocationEntry {

    async fn id(&self, ctx: &Context<'_>) -> Result<i32, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (id,): (i32,) = sqlx::query_as("SELECT id FROM location_entries WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(id)
    }

    async fn location_id(&self, ctx: &Context<'_>) -> Result<i32, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (location_id,): (i32,) = sqlx::query_as("SELECT location_id FROM location_entries WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(location_id)
    }

    async fn piece_id(&self, ctx: &Context<'_>) -> Result<i32, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (piece_id,): (i32,) = sqlx::query_as("SELECT piece_id FROM location_entries WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(piece_id)
    }

    async fn quantity(&self, ctx: &Context<'_>) -> Result<i32, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let (quantity,): (i32,) = sqlx::query_as("SELECT quantity FROM location_entries WHERE id = $1")
            .bind(self.id)
            .fetch_one(pool)
            .await?;
        Ok(quantity)
    }

}