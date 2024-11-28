// Login route

use rusoto_dynamodb::DynamoDbClient;
use serde::Serialize;

use serde::Deserialize;
use warp::Filter;

use crate::handlers::user_handler::get_user_by_email;
use crate::models::user::User;
use crate::routes::with_db;
use crate::utils::jwt::generate_token;

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    message: String,
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
    let user: User = match get_user_by_email(db_client, login_req.email).await {
        Ok(user) => user,
        Err(_) => return Err(warp::reject()),
    };

    // Verify password with the hashed password.
    let password_match = user.verify_password(&login_req.password);

    if !password_match.unwrap() {
        return Err(warp::reject());
    } else {
        println!("Password match");
        let token = generate_token(&user.id);
        match token {
            // Set the authorization header with the token
            Ok(token) => {
                return Ok(warp::reply::with_header(
                    warp::reply::json(&LoginResponse {
                        message: "User logged in successfully.".to_string(),
                    }),
                    "warehouse-token",
                    token,
                ));
            }
            Err(_) => return Err(warp::reject()),
        }
    }
}
