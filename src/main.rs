use std::sync::Arc;

use actix_session::{config::{PersistentSession, SessionLifecycle}, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, web::{self, Data}, HttpServer};
use config::config::Config;
use controller::activity_controller;
use domain::auth_api::AuthenticationApi;
use service::{auth_service::AuthenticationService, user_service::UserService};

mod config;
mod controller;
mod service;
mod domain;
mod error;
mod app_factory;


pub fn config_main_app(cfg: &mut web::ServiceConfig) {
    let user_service = Arc::new(UserService::new());
    let user_api = Arc::clone(&user_service);
    let auth_api: Arc<dyn AuthenticationApi> = Arc::new(AuthenticationService::new(Arc::clone(&user_service)));
    
    let user_api_data = Data::from(user_api);
    let auth_api_data = Data::from(auth_api);

    cfg.configure(activity_controller::config)
        .app_data(user_api_data.clone())
        .app_data(auth_api_data.clone());
}

pub fn create_session_middleware (key: Key) -> SessionMiddleware<CookieSessionStore> {
    let persistent_session = PersistentSession::default();
    let lc = SessionLifecycle::PersistentSession(persistent_session);
    SessionMiddleware::builder(CookieSessionStore::default(), key)
                .cookie_name("sessionId".to_string())
                .cookie_http_only(true)
                .cookie_same_site(actix_web::cookie::SameSite::Lax) 
                .cookie_secure(true)
                .session_lifecycle(lc)
                .build()    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let config = Config::from_env();
    let encrypt_key_for_cookies = Key::generate();

    let server = HttpServer::new(move || {
        app_factory::create_app(encrypt_key_for_cookies.clone())
    })
    .bind((config.host.clone(), config.port))?
    .run();

    println!("Server started on host: {} and port: {}", config.host, config.port);

    server.await
}
