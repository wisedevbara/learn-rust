//! Logging Middleware
//!
//! Request/response logging middleware.

#![allow(dead_code)]

use actix_web::{dev::ServiceResponse, Error, HttpMessage, HttpRequest};
use std::future::Future;
use std::pin::Pin;

/// Logging middleware for HTTP requests and responses
pub struct LoggingMiddleware;

impl LoggingMiddleware {
    /// Create a new logging middleware
    pub fn new() -> Self {
        Self
    }
}

impl Default for LoggingMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

impl<S, B> actix_web::dev::Transform<S, actix_web::dev::ServiceRequest>
    for LoggingMiddleware
where
    S: actix_web::dev::Service<
            actix_web::dev::ServiceRequest,
            Response = ServiceResponse<B>,
            Error = Error,
        > + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = LoggingMiddlewareService<S>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Transform, Self::InitError>>>>;

    fn new_transform(&self, service: S) -> Self::Future {
        Box::pin(async move { Ok(LoggingMiddlewareService { service }) })
    }
}

/// Logging middleware service
pub struct LoggingMiddlewareService<S> {
    service: S,
}

impl<S, B> actix_web::dev::Service<actix_web::dev::ServiceRequest>
    for LoggingMiddlewareService<S>
where
    S: actix_web::dev::Service<
            actix_web::dev::ServiceRequest,
            Response = ServiceResponse<B>,
            Error = Error,
        > + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: actix_web::dev::ServiceRequest) -> Self::Future {
        let method = req.method().clone();
        let uri = req.uri().clone();

        tracing::info!("Request: {} {}", method, uri);

        let future = self.service.call(req);

        Box::pin(async move {
            let result = future.await?;
            tracing::info!("Response status: {}", result.status());
            Ok(result)
        })
    }
}
