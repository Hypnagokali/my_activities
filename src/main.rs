use std::sync::Arc;

use authfix::actix_session::{config::{PersistentSession, SessionLifecycle}, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, middleware::Logger, HttpServer};
use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2, PasswordHasher};
use config::{config::Config, db::DbConfig};
use domain::{user::User, user_api::UserApi};
use rusqlite::Connection;
use service::user_service::UserService;

mod config;
mod controller;
mod service;
mod domain;
mod error;
mod app_factory;

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

pub fn create_db(db_config: &DbConfig) -> Connection {
    let conn = Connection::open(db_config.get_database()).unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT, email TEXT UNIQUE);", []).unwrap();

    let credential_table = r#"
        CREATE TABLE IF NOT EXISTS credentials (
            id INTEGER PRIMARY KEY, 
            password TEXT,
            mfa_id TEXT,
            mfa_secret TEXT,
            user_id INTEGER UNIQUE,
            FOREIGN KEY (user_id) REFERENCES users(id)
        );
    "#;

    conn.execute(credential_table, []).unwrap();

    conn
}

pub async fn create_test_user(db_config: DbConfig) {
    let user_service= UserService::new(Arc::new(db_config));


    match user_service.find_by_email("test@example.org").await {
        Ok(_) => {
            println!("Test user `Hans` already created");
        },
        Err(_) => {
            // assuming it was a not found error
            let user = User::new(0, "test@example.org".to_owned(), "Hans".to_owned());

            let salt = SaltString::generate(&mut OsRng);
            let password = Argon2::default().hash_password("test123".as_bytes(), &salt).expect("Hash test password not working");

            let user = user_service.save_user_with_credentials(user, &password.to_string()).await.expect("Cannot save test user");
            println!("Test user with id = {} created.", user.id);
        },
    }    

    match user_service.find_by_email("linda@example.org").await {
        Ok(_) => {
            println!("Test user `Linda` already created");
        },
        Err(_) => {
            // assuming it was a not found error
            let user = User::new(0, "linda@example.org".to_owned(), "Linda".to_owned());

            let salt = SaltString::generate(&mut OsRng);
            let password = Argon2::default().hash_password("linda123".as_bytes(), &salt).expect("Hash test password not working");

            let user = user_service.save_user_with_credentials(user, &password.to_string()).await.expect("Cannot save test user");
            println!("Test user with id = {} created.", user.id);
        },
    }    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let config = Config::from_env();

    let db_config = DbConfig::new("activities_db.sqlite3");
    create_db(&db_config);
    create_test_user(db_config).await;

    let encrypt_key_for_cookies = Key::generate();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    let server = HttpServer::new(move || {
        app_factory::create_app(encrypt_key_for_cookies.clone(), DbConfig::new("activities_db.sqlite3"))
        .wrap(Logger::default())
    })
    .bind((config.host.clone(), config.port))?
    .run();

    println!("Server started on host: {} and port: {}", config.host, config.port);

    server.await
}
