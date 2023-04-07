use async_graphql::{Object, Context};
use sqlx::{Postgres, Pool};



pub struct PieceCategory {
    id: i32,
    title: String,
    desc: String,
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
    
    async fn title(&self) -> &str {
        &self.title
    }

    async fn desc(&self) -> &str {
        &self.desc
    }
}