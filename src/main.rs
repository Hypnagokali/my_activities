use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, App, HttpServer};
use config::config::Config;
use controller::{activity_controller, authentication_controller};

mod config;
mod controller;
mod service;
mod domain;
mod error;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let config = Config::from_env();

    let key = Key::generate();

    let server = HttpServer::new(move || {
        App::new()
        .configure(activity_controller::config)
        .configure(authentication_controller::config)
        .wrap(SessionMiddleware::new(CookieSessionStore::default(), key.clone()))
    })
    .bind((config.host.clone(), config.port))?
    .run();

    println!("Server started on host: {} and port: {}", config.host, config.port);

    server.await
}
