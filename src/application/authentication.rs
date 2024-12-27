use std::{fmt::Debug, future::{ready, Ready}, ops::Deref};

use actix_session::{Session, SessionExt};
use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, Error};
use auth_lib::session::session_auth::DebuggableSession;
use futures::future::LocalBoxFuture;

use crate::domain::{auth_api::AuthToken, user::User};

pub struct AuthMiddleware;

impl AuthMiddleware {
    pub fn new() -> Self {
        AuthMiddleware {}
    }
}

pub struct AuthMiddlewareInner<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareInner<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Check authentication {}", req.path());
        let session = req.get_session();
        let ds = DebuggableSession(session);

        println!("{:?}", ds);
        
        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;

            println!("Process response");
            Ok(res)
        })
    }
    
}


impl<S, B> Transform<S, ServiceRequest> for AuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareInner<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        println!("new_transform (INIT process) called ...");
        ready(Ok(AuthMiddlewareInner { service }))
    }
}



