use actix_web::{get, App, HttpServer};

/// Index Route
#[get("/")]
async fn index() -> &'static str {
    "Hello world!"
}

/// Start the HttpServer
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(index)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}