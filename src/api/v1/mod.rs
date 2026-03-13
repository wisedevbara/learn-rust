//! API v1 Module
//!
//! Contains all v1 API endpoints.

#![allow(dead_code)]

pub mod auth;
pub mod users;

// Health check module - included in main.rs routing
pub mod health {
    use actix_web::{web, HttpResponse, Result};

    /// Health check endpoint
    pub fn configure(cfg: &mut web::ServiceConfig) {
        cfg.service(web::resource("/health").to(health_check));
    }

    async fn health_check() -> Result<HttpResponse> {
        Ok(HttpResponse::Ok().json(serde_json::json!({
            "status": "healthy",
            "service": "rust-backend-framework"
        })))
    }
}
