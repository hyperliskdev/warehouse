use rusoto_dynamodb::DynamoDbClient;
use serde::{Deserialize, Serialize};
use warp::Filter;

use crate::{
    handlers::user_handler::{create_user, get_user_by_email},
    models::user::{User, UserError},
    routes::with_db, utils::jwt::generate_token,
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
    let user = get_user_by_email(db_client.clone(), register_req.email.clone()).await;

    match user {
        Ok(_) => {
            // If the user exists, return an error
            return Err(warp::reject());
        }
        Err(e) => {
            // If the user is not found, create the user
            match e {
                UserError::UserNotFound => {
                    let mut user = User::new(
                        register_req.email,
                        register_req.username,
                        register_req.password
                    ).unwrap();

                    let token = generate_token(&user.id);

                    match create_user(db_client.clone(), &mut user).await {
                        Ok(_) => {
                            match token {
                                Ok(token) => {
                                    return Ok(
                                        warp::reply::with_header(warp::reply::json(
                                            &RegisterResponse {
                                                message: "User registered successfuly.".to_string(),
                                            }
                                        ), "warehouse-token", token)
                                    );
                                },
                                Err(_) => return Err(warp::reject()),
                            }
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
