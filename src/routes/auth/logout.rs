use serde::{Deserialize, Serialize};
use warp::Filter;

#[derive(Debug, Deserialize)]
pub struct LogoutRequest {}

#[derive(Debug, Serialize)]
pub struct LogoutResponse {
    message: String,
}

pub fn logout() -> impl warp::Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("logout")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(logout_handler)
}

async fn logout_handler(logout_req: LogoutRequest) -> Result<impl warp::Reply, warp::Rejection> {
    let response = LogoutResponse {
        message: "Successfully logged out".to_string(),
    };

    let reply = warp::reply::json(&response);

    // Delete the token from users session.
    Ok(warp::reply::with_header(reply, "warehouse-token", ""))
}
