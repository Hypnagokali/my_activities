use actix_web::{get, http::header::ContentType, web::ServiceConfig, HttpResponse, Responder};
use authfix::multifactor::google_auth::TotpSecretGenerator;

#[get("/qrcode")]
async fn get_qrcode() -> impl Responder {
    let generator = TotpSecretGenerator::new("MyActivities", "test@example.org");
    
    let _secret = generator.get_secret();

    let qrcode = generator.get_qr_code().unwrap();

    HttpResponse::Ok()
        .insert_header(ContentType(mime::IMAGE_SVG))
        .body(qrcode)
}


pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_qrcode);
}