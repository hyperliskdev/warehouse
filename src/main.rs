use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::{Client, Error};


// pub async fn upload_object(
//     client: &aws_sdk_s3::Client,
//     bucket_name: &str,
//     file_name: &str,
//     key: &str,
// ) -> Result<aws_sdk_s3::operation::put_object::PutObjectOutput, S3ExampleError> {
//     let body = aws_sdk_s3::primitives::ByteStream::from_path(std::path::Path::new(file_name)).await;
//     client
//         .put_object()
//         .bucket(bucket_name)
//         .key(key)
//         .body(body.unwrap())
//         .send()
//         .await
//         .map_err(S3ExampleError::from)
// }

/// Lists your DynamoDB tables in the default Region or us-east-1 if a default Region isn't set.
#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    let client = Client::new(&config);

    




    Ok(())
}