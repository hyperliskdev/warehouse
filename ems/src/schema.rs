


pub type EMSSchema = Schema<EMSQuery, EMSMutation, EmptySubscription>;


#[derive(Default)]
pub struct EMSQuery;

#[Object]
impl EMSQuery {
    pub async fn get_employee(&self, ctx: &Context<'_>, id: i32) -> Result<Employee> {
        let loader = ctx.data_unchecked::<DataLoader<EmployeeLoader>>();
        let employee = loader.load_one(id).await?;

        Ok(employee.unwrap())
    }

    #[graphql(entity)]
    pub async fn employee(&self, ctx: &Context<'_>, id: i32) -> Result<Employee> {
        let loader = ctx.data_unchecked::<DataLoader<EmployeeLoader>>();
        let employee = loader.load_one(id).await?;

        Ok(employee.unwrap())
    }
    
}