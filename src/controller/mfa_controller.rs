use authfix::{actix_session::Session, multifactor::authenticator::Authenticator, session::handlers::MfaRequestBody};
use actix_web::{error, get, http::header::ContentType, post, web::{Data, Json, ServiceConfig}, HttpResponse, Responder, Result};
use authfix::{multifactor::authenticator::{TotpSecretGenerator, MFA_ID_AUTHENTICATOR_TOTP}, AuthToken};

use crate::domain::{user::{MfaConfig, User}, user_api::UserApi};

const SESSION_KEY_TOTP_SECRET: &str = "totp_secret";

#[get("/totp/debug-user-data")]
async fn get_user_data(token: AuthToken<User>, user_api: Data<dyn UserApi>) -> impl Responder {
    let creds = user_api.find_credentials_by_user_id(token.get_authenticated_user().id).await.unwrap();

    let mfa = creds.mfa_config.unwrap();
    let r = format!(r#"{{
        "user": "{}",
        "mfa_id": "{}",
        "secret": "{}"
    }}"#, token.get_authenticated_user().name, mfa.mfa_id, mfa.secret.unwrap());

    HttpResponse::Ok().json(r)
}


#[get("/totp/qrcode")]
async fn get_qrcode(token: AuthToken<User>, session: Session) -> Result<impl Responder> {
    let email = &token.get_authenticated_user().email;

    let generator = TotpSecretGenerator::new("MyActivities", email);
    let secret = generator.get_secret();

    session.insert(SESSION_KEY_TOTP_SECRET, secret)?;

    let qrcode = generator.get_qr_code().unwrap();

    Ok(HttpResponse::Ok()
        .insert_header(ContentType(mime::IMAGE_SVG))
        .body(qrcode))
}

#[post("/totp/set-secret")]
async fn set_totp_secret(code: Json<MfaRequestBody>, token: AuthToken<User>, session: Session, user_api: Data<dyn UserApi>) 
    -> Result<impl Responder> 
{
    let user_id = token.get_authenticated_user().id;
    let mut creds = user_api.find_credentials_by_user_id(user_id).await
        .map_err(|err| {
            log::error!("Cannot load credentials: {}", err);
            error::ErrorBadRequest("Cannot save secret")
        })?;

    let secret = session.get::<String>(SESSION_KEY_TOTP_SECRET)?;

    if let Some(secret) = secret {
        // It seems to be a good practice to check a generated code before saving the secret
        if !Authenticator::verify(&secret, code.get_code(), 0) {
            return Err(error::ErrorUnauthorized("The TOTP was wrong"));
        }

        let mfa_config = MfaConfig::with_secret(MFA_ID_AUTHENTICATOR_TOTP, &secret);
        creds.set_mfa(mfa_config);
        user_api.save_credentials(creds).await
            .map_err(|err| {
                log::error!("Cannot save credentials after upating mfa_config: {}", err);
                error::ErrorBadRequest("Cannot save secret")
            })?;

        // clean up session
        session.remove(SESSION_KEY_TOTP_SECRET);
        Ok(HttpResponse::Ok())   
    } else {
        log::error!("Session does not contain the secret.");

        // clean up session
        session.remove(SESSION_KEY_TOTP_SECRET);
        Err(error::ErrorBadRequest("Cannot save secret"))
    }
}


pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(get_qrcode)
    .service(set_totp_secret)
    .service(get_user_data);
}