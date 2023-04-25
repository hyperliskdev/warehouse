use std::sync::Arc;

use async_graphql::{Object, dataloader::DataLoader, Context};
use sqlx::{Pool, Postgres};

use crate::objects::location_entry::{LocationEntry, LocationEntryLoader};

#[derive(Clone, Debug, Default)]
pub struct Inventory {
    pub entries: Vec<LocationEntry>,
}

#[Object]
impl Inventory {
    
    async fn entries(&self, ctx: &Context<'_>) -> Result<Vec<LocationEntry>, sqlx::Error> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let entries = sqlx::query_as!(
            LocationEntry, 
            "SELECT * FROM ims.location_entries"
        ).fetch_all(pool).await?;

        Ok(entries)
    }
}





