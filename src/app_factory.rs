use std::sync::Arc;

use actix_files::Files;
use authfix::actix_session::{config::{PersistentSession, SessionLifecycle}, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{body::MessageBody, cookie::Key, dev::{ServiceFactory, ServiceRequest, ServiceResponse}, get, web::{self, Data}, App, Error, HttpResponse, Responder};
use authfix::{config::Routes, session::app_builder::SessionLoginAppBuilder};
use serde::Serialize;

use crate::{config::db::DbConfig, controller::{activity_controller, qrcode_controller, root_controller}, domain::user_api::UserApi, service::{auth_service::AuthenticationService, user_service::UserService}};

pub fn create_test_session_middleware (key: Key) -> SessionMiddleware<CookieSessionStore> {
    let persistent_session = PersistentSession::default();
    let lc = SessionLifecycle::PersistentSession(persistent_session);
    SessionMiddleware::builder(CookieSessionStore::default(), key)
                .cookie_name("sessionId".to_string())
                .cookie_http_only(true)
                .cookie_same_site(actix_web::cookie::SameSite::Lax)
                .cookie_secure(false)
                .session_lifecycle(lc)
                .build()    
}

#[derive(Serialize)]
struct TestResponse {
    pub test: i32,
    pub title: String,
}

#[get("/test")]
async fn test_endpoint() -> impl Responder {
    HttpResponse::Ok().json(TestResponse { test: 42, title: "MyActivities".to_owned() })
}

pub fn create_app(cookie_key: Key, db_config: DbConfig) -> App<
impl ServiceFactory<
    ServiceRequest,
    Response = ServiceResponse<impl MessageBody>,
    Config = (),
    InitError = (),
    Error = Error,
>> {
    
    let user_service= Arc::new(UserService::new(Arc::new(db_config)));
    let user_api: Arc<dyn UserApi> = Arc::clone(&user_service) as Arc<dyn UserApi>;
    let user_api_data = Data::from(user_api);

    let routes = Routes::new("/api", "/login", "/login/mfa", "/logout");
    let login_handler = AuthenticationService::new(Arc::clone(&user_service));
    
    SessionLoginAppBuilder::create(login_handler, cookie_key.clone())
        .set_login_routes_and_unsecured_paths(routes, vec!["/api/test", "/web/index.html"])
        .set_session_middleware(create_test_session_middleware(cookie_key))
        .build()
    .service(
        web::scope("/api")
            .service(test_endpoint)
            .configure(activity_controller::config)
            .configure(root_controller::config)
            .configure(qrcode_controller::config)
    )
    .service(Files::new("/web", "./static"))
    .app_data(user_api_data.clone())
}