// Login route


use rusoto_dynamodb::DynamoDb;
use rusoto_dynamodb::DynamoDbClient;
use rusoto_dynamodb::GetItemInput;
use rusoto_dynamodb::QueryInput;
use serde::Serialize;

use serde::Deserialize;
use warp::Filter;

use crate::routes::with_db;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    token: String
}

pub fn login(
    db_client: DynamoDbClient,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("login")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db_client))
        .and_then(login_handler)
}

async fn login_handler(
    login_req: LoginRequest,
    db_client: DynamoDbClient,
) -> Result<impl warp::Reply, warp::Rejection> {

    let mut key =  HashMap::new();
    key.insert("email".to_string(), login_req.email.clone().into());

    let input = GetItemInput {
        table_name: "users".to_string(),
        key: key,
        ..Default::default()
    };

    
    match db_client.get_item(input).await {
        Ok(output) => {
            
            
        },
        Err(_) => {
            Ok(warp::reply::json(&"Internal server error"))
        }
    }

}