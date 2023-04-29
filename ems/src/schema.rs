use async_graphql::{Context, EmptySubscription, Schema, Object, Result};

use crate::objects::employee::Employee;




pub type EMSSchema = Schema<EMSQuery, EMSMutation, EmptySubscription>;


#[derive(Default)]
pub struct EMSQuery;

#[Object]
impl EMSQuery {
    pub async fn get_employee(&self, ctx: &Context<'_>, id: i32) -> Result<Employee> {

        todo!()
    }

    #[graphql(entity)]
    pub async fn resolve_employee(&self, ctx: &Context<'_>, id: i32) -> Result<Employee> {
        todo!()
    }
    
}


#[derive(Default)]
pub struct EMSMutation;

#[Object]
impl EMSMutation {

    pub async fn create_employee(&self, ctx: &Context<'_>, new_employee: InputEmployee) -> Result<Employee> {
        todo!()
    }

}