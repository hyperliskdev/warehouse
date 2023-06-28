


pub type HRMSSchema = Schema<HRMSQuery, HRMSMutation, EmptySubscription>;

pub struct HRMSQuery;


#[Object]
impl HRMSQuery {

    #[graphql(entity)]
    async fn resolve_customer(&self, ctx: &Context<'_>, id: i32) -> Result<Customer, FieldError> {
            
            let loader = ctx.data_unchecked::<DataLoader<CustomerLoader>>();
    
            let customer = loader.load_one(id).await?;
    
            if let Some(customer) = customer {
                Ok(customer)
            } else {
                Err("Customer not found".into())
            }
    
    }

    async fn customer(&self, ctx: &Context<'_>, id: i32) -> Result<Customer, FieldError> {
            
            let loader = ctx.data_unchecked::<DataLoader<CustomerLoader>>();
    
            let customer = loader.load_one(id).await?;
    
            if let Some(customer) = customer {
                Ok(customer)
            } else {
                Err("Customer not found".into())
            }
    
    }


    
}