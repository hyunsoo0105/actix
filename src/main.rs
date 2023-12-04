use actix_web::{HttpServer, web, App};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| async { "hello actix_web" }))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}