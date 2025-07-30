use actix_web::{get, web::ServiceConfig, HttpResponse, Responder};
use authfix::AuthToken;

use crate::domain::user::User;

#[get("/current-user")]
pub async fn get_authenticated_user(auth_token: AuthToken<User>) -> impl Responder {
    HttpResponse::Ok().json(&*auth_token.authenticated_user())
}


pub fn config(config: &mut ServiceConfig) {
    config.service(get_authenticated_user);
}