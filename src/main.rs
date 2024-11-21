pub mod routes;
pub mod utils;
pub mod models;
pub mod handlers;

use rusoto_core::Region;
use rusoto_dynamodb::DynamoDbClient;
use rusoto_s3::S3Client;
use warp::{Error, Filter};



#[tokio::main]
async fn main() -> Result<(), Error> {
    let db_client = DynamoDbClient::new(Region::default());
    let s3_client = S3Client::new(Region::default());


    let api = routes::api(db_client, s3_client)
        .with(warp::log("api"));

    warp::serve(api).run(([127, 0, 0, 1], 3030)).await;


    Ok(())
}