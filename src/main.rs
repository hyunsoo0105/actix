use actix_web::{HttpServer, web, App};

mod state;

// #[actix_web::main]
// async fn main() -> std::io::Result<()> {
//     HttpServer::new(|| {
//         App::new()
//             .route("/", web::get().to(|| async { "hello actix_web" }))
//             .configure(state::state::state_config)
//     })
//     .bind(("127.0.0.1", 8080))?
//     .run()
//     .await
// }

#[actix_web::main]
async fn main() {
    let _run_worker_http_serverrr = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(|| async { "hello actix_web" }))
            .configure(state::state::state_config)
    })
    .workers(2)
    .bind("127.0.0.1:8080")
    .expect("Can not run server")
    .run().await;
}