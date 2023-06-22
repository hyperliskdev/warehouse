mod schema;
mod input;

use std::env;

use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema, dataloader::DataLoader};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use schema::IMSSchema;
use sqlx::postgres::PgPoolOptions;

use models::order::OrderLoader;

use crate::schema::{IMSQuery, IMSMutation};

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

    let addr = env::var("SERVER_ADDR").unwrap();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&env::var("DATABASE_URL").unwrap()).await.unwrap();




    let schema = Schema::build(IMSQuery, IMSMutation, EmptySubscription)
        .data(pool.clone())
        .enable_federation()
        .data(
            DataLoader::new(
                OrderLoader::new(pool.clone()),
                async_std::task::spawn,
        ))
        .finish();

    println!("GraphiQL IDE: http://localhost:8002");


    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(schema.clone()))
            .service(web::resource("/").guard(guard::Post()).to(index))
            .service(web::resource("/").guard(guard::Get()).to(index_graphiql))
    })
    .bind(addr)?
    .run()
    .await
}