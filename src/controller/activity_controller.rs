use actix_session::Session;
use actix_web::{get, web};

#[get("/activities")]
pub async fn activities(session: Session) -> String {
    

    "Users activities".to_owned()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(activities);
}