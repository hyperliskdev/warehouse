use thiserror::Error;
use warp::Filter;

pub mod login;
pub mod register;
pub mod logout;
pub mod refresh;


// AuthError
#[derive(Debug, Error)]
pub enum AuthError {
    #[error("Invalid hash")]
    InvalidHash,
    #[error("Invalid password")]
    InvalidPassword,
    #[error("Hash error: {0}")]
    HashError(String),
}

pub fn auth(
    db_client: rusoto_dynamodb::DynamoDbClient,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {


    let login_route = login::login(db_client.clone());
    let register_route = register::register(db_client.clone());
    let logout_route = logout::logout();
    let refresh_route = refresh::refresh(db_client.clone());

    warp::path!("auth")
        .and(login_route.or(register_route).or(logout_route).or(refresh_route))
}