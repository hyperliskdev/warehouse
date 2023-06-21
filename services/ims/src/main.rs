mod schema;
mod input;

use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema, dataloader::DataLoader};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use schema::IMSSchema;
use sqlx::postgres::PgPoolOptions;


async fn index(schema: web::Data<IMSSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:password@localhost/test").await?;

    


    let schema = Schema::build(IMSQuery, IMSMutation, EmptySubscription)
        .data(pool)
        .data(
            DataLoader::new(
                OrderLoader::new(pool.clone()),
                async_std::task::spawn, // or `tokio::spawn`
        ))
        .finish();

    println!("GraphiQL IDE: http://localhost:8003");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind("127.0.0.1:8002")?
    .run()
    .await
}