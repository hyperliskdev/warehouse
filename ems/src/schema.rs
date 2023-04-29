use async_graphql::{Context, EmptySubscription, Schema, Object, Result};
use sqlx::{Pool, Postgres};

use crate::objects::employee::{Employee, InputEmployee};

pub type EMSSchema = Schema<EMSQuery, EMSMutation, EmptySubscription>;


#[derive(Default)]
pub struct EMSQuery;

#[Object]
impl EMSQuery {
    pub async fn employee(&self, ctx: &Context<'_>, id: i32) -> Result<Employee> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let employee = sqlx::query_as!(Employee, "SELECT * FROM ems.employees WHERE id = $1", id)
            .fetch_one(pool)
            .await?;

        Ok(employee)
        
    }

    #[graphql(entity)]
    pub async fn resolve_employee(&self, ctx: &Context<'_>, id: i32) -> Result<Employee> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        let employee = sqlx::query_as!(Employee, "SELECT * FROM ems.employees WHERE id = $1", id)
            .fetch_one(pool)
            .await?;

        Ok(employee)
    }
    
}


#[derive(Default)]
pub struct EMSMutation;

#[Object]
impl EMSMutation {

    pub async fn create_employee(&self, ctx: &Context<'_>, new_employee: InputEmployee) -> Result<String> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();

        // Create the new location entry
        let employee = sqlx::query_as!(
            Employee,
            "INSERT INTO ems.employees (first_name, last_name, password) VALUES ($1, $2, $3) RETURNING *",
            new_employee.first_name,
            new_employee.last_name,
            new_employee.password,
        )
        .fetch_one(pool)
        .await?;

        Ok(employee.employee_code.to_string())
    }

}