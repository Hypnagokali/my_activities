use std::sync::Arc;

use actix_session::{config::{PersistentSession, SessionLifecycle}, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{cookie::Key, web::{self, Data}, HttpServer};
use argon2::{password_hash::{rand_core::OsRng, SaltString}, Argon2, PasswordHasher};
use config::config::Config;
use controller::activity_controller;
use domain::{auth_api::AuthenticationApi, user::User};
use rusqlite::Connection;
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

pub fn create_test_user() {
    let conn = Connection::open("activities_db.sqlite3").unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT, email TEXT UNIQUE);", []).unwrap();

    let credential_table = r#"
        CREATE TABLE IF NOT EXISTS credentials (
            id INTEGER PRIMARY KEY, 
            password TEXT,
            user_id INTEGER,
            FOREIGN KEY (user_id) REFERENCES users(id)
        );
    "#;

    conn.execute(credential_table, []).unwrap();

    // ToDo: Remove unwrap and encapsulate
    let test_user = conn
        .query_row("SELECT id, name, email FROM users WHERE email = 'test@example.org'", [], |row| {
            let name: String = row.get(1).unwrap();
            let email: String = row.get(2).unwrap();
            Ok(User::new(row.get(0).unwrap(), &email, &name))
        });
    
    match test_user {
        Ok(_) => {
            println!("Test user: already exists.")
        },
        Err(_) => {
            let insert_user = r#"
                INSERT INTO users (name, email) values('Hans', 'test@example.org');
            "#;

            conn.execute(insert_user, []).unwrap();
            let id = conn.last_insert_rowid();

            let salt = SaltString::generate(&mut OsRng);
            let hash = Argon2::default().hash_password("test123".as_bytes(), &salt).unwrap().to_string();

            let insert_credentials = r#"
                INSERT INTO credentials (password, user_id) values(?1, ?2)
            "#;

            conn.execute(insert_credentials, (&hash, id)).unwrap();

            println!("Test user with id = {} created.", id);
        },
    };


    
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let config = Config::from_env();
    let encrypt_key_for_cookies = Key::generate();

    create_test_user();

    let server = HttpServer::new(move || {
        app_factory::create_app(encrypt_key_for_cookies.clone())
    })
    .bind((config.host.clone(), config.port))?
    .run();

    println!("Server started on host: {} and port: {}", config.host, config.port);

    server.await
}
