use std::{collections::HashMap, future::{ready, Ready}, marker::PhantomData};

use actix_web::{dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform}, web::Data, Error, HttpMessage};
use futures::future::LocalBoxFuture;
use regex::Regex;
use serde::de::DeserializeOwned;

use crate::GetAuthenticatedUserFromRequest;

/// PathMatcher is used to secure specific paths or to exclude paths from authenticatio
/// exclude: exclude the path_list from authentication if true, else handles path_list as included for authentication.
/// path_list: Vec of paths. The path_list may include wildcards like "/api/user/*"
pub struct PathMatcher {
    exclude: bool,
    path_regex_list: Vec<(&'static str, Regex)>
}

impl PathMatcher {
    pub fn new(path_list: Vec<&'static str>, exclude: bool) -> Self {
        let mut path_regex_list = Vec::new();
        for pattern in path_list.into_iter() {
            let valid_regex = pattern.replace('*', ".*");
            path_regex_list.push((pattern, Regex::new(&valid_regex).unwrap()));
        }
        Self {
            exclude,
            path_regex_list,
        }
    }

    pub fn matches(&self, path: &str) -> bool {
        self.path_regex_list.iter().any(|p| (p.1.is_match(path) && !self.exclude) || !p.1.is_match(path))
    }
}

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

        let p = req.request().path();
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

#[cfg(test)]
mod tests {
    use super::PathMatcher;

    #[test]
    fn path_matcher_should_match_wildcard() {
        let matcher = PathMatcher::new(vec!["/api/users/*", "/some-other/route"], false);

        assert!(matcher.matches("/api/users/231/edit"));
    }

    #[test]
    fn path_matcher_should_match_any_not_in_list_when_excluded() {
        let matcher = PathMatcher::new(vec!["/some-other/route"], true);

        assert!(matcher.matches("/api/users/231/edit"));
    }

}