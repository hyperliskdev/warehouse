use async_graphql::{dataloader::DataLoader, Context, Schema, EmptySubscription, Object, Result};
use sqlx::{Pool, Postgres};

use crate::objects::{order::{OrderLoader, Order, InputOrder}, order_entry::{OrderEntry, OrderEntryLoader, InputOrderEntry}};

pub type OMSSchema = Schema<OMSQuery, OMSMutation, EmptySubscription>;


#[derive(Default)]
pub struct OMSQuery;

#[Object]
impl OMSQuery {

    pub async fn order(&self, ctx: &Context<'_>, id: i32) -> Result<Order> {
        let loader = ctx.data_unchecked::<DataLoader<OrderLoader>>();
        let order = loader.load_one(id).await?;

        Ok(order.unwrap())
    }


    #[graphql(entity)]
    pub async fn resolve_order(&self, ctx: &Context<'_>, id: i32) -> Result<Order> {
        let loader = ctx.data_unchecked::<DataLoader<OrderLoader>>();
        let order = loader.load_one(id).await?;

        Ok(order.unwrap())
    }

    pub async fn order_entry(&self, ctx: &Context<'_>, id: i32) -> Result<OrderEntry> {
        let loader = ctx.data_unchecked::<DataLoader<OrderEntryLoader>>();
        let order_entry = loader.load_one(id).await?;

        Ok(order_entry.unwrap())
    }

    #[graphql(entity)]
    pub async fn resolve_order_entry(&self, ctx: &Context<'_>, id: i32) -> Result<OrderEntry> {
        let loader = ctx.data_unchecked::<DataLoader<OrderEntryLoader>>();
        let line_item = loader.load_one(id).await?;

        Ok(line_item.unwrap())
    }
}

#[derive(Default)]
pub struct OMSMutation;

#[Object]
impl OMSMutation {

    pub async fn create_order(&self, ctx: &Context<'_>, new_order: InputOrder) -> Result<Order> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let loader = ctx.data_unchecked::<DataLoader<OrderLoader>>();
        
        let order = sqlx::query_as!(
            Order,
            "INSERT INTO oms.orders (title, description) VALUES ($1, $2) RETURNING *",
            new_order.title,
            new_order.description
        )
        .fetch_one(pool)
        .await?;

        loader.feed_one(order.id, order.clone()).await;

        Ok(order)
    }

    pub async fn create_order_entry(&self, ctx: &Context<'_>, new_order_entry: InputOrderEntry) -> Result<OrderEntry> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let loader = ctx.data_unchecked::<DataLoader<OrderEntryLoader>>();
        

        let entry = sqlx::query_as!(
            OrderEntry,
            "INSERT INTO oms.order_entries (order_id, piece_id, quantity) VALUES ($1, $2, $3) RETURNING *",
            new_order_entry.order_id,
            new_order_entry.piece_id,
            new_order_entry.quantity
        )
        .fetch_one(pool)
        .await?;

        loader.feed_one(entry.id, entry.clone()).await;

        Ok(entry)
    }
}