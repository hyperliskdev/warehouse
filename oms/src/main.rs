mod objects;
mod schema;

use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result, rt::spawn};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema, dataloader::DataLoader};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use schema::OMSSchema;
use crate::{schema::{OMSMutation, OMSQuery}, objects::{order::OrderLoader, order_entry::OrderEntryLoader}};

async fn index(schema: web::Data<OMSSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}


async fn index_graphiql() -> Result<HttpResponse> {
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(GraphiQLSource::build().endpoint("/").finish()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    let pg_pool = sqlx::postgres::PgPoolOptions::new()
    .max_connections(5)
    .connect("postgres://postgres:hyperlisk@localhost:5432/warehouse")
    .await
    .unwrap();

    let schema = Schema::build(OMSQuery, OMSMutation, EmptySubscription)
        .enable_federation()
        .data(pg_pool.clone())
        .data(
            DataLoader::new(
                OrderLoader::new(pg_pool.clone()), spawn
            )
        )
        .data(
            DataLoader::new(
                OrderEntryLoader::new(pg_pool.clone()), spawn
            )
        )
        .finish();

    println!("GraphiQL IDE: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}
