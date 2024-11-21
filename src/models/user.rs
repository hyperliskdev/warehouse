// User model in dynamodb

use rusoto_dynamodb::{AttributeValue, DynamoDb, DynamoDbClient, GetItemInput, PutItemInput};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    pub password: String,
    pub created_at: String,
    pub updated_at: String,
}

impl User {
    pub async fn get_item(db_client: DynamoDbClient, id: &str) -> Result<Option<User>, rusoto_core::RusotoError<rusoto_dynamodb::GetItemError>> {
        let get_item = GetItemInput {
            key: hashmap! {
                "id".to_string() => AttributeValue {
                    s: Some(id.to_string()),
                    ..Default::default()
                }
            },
            table_name: "users".to_string(),
            ..Default::default()
        };

        let result = db_client.get_item(get_item).await?;
        let item = result.item;

        match item {
            Some(item) => {
                let user = User {
                    id: item.get("id").unwrap().s.as_ref().unwrap().to_string(),
                    username: item.get("username").unwrap().s.as_ref().unwrap().to_string(),
                    password: item.get("password").unwrap().s.as_ref().unwrap().to_string(),
                    created_at: item.get("created_at").unwrap().s.as_ref().unwrap().to_string(),
                    updated_at: item.get("updated_at").unwrap().s.as_ref().unwrap().to_string(),
                };
                Ok(Some(user))
            }
            None => Ok(None),
        }
    }

    pub async fn put_item(&self, db_client: DynamoDbClient) -> Result<(), rusoto_core::RusotoError<rusoto_dynamodb::PutItemError>> {
        let put_item = PutItemInput {
            item: hashmap! {
                "id".to_string() => AttributeValue {
                    s: Some(self.id.clone()),
                    ..Default::default()
                },
                "username".to_string() => AttributeValue {
                    s: Some(self.username.clone()),
                    ..Default::default()
                },
                "password".to_string() => AttributeValue {
                    s: Some(self.password.clone()),
                    ..Default::default()
                },
                "created_at".to_string() => AttributeValue {
                    s: Some(self.created_at.clone()),
                    ..Default::default()
                },
                "updated_at".to_string() => AttributeValue {
                    s: Some(self.updated_at.clone()),
                    ..Default::default()
                },
            },
            table_name: "users".to_string(),
            ..Default::default()
        };

        db_client.put_item(put_item).await?;
        Ok(())
    }
}