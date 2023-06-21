use async_graphql::*;
use models::order::Order;

use crate::input::OrderInput;


pub type IMSSchema = Schema<IMSQuery, IMSMutation, EmptySubscription>;

pub struct IMSQuery;

#[Object]
impl IMSQuery {

    #[graphql(entity)]
    async fn resolve_order(&self, ctx: &Context<'_>, id: String) -> Result<Order> {

        todo!()

    }

    // #[graphql(entity)]
    // async fn resolve_product(&self, ctx: &Context<'_>, id: String) -> Result<Product> {

    //     todo!()

    // }

    // #[graphql(entity)]
    // async fn resolve_location(&self, ctx: &Context<'_>, id: String) -> Result<Location> {

    //     todo!()

    // }

    // #[graphql(entity)]
    // async fn resolve_location_entry(&self, ctx: &Context<'_>, id: String) -> Result<LocationEntry> {

    //     todo!()

    // }

    async fn order(&self, ctx: &Context<'_>, id: String) -> Result<Order> {

        todo!()

    }

    // async fn product(&self, ctx: &Context<'_>, id: String) -> Result<Product> {

    //     todo!()

    // }

    // async fn location(&self, ctx: &Context<'_>, id: String) -> Result<Location> {

    //     todo!()

    // }
}


pub struct IMSMutation;

#[Object]
impl IMSMutation {

    async fn create_order(&self, input: OrderInput) -> Result<Order> {

        todo!()

    }
        
}