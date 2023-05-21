use async_graphql::{dataloader::DataLoader, Context, Schema, EmptySubscription};

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

    pub async fn line_item(&self, ctx: &Context<'_>, id: i32) -> Result<LineItem> {
        let loader = ctx.data_unchecked::<DataLoader<LineItemLoader>>();
        let line_item = loader.load_one(id).await?;

        Ok(line_item.unwrap())
    }

    #[graphql(entity)]
    pub async fn resolve_line_item(&self, ctx: &Context<'_>, id: i32) -> Result<LineItem> {
        let loader = ctx.data_unchecked::<DataLoader<LineItemLoader>>();
        let line_item = loader.load_one(id).await?;

        Ok(line_item.unwrap())
    }
}

#[derive(Default)]
pub struct OMSMutation;

#[Object]
impl OMSMutation {

    pub async fn create_order(&self, ctx: &Context<'_>, input: NewOrder) -> Result<Order> {
        let loader = ctx.data_unchecked::<DataLoader<OrderLoader>>();
        let order = loader.create_one(input).await?;

        Ok(order.unwrap())
    }

    pub async fn create_line_item(&self, ctx: &Context<'_>, input: NewLineItem) -> Result<LineItem> {
        let loader = ctx.data_unchecked::<DataLoader<LineItemLoader>>();
        let line_item = loader.create_one(input).await?;

        Ok(line_item.unwrap())
    }
}