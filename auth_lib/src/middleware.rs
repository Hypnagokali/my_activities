use std::{future::{ready, Ready}, marker::PhantomData};

use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, web::Data, Error, HttpMessage};
use futures::future::LocalBoxFuture;
use serde::de::DeserializeOwned;

use crate::GetAuthenticatedUserFromRequest;

pub struct AuthMiddleware<GetUserTrait, U> 
where 
    GetUserTrait: GetAuthenticatedUserFromRequest<U>,
    U: DeserializeOwned
{
    get_user_trait: GetUserTrait,
    user_type: PhantomData<U>
}

impl<GetUserTrait, U> AuthMiddleware<GetUserTrait, U> 
where 
    GetUserTrait: GetAuthenticatedUserFromRequest<U>,
    U: DeserializeOwned
{
    pub fn new(get_user_trait: GetUserTrait) -> Self {
        AuthMiddleware {
            get_user_trait,
            user_type: PhantomData,
        }
    }
}

pub struct AuthMiddlewareInner<S, GetUserTrait, U>
where 
    GetUserTrait: GetAuthenticatedUserFromRequest<U>,
    U: DeserializeOwned
{
    service: S,
    get_user_trait: GetUserTrait,
    user_type: PhantomData<U>
}

impl<S, B, GetUserTrait, U> Service<ServiceRequest> for AuthMiddlewareInner<S, GetUserTrait, U>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
    U: DeserializeOwned,
    GetUserTrait: GetAuthenticatedUserFromRequest<U>,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        println!("Session authentication {}", req.path());
        match self.get_user_trait.get_authenticated_user(&req.request()) {
            Ok(_) => println!("User found in get_user_trait"),
            Err(_) => println!("User not found. Error. No problem when its not an authenticated route"),
        }
        // ToDo:
        // 1. create AuthToken
        // 2. add AuthToken to extensions

        // let mut ext = req.extensions_mut();
        // ext.insert();
        
        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
    
}


impl<S, B, GetUserTrait, U> Transform<S, ServiceRequest> for AuthMiddleware<GetUserTrait, U>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
    GetUserTrait: GetAuthenticatedUserFromRequest<U> + Clone,
    U: DeserializeOwned
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddlewareInner<S, GetUserTrait, U>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        println!("Init AuthSessionMiddleware...");
        ready(Ok(AuthMiddlewareInner { 
            service,
            get_user_trait: self.get_user_trait.clone(), // couldnt we handle this differently?
            user_type: PhantomData, 
        }))
    }
}