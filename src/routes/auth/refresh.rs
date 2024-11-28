use rusoto_dynamodb::DynamoDbClient;
use serde::{Deserialize, Serialize};
use warp::Filter;

use crate::{
    handlers::user_handler::get_user_by_id,
    models::user::User,
    routes::with_db,
    utils::jwt::{generate_token, validate_token},
};

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    body: String,
}

#[derive(Debug, Serialize)]
pub struct RefreshResponse {
    message: String,
}

pub fn refresh(
    db_client: DynamoDbClient,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("refresh")
        .and(warp::post())
        .and(warp::header::<String>("warehouse-token"))
        .and(with_db(db_client))
        .and_then(refresh_handler)
}

async fn refresh_handler(
    token: String,
    db_client: DynamoDbClient,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Before refreshing the token, we need to validate the token.
    // If the token is invalid, we will return an error.
    // If the token is valid, we will refresh the token.
    if token.is_empty() {
        return Err(warp::reject());
    }

    let validation = validate_token(&token);

    match validation {
        Ok(validation) => {
            if validation.exp < chrono::Utc::now().timestamp() as usize {
                return Err(warp::reject());
            }

            let user: User = match get_user_by_id(db_client, validation.sub).await {
                Ok(user) => user,
                Err(_) => return Err(warp::reject()),
            };
            // Check the UserID from the claims.

            // Generate a new token
            let new_token = generate_token(&user.id);

            match new_token {
                Ok(new_token) => {
                    return Ok(warp::reply::with_header(
                        warp::reply::json(&RefreshResponse {
                            message: "Refreshed token".to_string(),
                        }),
                        "warehouse-token",
                        new_token,
                    ));
                }
                Err(_) => return Err(warp::reject()),
            }
        }
        Err(_) => return Err(warp::reject()),
    }
}
