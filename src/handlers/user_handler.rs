use std::collections::HashMap;

use rusoto_core::RusotoError;
use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput, PutItemError, PutItemInput};

use crate::models::user::{User, UserError};

// Create a new user
pub async fn create_user(
    db_client: DynamoDbClient,
     user: &mut User,
) -> Result<(), Box<dyn std::error::Error>> {

    // Hash the password before trying anything.
    user.hash_password()?;

    // Create a put_item input
    let item = user.to_item();

    let input = PutItemInput {
        item: item,
        table_name: "users".to_string(),
        condition_expression: Some(
            "attribute_not_exists(email) AND attribute_not_exists(username)".to_string(),
        ),
        ..Default::default()
    };

    //
    match db_client.put_item(input).await {
        Ok(_) => Ok(()),
        Err(e) => {
            // Check if the error is a conditional check failed exception
            // Use match statement for PutItemError
            match e {
                RusotoError::Service(PutItemError::ConditionalCheckFailed(_)) => {
                    Err(Box::new(UserError::UserAlreadyExists))
                }
                _ => Err(Box::new(e)),
            }
        }
    }
}

pub async fn get_user(
    db_client: DynamoDbClient,
    email: String,
) -> Result<User, UserError> {
    // Create a get_item input
    let input = GetItemInput {
        key: {
            let mut key = HashMap::new();
            key.insert(
                "email".to_string(),
                AttributeValue {
                    s: Some(email),
                    ..Default::default()
                },
            );
            key
        },
        table_name: "users".to_string(),
        ..Default::default()
    };

    // Get the item from the database
    match db_client.get_item(input).await {
        Ok(output) => {
            // Check if the item exists
            match output.item {
                Some(item) => {
                    // Convert the item to a user
                    let user = User::from_item(item);
                    Ok(user)
                }
                None => Err(UserError::UserNotFound),
            }
        }
        Err(e) => Err(UserError::DynamoDBError(e.to_string())),
    }
}
