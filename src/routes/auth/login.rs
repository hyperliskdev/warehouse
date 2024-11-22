// Login route


use std::collections::HashMap;

use rusoto_dynamodb::AttributeValue;
use rusoto_dynamodb::DynamoDb;
use rusoto_dynamodb::DynamoDbClient;
use rusoto_dynamodb::GetItemInput;
use rusoto_dynamodb::QueryInput;
use serde::Serialize;

use serde::Deserialize;
use warp::Filter;

use crate::handlers::user_handler::get_user;
use crate::models::user::User;
use crate::routes::with_db;
use crate::utils::jwt::generate_token;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    warehouse_token: String
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

    
    let user: User = match get_user(db_client, login_req.email).await {
        Ok(user) => user,
        Err(_) => return Err(warp::reject())
    };

    // Verify password with the hashed password.
    let password_match = user.verify_password(&login_req.password);

    if !password_match.unwrap() {
        return Err(warp::reject());
    } else {
        println!("Password match");
        let token = generate_token(&user.id);
        match token {
            Ok(token) => {
                return Ok(warp::reply::json(&LoginResponse {
                    warehouse_token: token
                }))
            },
            Err(_) => return Err(warp::reject())
        }
    }
}