mod objects;
mod schema;

use actix_web::{guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use async_graphql::{http::GraphiQLSource, EmptySubscription, MergedObject, Schema};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};
use objects::ObjectQuery;
use schema::IMSSchema;
use crate::schema::{IMSMutation, IMSQuery};

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
    let schema = Schema::build(IMSQuery, IMSMutation, EmptySubscription)
        .enable_federation()
        .data(
            sqlx::postgres::PgPoolOptions::new()
                .max_connections(5)
                .connect("postgres://postgres:hyperlisk@localhost:5432/warehouse")
                .await
                .unwrap(),
        )
        // .data(DataLoader::new(LocationLoader, async_std::task::spawn))
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
