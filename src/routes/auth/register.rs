use rusoto_dynamodb::DynamoDbClient;
use serde::{Deserialize, Serialize};
use warp::Filter;

use crate::{
    handlers::user_handler::{create_user, get_user},
    models::user::UserError,
    routes::with_db,
};

#[derive(Debug, Deserialize)]
struct RegisterRequest {
    email: String,
    username: String,
    password: String,
}

#[derive(Debug, Serialize)]
struct RegisterResponse {
    message: String,
}

pub fn register(
    db_client: DynamoDbClient,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("register")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_db(db_client))
        .and_then(register_handler)
}

async fn register_handler(
    register_req: RegisterRequest,
    db_client: DynamoDbClient,
) -> Result<impl warp::Reply, warp::Rejection> {
    
    // Check if the user already exists
    // If the user exists, return an error
    // If the user does not exist, create the user
    // Return a success message
    let user = get_user(db_client.clone(), register_req.email.clone()).await;

    match user {
        Ok(_) => {
            // If the user exists, return an error
            return Err(warp::reject());
        }
        Err(e) => {
            // If the user is not found, create the user
            match e {
                UserError::UserNotFound => {
                    let mut user = crate::models::user::User {
                        id: uuid::Uuid::new_v4().to_string(),
                        email: register_req.email.clone(),
                        username: register_req.username.clone(),
                        password: register_req.password.clone(),
                        created_at: chrono::Utc::now().to_string(),
                        updated_at: chrono::Utc::now().to_string(),
                    };

                    match create_user(db_client.clone(), &mut user).await {
                        Ok(_) => {
                            return Ok(warp::reply::json(&RegisterResponse {
                                message: "User created successfully".to_string(),
                            }));
                        }
                        Err(_) => {
                            return Err(warp::reject());
                        }
                    }
                }
                _ => {
                    return Err(warp::reject());
                }
            }

        }
    }
}
