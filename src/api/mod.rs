//! API Layer - HTTP Handlers and Route Definitions
//!
//! This module contains all HTTP handlers and route definitions organized by API version.

#![allow(dead_code)]

use actix_web::HttpMessage;

pub mod v1;

// Re-export commonly used types
pub use actix_web::{web, HttpResponse, Result};

/// Extension trait to get user claims from request
pub trait RequestExtract {
    fn claims(&self) -> Option<crate::business::services::auth_service::AccessClaims>;
    fn user_id(&self) -> Option<uuid::Uuid>;
}

impl RequestExtract for actix_web::HttpRequest {
    fn claims(&self) -> Option<crate::business::services::auth_service::AccessClaims> {
        self.extensions().get::<crate::business::services::auth_service::AccessClaims>().cloned()
    }
    
    fn user_id(&self) -> Option<uuid::Uuid> {
        self.claims().and_then(|c| uuid::Uuid::parse_str(&c.sub).ok())
    }
}
