use actix_session::Session;
use actix_web::{get, web::{self, Data}, HttpResponse, Responder};

use crate::{application::authentication::SessionAuthToken, domain::auth_api::{self, AuthToken, AuthenticationApi}};

#[get("/activities")]
pub async fn activities(session: Session, auth_api: Data<Box<dyn AuthenticationApi>>) -> impl Responder {
    let token: Box<dyn AuthToken> = Box::new(SessionAuthToken::new(session));
    if !auth_api.is_authenticated(&*token) {
        return HttpResponse::Unauthorized().finish();
    }

    HttpResponse::Ok().body("my activities")
}


pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(activities);
}