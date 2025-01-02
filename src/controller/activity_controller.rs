use actix_web::{get, web::{Query, ServiceConfig}, HttpResponse, Responder};
use auth_middleware_for_actix_web::AuthToken;
use serde::Deserialize;

use crate::domain::user::User;

#[get("/activities")]
pub async fn activities(token: AuthToken<User>) -> impl Responder {
    HttpResponse::Ok().body(format!("my activities. Request from user: {}", token.get_authenticated_user().email))
}


pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(activities);
}