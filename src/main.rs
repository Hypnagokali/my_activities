use actix_web::{web::{self}, App, HttpResponse, HttpServer};
use config::config::Config;

mod config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let config = Config::from_env();

    let server = HttpServer::new(|| {
        App::new().service(
            web::resource("/test")
            .route(web::get().to(|| HttpResponse::Ok()))
        )
    })
    .bind((config.host.clone(), config.port))?
    .run();

    println!("Server started on host: {} and port: {}", config.host, config.port);

    server.await
}
