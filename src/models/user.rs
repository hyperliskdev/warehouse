// User model in dynamodb


use std::{collections::HashMap, error::Error};

use rusoto_dynamodb::{AttributeValue};
use serde::{Deserialize, Serialize};
use thiserror::Error;

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
}

// User Errors
#[derive(Debug, Error)]
pub enum UserError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("User not found")]
    UserNotFound,
}