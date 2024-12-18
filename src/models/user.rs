// User model in dynamodb


use core::hash;
use std::{collections::HashMap, error::Error};

use rusoto_dynamodb::{AttributeValue};
use serde::{Deserialize, Serialize};
use thiserror::Error;
use uuid::Uuid;

use crate::utils::hash::{hash_password, verify_password};

// UserErrors 
#[derive(Debug, Error)]
pub enum UserError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("User not found")]
    UserNotFound,
    #[error("DynamoDB Error: {0}")]
    DynamoDBError(String),
    #[error("Password hash error")]
    PasswordHashError,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default, )]
pub struct User {
    pub id: String,
    pub email: String,
    pub username: String,
    pub password: String,
    pub created_at: String,
    pub updated_at: String,
}

impl User {

    pub fn new(email: String, username: String, password: String) -> Result<User, UserError> {

        let password = hash_password(&password);

        match password {
            Ok(password) => return Ok(User {
                id: Uuid::new_v4().to_string(),
                email,
                username,
                password,
                created_at: chrono::Utc::now().to_string(),
                updated_at: chrono::Utc::now().to_string(),
            }),
            Err(_) => {
                return Err(UserError::PasswordHashError)
            },
        }
        
    }

    pub fn to_item(&self) -> HashMap<String, AttributeValue> {
        let mut item = HashMap::new();
        item.insert("id".to_string(), AttributeValue { s: Some(self.id.clone()), ..Default::default() });
        item.insert("email".to_string(), AttributeValue { s: Some(self.email.clone()), ..Default::default() });
        item.insert("username".to_string(), AttributeValue { s: Some(self.username.clone()), ..Default::default() });
        item.insert("password".to_string(), AttributeValue { s: Some(self.password.clone()), ..Default::default() });
        item.insert("created_at".to_string(), AttributeValue { s: Some(self.created_at.clone()), ..Default::default() });
        item.insert("updated_at".to_string(), AttributeValue { s: Some(self.updated_at.clone()), ..Default::default() });
        item
    }

    pub fn from_item(item: HashMap<String, AttributeValue>) -> User {
        User {
            id: item.get("id").unwrap().s.as_ref().unwrap().to_string(),
            email: item.get("email").unwrap().s.as_ref().unwrap().to_string(),
            username: item.get("username").unwrap().s.as_ref().unwrap().to_string(),
            password: item.get("password").unwrap().s.as_ref().unwrap().to_string(),
            created_at: item.get("created_at").unwrap().s.as_ref().unwrap().to_string(),
            updated_at: item.get("updated_at").unwrap().s.as_ref().unwrap().to_string(),
        }
    }

    pub fn hash_password(&mut self) -> Result<(), UserError> {

        let mut hashed_password = hash_password(&self.password);
        let password = match hashed_password {
            Ok(password) => password,
            Err(_) => return Err(UserError::PasswordHashError),
        };

        self.password = password;

        Ok(())
    }

    pub fn verify_password(&self, password: &str) -> Result<bool, Box<dyn std::error::Error>> {
        let is_verified = verify_password(password, &self.password)?;

        Ok(is_verified)
    }
}

