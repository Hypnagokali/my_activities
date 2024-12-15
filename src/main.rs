use std::sync::Arc;

use actix_session::{storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, dev::ServiceFactory, web::{self, Data}, App, HttpServer};
use application::authentication::Auth;
use config::config::Config;
use controller::{activity_controller, authentication_controller};
use domain::{auth_api::AuthenticationApi, user_api::UserApi};
use service::{auth_service::TestAuthenticationService, user_service::TestUserService};

mod config;
mod controller;
mod service;
mod domain;
mod error;
mod application;


pub fn config_main_app(cfg: &mut web::ServiceConfig) {
    let user_api: Arc<dyn UserApi> = Arc::new(TestUserService::new());
    let auth_api: Arc<dyn AuthenticationApi> = Arc::new(TestAuthenticationService::new());

    let user_api_data = Data::from(user_api);
    let auth_api_data = Data::from(auth_api);

    cfg.configure(activity_controller::config)
        .configure(authentication_controller::config)
        .app_data(user_api_data.clone())
        .app_data(auth_api_data.clone());
}


pub fn create_session_middleware () -> SessionMiddleware<CookieSessionStore> {
    let encrypt_key_for_cookies = Key::generate();

    SessionMiddleware::builder(CookieSessionStore::default(), encrypt_key_for_cookies.clone())
                .cookie_name("sessionId".to_string())
                .cookie_http_only(true)
                .cookie_same_site(actix_web::cookie::SameSite::None)
                .cookie_secure(true)
                .build()
            
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let config = Config::from_env();
    
    let server = HttpServer::new(move || {
        App::new()
        .configure(config_main_app)
        .wrap(Auth::new())
        .wrap(create_session_middleware())
        
    })
    .bind((config.host.clone(), config.port))?
    .run();

    println!("Server started on host: {} and port: {}", config.host, config.port);

    server.await
}
