use actix_web::{get, http::header::ContentType, web::ServiceConfig, HttpResponse, Responder};
use authfix::multifactor::google_auth::TotpSecretGenerator;

#[get("/qrcode")]
async fn get_qrcode() -> impl Responder {
    let generator = TotpSecretGenerator::new();
    let secret = generator.create_secret();

    let qrcode = TotpSecretGenerator::create_qr_code(&secret, "MyActivities", "test@example.org").unwrap(); 

    HttpResponse::Ok()
        .insert_header(ContentType(mime::IMAGE_SVG))
        .body(qrcode)
}


pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_qrcode);
}