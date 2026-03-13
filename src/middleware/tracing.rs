//! Tracing Middleware
//!
//! Distributed tracing middleware using tracing.

#![allow(dead_code)]

use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;
use actix_web::{dev::ServiceRequest, dev::ServiceResponse, Error};
use tracing::{info, instrument};

/// Tracing middleware for distributed tracing
pub struct TracingMiddleware;

impl TracingMiddleware {
    /// Create a new tracing middleware
    pub fn new() -> Self {
        Self
    }

    /// Extract trace ID from request
    fn extract_trace_id(req: &ServiceRequest) -> String {
        // Try to get from header or generate new
        req.headers()
            .get("X-Trace-ID")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string())
            .unwrap_or_else(|| uuid::Uuid::new_v4().to_string())
    }
}

impl Default for TracingMiddleware {
    fn default() -> Self {
        Self::new()
    }
}

impl<S, B> actix_web::dev::Transform<S, ServiceRequest> for TracingMiddleware
where
    S: actix_web::dev::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>
        + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = TracingMiddlewareService<S>;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Transform, Self::InitError>>>>;

    fn new_transform(&self, service: S) -> Self::Future {
        Box::pin(async move { Ok(TracingMiddlewareService { service }) })
    }
}

/// Tracing middleware service
pub struct TracingMiddlewareService<S> {
    service: S,
}

impl<S, B> actix_web::dev::Service<ServiceRequest> for TracingMiddlewareService<S>
where
    S: actix_web::dev::Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>
        + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    #[instrument(
        name = "request"
        skip(self, req),
        fields(
            method = %req.method(),
            uri = %req.uri(),
        )
    )]
    fn call(&self, req: ServiceRequest) -> Self::Future {
        let trace_id = Self::extract_trace_id(&req);
        
        info!("Starting request with trace_id: {}", trace_id);

        let future = self.service.call(req);

        Box::pin(async move {
            let result = future.await?;
            info!("Request completed");
            Ok(result)
        })
    }
}
