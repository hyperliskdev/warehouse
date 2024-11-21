use rusoto_core::RusotoError;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, PutItemError, PutItemInput};

use crate::models::user::{User, UserError};

// Create a new user
pub async fn create_user(
    db_client: DynamoDbClient,
    user: User,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut item = user.to_item();

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
