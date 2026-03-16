//! Error Handler
//!
//! HTTP error handler implementations.

#![allow(dead_code)]

use actix_web::{HttpResponse, Result};

/// Not found handler (404)
pub async fn not_found() -> Result<HttpResponse> {
    Ok(HttpResponse::NotFound().json(serde_json::json!({
        "error": "Resource not found",
        "code": "NOT_FOUND"
    })))
}

/// Handle validation errors
#[allow(dead_code)]
pub fn handle_validation_error(errors: &validator::ValidationErrors) -> HttpResponse {
    let mut error_messages = Vec::new();
    
    for (field, errors) in errors.0.iter() {
        error_messages.push(format!("{}: validation error", field));
    }
    
    HttpResponse::BadRequest().json(serde_json::json!({
        "error": error_messages.join(", "),
        "code": "VALIDATION_ERROR"
    }))
}

/// Handle actix-web internal errors
#[allow(dead_code)]
pub fn handle_internal_error<E: std::fmt::Debug>(err: E) -> HttpResponse {
    tracing::error!("Internal error: {:?}", err);
    
    HttpResponse::InternalServerError().json(serde_json::json!({
        "error": "Internal server error",
        "code": "INTERNAL_ERROR"
    }))
}
