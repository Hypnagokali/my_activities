use actix_web::{App, HttpServer};
use config::config::Config;
use controller::activity_controller;

mod config;
mod controller;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let config = Config::from_env();

    let server = HttpServer::new(|| {
        App::new()
        .configure(activity_controller::config)
    })
    .bind((config.host.clone(), config.port))?
    .run();

    println!("Server started on host: {} and port: {}", config.host, config.port);

    server.await
}
