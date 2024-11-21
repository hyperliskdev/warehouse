pub mod health;

pub mod auth;
pub mod model;
pub mod deploy;


use rusoto_dynamodb::DynamoDbClient;
use rusoto_s3::S3Client;
use warp::Filter;



pub fn api(
    db_client: DynamoDbClient,
    s3_client: S3Client,
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {

    let db_filter = warp::any().map(move || db_client.clone());
    let s3_filter = warp::any().map(move || s3_client.clone());

    // Health route defined by default.
    let health_route = warp::path!("health").and(warp::get()).and_then(health::health);

    warp::path("api").and(health_route)

}