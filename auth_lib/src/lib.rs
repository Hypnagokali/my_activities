use core::fmt;
use std::future::{ready, Ready};

use actix_session::{Session, SessionExt};
use actix_web::{error::HttpError, Error, FromRequest, HttpRequest, HttpResponse, ResponseError};
use serde::de::DeserializeOwned;
use session::session_auth::DebuggableSession;

pub mod session;

pub trait GetAuthenticatedUserFromRequest<U> 
where 
    U: DeserializeOwned {
    fn get_authenticated_user(req: &HttpRequest) -> Result<U, ()>;
}

pub struct AuthSession (Session);


pub struct AuthToken<U> 
where
    U: DeserializeOwned
{
    user: U,
}

impl<U> AuthToken<U>
where
    U: DeserializeOwned
{

    fn new(user: U) -> Self {
        Self {
            user,
        }
    }

    pub fn get_authenticated_user(self) -> U {
        self.user
    }
}


impl<U> FromRequest for AuthToken<U> 
where
    U: DeserializeOwned
{
    type Error = Error;
    type Future = Ready<Result<AuthToken<U>, Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut actix_web::dev::Payload) -> Self::Future {
        let s: Session = req.get_session();
        let ds = DebuggableSession(s);
        println!("FromRequest -> Session: {:?}", ds);

        if let Ok(Some(user)) = ds.get::<U>("user") {
            return ready(Ok(AuthToken::new(user)))
        } 

        ready(Err(NotAuthenticatedError { message: "Error: No authentciated user".to_owned() }.into()))
    }
}

#[derive(Debug)]
pub struct NotAuthenticatedError {
    pub message: String,
}

impl fmt::Display for NotAuthenticatedError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Debug unauth error")
    }
}


impl ResponseError for NotAuthenticatedError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        actix_web::http::StatusCode::UNAUTHORIZED
    }

    fn error_response(&self) -> HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::Unauthorized().json(self.message.clone())
    }
}

// impl SessionAuthToken {
//     pub fn new(session: Session) -> Self {
//         SessionAuthToken {
//             session,
//         }
//     }
// }

// impl<U> AuthToken<U> for SessionAuthToken 
// where 
//     U: DeserializeOwned {
//     fn get_authenticated_user(&self) -> Result<U, ()> {
//         match self.session.get::<U>("user") {
//             Ok(Some(user)) => Ok(user),
//             _ => Err(()),
//         }
//     }
// }
