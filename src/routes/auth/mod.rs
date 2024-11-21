pub mod login;
pub mod register;
pub mod logout;
pub mod refresh;


pub fn auth(
    db_client: rusoto_dynamodb::DynamoDbClient,
    s3_client: rusoto_s3::S3Client,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {

    let db_filter = warp::any().map(move || db_client.clone());
    let s3_filter = warp::any().map(move || s3_client.clone());

    let login_route = login::login(db_client.clone());
    let register_route = register::register(db_client.clone());
    let logout_route = logout::logout(db_client.clone());
    let refresh_route = refresh::refresh(db_client.clone());

    warp::path("auth")
        .and(login_route.or(register_route).or(logout_route).or(refresh_route))
        .with(warp::log("auth"))

}