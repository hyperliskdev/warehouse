use aws_config::{meta::region::RegionProviderChain, SdkConfig};
use aws_sdk_dynamodb::types::{AttributeDefinition, KeySchemaElement, KeyType, ProvisionedThroughput, ScalarAttributeType};
use warp::{Error, Filter};

pub async fn get_shared_config() -> SdkConfig {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-2");
    let region = region_provider.region().await.unwrap();
    let shared_config = aws_config::from_env().region(region).load().await;

    return shared_config;
}

async fn create_table(client: &aws_sdk_dynamodb::Client) -> Result<(), aws_sdk_dynamodb::Error> {

    let attribute_definitions = vec![
        AttributeDefinition::builder()
            .attribute_name("creator")
            .attribute_type(ScalarAttributeType::S)
            .build().expect("Error creating attribute definition: creator attr"),
        AttributeDefinition::builder()
            .attribute_name("version")
            .attribute_type(ScalarAttributeType::N)
            .build().expect("Error creating attribute definition: version attr"),
        AttributeDefinition::builder()
            .attribute_name("model_name")
            .attribute_type(ScalarAttributeType::S)
            .build().expect("Error creating attribute definition: model_name attr"),
        AttributeDefinition::builder()
            .attribute_name("upload_date")
            .attribute_type(ScalarAttributeType::N)
            .build().expect("Error creating attribute definition: upload_date attr"),
        AttributeDefinition::builder()
            .attribute_name("file_size")
            .attribute_type(ScalarAttributeType::N)
            .build().expect("Error creating attribute definition: file_size attr"),
        AttributeDefinition::builder()
            .attribute_name("file_type")
            .attribute_type(ScalarAttributeType::S)
            .build().expect("Error creating attribute definition: file_type attr"),
        AttributeDefinition::builder()
            .attribute_name("tags")
            .attribute_type(ScalarAttributeType::S)
            .build().expect("Error creating attribute definition: tags attr"),
    ];

    let partition_key_schema = KeySchemaElement::builder()
        .attribute_name("creator")
        .key_type(KeyType::Hash)
        .build().expect("Error creating partition key schema");
        

    let sort_key_schema = KeySchemaElement::builder()
        .attribute_name("version")
        .key_type(KeyType::Range)
        .build().expect("Error creating sort key schema");

    let provisioned_throughput = ProvisionedThroughput::builder()
        .read_capacity_units(5)
        .write_capacity_units(5)
        .build().expect("Error creating provisioned throughput");


    let create_table_response = client.create_table()
        .table_name("models_metadata")
        .set_key_schema(vec![partition_key_schema, sort_key_schema].into())
        .set_attribute_definitions(Some(attribute_definitions))
        .provisioned_throughput(provisioned_throughput)
        .send().await?;

    Ok(())

    
}

#[tokio::main]
async fn main() -> Result<(), Error> {

    warp::serve(warp::get().map(|| warp::reply::html("Hello, World!")))
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}