use std::{fmt::Debug, ops::Deref};

use actix_session::{Session, SessionExt};
use serde::de::DeserializeOwned;

use crate::GetAuthenticatedUserFromRequest;


#[derive(Clone)]
pub struct GetUserFromSession;

impl<U> GetAuthenticatedUserFromRequest<U> for GetUserFromSession
where
    U: DeserializeOwned
{
    fn get_authenticated_user(&self, req: &actix_web::HttpRequest) -> Result<U, ()> {
        let s: Session = req.get_session();
        let ds = DebuggableSession(s);
        println!("FromRequest -> Session: {:?}", ds);

        if let Ok(Some(user)) = ds.get::<U>("user") {
            return Ok(user)
        } else {
            Err(())
        }
    }
}

pub struct DebuggableSession(pub Session);

impl Deref for DebuggableSession {
    type Target = Session;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for DebuggableSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let entries = self.0.entries();
        let keys = entries.keys();

        let mut debug = f.debug_tuple("Session");
        for key in keys {
            match self.0.get::<String>(key) {
                Ok(Some(s)) => {
                    debug.field(&format!("{} => {}", key, s));
                },
                Ok(None) => {},
                Err(_) => {},
            }
        };

        debug.finish()
    }
}
