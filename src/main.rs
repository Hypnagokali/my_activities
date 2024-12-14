use std::sync::Arc;

use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, web::Data, App, HttpServer};
use application::auth_middleware::Auth;
use config::config::Config;
use controller::{activity_controller, authentication_controller};
use domain::user_api::UserApi;
use service::user_service::TestUserService;

mod config;
mod controller;
mod service;
mod domain;
mod error;
mod application;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let config = Config::from_env();

    let user_api: Arc<dyn UserApi + Sync + Send> = Arc::new(TestUserService::new());
    let app_data = Data::from(user_api);

    let encrypt_key_for_cookies = Key::generate();
    let server = HttpServer::new(move || {
        App::new()
        .configure(activity_controller::config)
        .configure(authentication_controller::config)
        .app_data(app_data.clone())
        .wrap(Auth::new())
        .wrap(SessionMiddleware::new(CookieSessionStore::default(), encrypt_key_for_cookies.clone()))
    })
    .bind((config.host.clone(), config.port))?
    .run();

    println!("Server started on host: {} and port: {}", config.host, config.port);

    server.await
}
