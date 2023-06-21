use async_graphql::*;


pub type IMSSchema = Schema<IMSQuery, IMSMutation, EmptySubscription>;

struct IMSQuery;

#[Object]
impl IMSQuery {

    #[graphql(entity)]
    async fn resolve_order(&self, ctx: &Context<'_>, id: String) -> Result<Order> {

        todo!()

    }
    
}


struct IMSMutation;

#[Object]
impl IMSMutation {

    async fn create_order(&self, input: OrderInput) -> Result<Order> {

        todo!()

    }
        
}