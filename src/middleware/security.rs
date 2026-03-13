//! Security Headers Middleware
//!
//! Implements security headers from SECURITY-BASELINE.md

use actix_web::{
    body::BoxBody,
    dev::{ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage,
};
use std::future::{ready, Ready};
use std::rc::Rc;

/// Security Headers Middleware
///
/// Adds security headers to all HTTP responses:
/// - Strict-Transport-Security: max-age=31536000
/// - Content-Security-Policy: restrictive default
/// - X-Frame-Options: DENY
/// - X-Content-Type-Options: nosniff
/// - Referrer-Policy: strict-origin-when-cross-origin
pub struct SecurityHeaders;

impl SecurityHeaders {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SecurityHeaders {
    fn default() -> Self {
        Self::new()
    }
}

/// Transform implementation for SecurityHeaders middleware
impl<S, B> Transform<S, ServiceRequest> for SecurityHeaders
where
    S: actix_web::dev::Service<
            ServiceRequest,
            Response = ServiceResponse<B>,
            Error = Error,
        > + 'static,
    B: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type InitError = ();
    type Transform = SecurityHeadersMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(SecurityHeadersMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct SecurityHeadersMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> actix_web::dev::Service<ServiceRequest> for SecurityHeadersMiddleware<S>
where
    S: actix_web::dev::Service<
            ServiceRequest,
            Response = ServiceResponse<B>,
            Error = Error,
        > + 'static,
    B: 'static,
{
    type Response = ServiceResponse<BoxBody>;
    type Error = Error;
    type Future = actix_web::dev::Pipeline<BoxBody>;

    fn poll_ready(
        &self,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let srv = self.service.clone();

        Box::pin(async move {
            let res = srv.call(req).await?;
            
            // Add security headers to the response
            let mut response = res.into_response();
            let headers = response.headers_mut();
            
            // Strict-Transport-Security: max-age=31536000 (1 year)
            headers.insert(
                actix_web::http::header::STRICT_TRANSPORT_SECURITY,
                actix_web::http::HeaderValue::from_static("max-age=31536000; includeSubDomains"),
            );
            
            // Content-Security-Policy: restrictive default
            headers.insert(
                actix_web::http::header::CONTENT_SECURITY_POLICY,
                actix_web::http::HeaderValue::from_static(
                    "default-src 'self'; script-src 'self'; style-src 'self' 'unsafe-inline'",
                ),
            );
            
            // X-Frame-Options: DENY
            headers.insert(
                actix_web::http::header::X_FRAME_OPTIONS,
                actix_web::http::HeaderValue::from_static("DENY"),
            );
            
            // X-Content-Type-Options: nosniff
            headers.insert(
                actix_web::http::header::X_CONTENT_TYPE_OPTIONS,
                actix_web::http::HeaderValue::from_static("nosniff"),
            );
            
            // Referrer-Policy: strict-origin-when-cross-origin
            headers.insert(
                actix_web::http::header::REFERRER_POLICY,
                actix_web::http::HeaderValue::from_static("strict-origin-when-cross-origin"),
            );
            
            // X-XSS-Protection: 1; mode=block
            headers.insert(
                actix_web::http::header::HeaderName::from_static("x-xss-protection"),
                actix_web::http::HeaderValue::from_static("1; mode=block"),
            );
            
            Ok(response.map_into_boxed_body())
        })
    }
}
