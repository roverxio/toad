use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use futures::future::LocalBoxFuture;
use log::error;
use reqwest::header::HeaderName;
use std::future::{ready, Ready};

use crate::errors::ApiError;

pub struct ToadAuthMiddleware;

impl<S, B> Transform<S, ServiceRequest> for ToadAuthMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Transform = AuthMiddlewareService<S>;
    type InitError = ();
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddlewareService { service }))
    }
}

pub struct AuthMiddlewareService<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthMiddlewareService<S>
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
        let header = req.headers().get(HeaderName::from_static("user"));
        match header {
            None => {
                error!("Unauthorized request");
                Box::pin(async { Err(Error::from(ApiError::Unauthorized)) })
            }
            Some(user) => {
                req.extensions_mut()
                    .insert(user.to_str().unwrap().to_string());
                Box::pin(self.service.call(req))
            }
        }
    }
}
