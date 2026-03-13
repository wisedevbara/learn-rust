//! Error Handler
//!
//! HTTP error handler implementations.

#![allow(dead_code)]

use actix_web::{HttpResponse, Result};
use actix_web::error::ResponseError;

/// Not found handler (404)
pub async fn not_found() -> Result<HttpResponse> {
    Ok(HttpResponse::NotFound().json(serde_json::json!({
        "error": "Resource not found",
        "code": "NOT_FOUND"
    })))
}

/// Handle validation errors
pub fn handle_validation_error(errors: &validator::ValidationErrors) -> HttpResponse {
    let mut error_messages = Vec::new();
    
    for (field, error) in errors.iter() {
        error_messages.push(format!("{}: {}", field, error));
    }
    
    HttpResponse::BadRequest().json(serde_json::json!({
        "error": error_messages.join(", "),
        "code": "VALIDATION_ERROR"
    }))
}

/// Handle actix-web internal errors
pub fn handle_internal_error<E: std::fmt::Debug>(err: E) -> HttpResponse {
    tracing::error!("Internal error: {:?}", err);
    
    HttpResponse::InternalServerError().json(serde_json::json!({
        "error": "Internal server error",
        "code": "INTERNAL_ERROR"
    }))
}
