use std::sync::Arc;

use actix_files::Files;
use actix_session::{config::{PersistentSession, SessionLifecycle}, storage::CookieSessionStore, SessionMiddleware};
use actix_web::{body::MessageBody, cookie::Key, dev::{ServiceFactory, ServiceRequest, ServiceResponse}, get, http::StatusCode, web::Data, App, Error, HttpResponse, Responder};
use authfix::{middleware::{AuthMiddleware, PathMatcher}, session::{handlers::SessionLoginHandler, session_auth::{session_login_factory, SessionAuthProvider}}};
use serde::Serialize;

use crate::{controller::activity_controller, domain::user_api::UserApi, service::{auth_service::AuthenticationService, user_service::UserService}};

pub fn create_test_session_middleware (key: Key) -> SessionMiddleware<CookieSessionStore> {
    let persistent_session = PersistentSession::default();
    let lc = SessionLifecycle::PersistentSession(persistent_session);
    SessionMiddleware::builder(CookieSessionStore::default(), key)
                .cookie_name("sessionId".to_string())
                .cookie_http_only(true)
                .cookie_same_site(actix_web::cookie::SameSite::None)
                .cookie_secure(false)
                .session_lifecycle(lc)
                .build()    
}

#[derive(Serialize)]
struct TestResponse {
    pub test: i32,
    pub title: String,
}

#[get("/api/test")]
async fn test_endpoint() -> impl Responder {
    HttpResponse::Ok().json(TestResponse { test: 42, title: "MyActivities".to_owned() })
}

pub fn create_app(cookie_key: Key) -> App<
impl ServiceFactory<
    ServiceRequest,
    Response = ServiceResponse<impl MessageBody>,
    Config = (),
    InitError = (),
    Error = Error,
>> {
    let user_service= Arc::new(UserService::new());
    let user_api: Arc<dyn UserApi> = Arc::clone(&user_service) as Arc<dyn UserApi>;
    let user_api_data = Data::from(user_api);

    session_login_factory(
        SessionLoginHandler::new(AuthenticationService::new(Arc::clone(&user_service))), 
        AuthMiddleware::new(SessionAuthProvider, PathMatcher::new(vec!["/api/login/*","/api/test", "/web/index.html"], true)), 
        create_test_session_middleware(cookie_key)
    )
    .service(Files::new("/web", "./static"))
    .service(test_endpoint)
    .app_data(user_api_data.clone())
    .configure(activity_controller::config)        
}