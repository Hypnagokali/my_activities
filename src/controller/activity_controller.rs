use actix_web::{get, web};

#[get("/activities")]
pub async fn activities() -> String {
    "ToDo".to_owned()
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(activities);
}