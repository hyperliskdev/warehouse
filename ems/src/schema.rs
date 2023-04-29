


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
    pub async fn resolve_employee(&self, ctx: &Context<'_>, id: i32) -> Result<Employee> {
        let loader = ctx.data_unchecked::<DataLoader<EmployeeLoader>>();
        let employee = loader.load_one(id).await?;

        Ok(employee.unwrap())
    }
    
}


#[derive(Default)]
pub struct EMSMutation;

#[Object]
impl EMSMutation {

    pub async fn create_employee(&self, ctx: &Context<'_>, new_employee: InputEmployee) -> Result<Employee> {
        
    }

}