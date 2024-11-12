use aws_config::{meta::region::RegionProviderChain, SdkConfig};
use warp::{Error, Filter};

pub async fn get_shared_config() -> SdkConfig {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-2");
    let region = region_provider.region().await.unwrap();
    let shared_config = aws_config::from_env().region(region).load().await;

    return shared_config;
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    warp::serve(warp::get().map(|| warp::reply::html("Hello, World!")))
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}