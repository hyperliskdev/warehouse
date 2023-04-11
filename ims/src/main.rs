mod objects;
mod schema;

use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result, rt::spawn};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema, dataloader::DataLoader};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use schema::IMSSchema;
use crate::{schema::{IMSMutation, IMSQuery}, objects::{location::LocationLoader, piece_category::PieceCategoryLoader, piece::PieceLoader, location_entry::LocationEntryLoader}};

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

    let pg_pool = sqlx::postgres::PgPoolOptions::new()
    .max_connections(5)
    .connect("postgres://postgres:hyperlisk@localhost:5432/warehouse")
    .await
    .unwrap();

    let schema = Schema::build(IMSQuery, IMSMutation, EmptySubscription)
        .enable_federation()
        .data( pg_pool.clone()
        )
        .data(
            DataLoader::new(
                LocationLoader::new(pg_pool.clone()), spawn
            )
        )
        .data(
            DataLoader::new(
                PieceLoader::new(pg_pool.clone()), spawn
            )
        )
        .data(
            DataLoader::new(
                PieceCategoryLoader::new(pg_pool.clone()), spawn
            )
        )
        .data(
            DataLoader::new(
                LocationEntryLoader::new(pg_pool.clone()), spawn
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
