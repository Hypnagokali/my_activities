use actix_session::Session;
use actix_web::{post, web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
struct FormLogin {
    email: String,
    password: String,
}

#[post("/login")]
async fn login(login_form: web::Form<FormLogin>, session: Session) -> impl Responder {
    println!("login request: email={}, password={}", login_form.email, login_form.password);

    // create dummy session
    session.insert("testsession", "someuniquevalue").expect("Cant create Session :(");
    HttpResponse::Ok()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(login);
}