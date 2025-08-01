use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};
use authfix::AuthToken;

use crate::domain::user::User;

#[get("/activities")]
pub async fn activities(token: AuthToken<User>) -> impl Responder {
    HttpResponse::Ok().body(format!("my activities. Request from user: {}", token.authenticated_user().email))
}

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(activities);
}