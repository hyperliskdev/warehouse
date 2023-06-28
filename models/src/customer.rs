use std::{sync::Arc, collections::HashMap};

use async_graphql::{Object, Context, dataloader::{DataLoader, Loader}, async_trait, futures_util::TryStreamExt, FieldError, ComplexObject};
use chrono::{DateTime, Utc};
use sqlx::{FromRow, Pool, Postgres};

<<<<<<< HEAD
use crate::order::Order;



=======
>>>>>>> 784820bc7545f71f7550e6b67e876b8ea483e37b
#[derive(Debug, FromRow, Clone)]
pub struct Customer {
    pub id: i32,
    pub name: String,
    pub email: String,
    pub phone: String,
    pub updated_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[Object]
impl Customer {

    pub async fn id(&self, ctx: &Context<'_>, id: i32) -> Result<i32, FieldError> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let loader = ctx.data_unchecked::<DataLoader<CustomerLoader>>();
        
        let customer = loader.load_one(id).await?;

        if let Some(customer) = customer {
            Ok(customer.id)
        } else {

            let customer: Customer = sqlx::query_as("SELECT * FROM orders WHERE id = $1")
                .bind(id)
                .fetch_one(pool)
                .await?;

            loader.feed_one(id, customer.clone()).await;

            Ok(customer.id)
        }
    }

    pub async fn name(&self, ctx: &Context<'_>, id: i32) -> Result<String, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<CustomerLoader>>();
        
        let customer = loader.load_one(id).await?;

        if let Some(customer) = customer {
            Ok(customer.name)
        } else {
            Err("Customer not found".into())
        }
    }

    pub async fn email(&self, ctx: &Context<'_>, id: i32) -> Result<String, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<CustomerLoader>>();
        
        let customer = loader.load_one(id).await?;

        if let Some(customer) = customer {
            Ok(customer.email)
        } else {
            Err("Customer not found".into())
        }
    }

    pub async fn phone(&self, ctx: &Context<'_>, id: i32) -> Result<String, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<CustomerLoader>>();
        
        let customer = loader.load_one(id).await?;

        if let Some(customer) = customer {
            Ok(customer.phone)
        } else {
            Err("Customer not found".into())
        }
    }

    pub async fn updated_at(&self, ctx: &Context<'_>, id: i32) -> Result<DateTime<Utc>, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<CustomerLoader>>();
        
        let customer = loader.load_one(id).await?;

        if let Some(customer) = customer {
            Ok(customer.updated_at)
        } else {
            Err("Customer not found".into())
        }
    }

    pub async fn created_at(&self, ctx: &Context<'_>, id: i32) -> Result<DateTime<Utc>, FieldError> {
        let loader = ctx.data_unchecked::<DataLoader<CustomerLoader>>();
        
        let customer = loader.load_one(id).await?;

        if let Some(customer) = customer {
            Ok(customer.created_at)
        } else {
            Err("Customer not found".into())
        }
    }
}

#[ComplexObject]
impl Order {
    pub async fn customer(&self, ctx:&Context<'_>, customer_id: i32) -> Result<Customer, FieldError> {
        let pool = ctx.data_unchecked::<Pool<Postgres>>();
        let loader = ctx.data_unchecked::<DataLoader<CustomerLoader>>();
        
        let customer = loader.load_one(customer_id).await?;

        if let Some(customer) = customer {
            Ok(customer)
        } else {

            let customer: Customer = sqlx::query_as("SELECT * FROM orders WHERE id = $1")
                .bind(customer_id)
                .fetch_one(pool)
                .await?;

            loader.feed_one(customer_id, customer.clone()).await;

            Ok(customer)
        }
    }
}

struct CustomerLoader {
    pool: sqlx::PgPool,
}

#[async_trait::async_trait]
impl Loader<i32> for CustomerLoader {
    type Value = Customer;
    type Error = Arc<sqlx::Error>;

    async fn load(&self, keys: &[i32]) -> Result<HashMap<i32, Self::Value>, Self::Error> {
        Ok(sqlx::query_as("SELECT * FROM customers WHERE id = ANY($1)")
            .bind(keys)
            .fetch(&self.pool)
            .map_ok(|cus: Customer| (cus.id, cus))
            .map_err(Arc::new)
            .try_collect::<HashMap<i32, Customer>>()
            .await?
        )
    }
}