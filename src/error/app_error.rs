//! Application Error Types
//!
//! Centralized error types for the application.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Application error codes
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorCode {
    // Authentication errors
    Unauthorized,
    InvalidCredentials,
    TokenExpired,
    InvalidToken,
    
    // Authorization errors
    Forbidden,
    InsufficientPermissions,
    
    // Resource errors
    NotFound,
    AlreadyExists,
    
    // Validation errors
    ValidationError,
    InvalidInput,
    
    // Database errors
    DatabaseError,
    ConnectionError,
    
    // Cache errors
    CacheError,
    
    // Internal errors
    InternalError,
    NotImplemented,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ErrorCode::Unauthorized => write!(f, "UNAUTHORIZED"),
            ErrorCode::InvalidCredentials => write!(f, "INVALID_CREDENTIALS"),
            ErrorCode::TokenExpired => write!(f, "TOKEN_EXPIRED"),
            ErrorCode::InvalidToken => write!(f, "INVALID_TOKEN"),
            ErrorCode::Forbidden => write!(f, "FORBIDDEN"),
            ErrorCode::InsufficientPermissions => write!(f, "INSUFFICIENT_PERMISSIONS"),
            ErrorCode::NotFound => write!(f, "NOT_FOUND"),
            ErrorCode::AlreadyExists => write!(f, "ALREADY_EXISTS"),
            ErrorCode::ValidationError => write!(f, "VALIDATION_ERROR"),
            ErrorCode::InvalidInput => write!(f, "INVALID_INPUT"),
            ErrorCode::DatabaseError => write!(f, "DATABASE_ERROR"),
            ErrorCode::ConnectionError => write!(f, "CONNECTION_ERROR"),
            ErrorCode::CacheError => write!(f, "CACHE_ERROR"),
            ErrorCode::InternalError => write!(f, "INTERNAL_ERROR"),
            ErrorCode::NotImplemented => write!(f, "NOT_IMPLEMENTED"),
        }
    }
}

/// Application error
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Authentication error: {0}")]
    Unauthorized(String),
    
    #[error("Invalid credentials")]
    InvalidCredentials,
    
    #[error("Token expired")]
    TokenExpired,
    
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    
    #[error("Forbidden: {0}")]
    Forbidden(String),
    
    #[error("Insufficient permissions")]
    InsufficientPermissions,
    
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("Resource already exists: {0}")]
    AlreadyExists(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Invalid input: {0}")]
    InvalidInput(String),
    
    #[error("Database error: {0}")]
    DatabaseError(String),
    
    #[error("Connection error: {0}")]
    ConnectionError(String),
    
    #[error("Cache error: {0}")]
    CacheError(String),
    
    #[error("Internal error: {0}")]
    InternalError(String),
    
    #[error("Not implemented: {0}")]
    NotImplemented(String),
}

impl AppError {
    /// Get the error code
    pub fn code(&self) -> ErrorCode {
        match self {
            AppError::Unauthorized(_) => ErrorCode::Unauthorized,
            AppError::InvalidCredentials => ErrorCode::InvalidCredentials,
            AppError::TokenExpired => ErrorCode::TokenExpired,
            AppError::InvalidToken(_) => ErrorCode::InvalidToken,
            AppError::Forbidden(_) => ErrorCode::Forbidden,
            AppError::InsufficientPermissions => ErrorCode::InsufficientPermissions,
            AppError::NotFound(_) => ErrorCode::NotFound,
            AppError::AlreadyExists(_) => ErrorCode::AlreadyExists,
            AppError::ValidationError(_) => ErrorCode::ValidationError,
            AppError::InvalidInput(_) => ErrorCode::InvalidInput,
            AppError::DatabaseError(_) => ErrorCode::DatabaseError,
            AppError::ConnectionError(_) => ErrorCode::ConnectionError,
            AppError::CacheError(_) => ErrorCode::CacheError,
            AppError::InternalError(_) => ErrorCode::InternalError,
            AppError::NotImplemented(_) => ErrorCode::NotImplemented,
        }
    }

    /// Get error message
    pub fn message(&self) -> String {
        self.to_string()
    }
}

// Implement Actix-web error conversion
impl actix_web::ResponseError for AppError {
    fn error_response(&self) -> actix_web::HttpResponse {
        let status = match self.code() {
            ErrorCode::Unauthorized => actix_web::http::StatusCode::UNAUTHORIZED,
            ErrorCode::InvalidCredentials => actix_web::http::StatusCode::UNAUTHORIZED,
            ErrorCode::TokenExpired => actix_web::http::StatusCode::UNAUTHORIZED,
            ErrorCode::InvalidToken => actix_web::http::StatusCode::UNAUTHORIZED,
            ErrorCode::Forbidden => actix_web::http::StatusCode::FORBIDDEN,
            ErrorCode::InsufficientPermissions => actix_web::http::StatusCode::FORBIDDEN,
            ErrorCode::NotFound => actix_web::http::StatusCode::NOT_FOUND,
            ErrorCode::AlreadyExists => actix_web::http::StatusCode::CONFLICT,
            ErrorCode::ValidationError => actix_web::http::StatusCode::BAD_REQUEST,
            ErrorCode::InvalidInput => actix_web::http::StatusCode::BAD_REQUEST,
            ErrorCode::DatabaseError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ErrorCode::ConnectionError => actix_web::http::StatusCode::SERVICE_UNAVAILABLE,
            ErrorCode::CacheError => actix_web::http::StatusCode::SERVICE_UNAVAILABLE,
            ErrorCode::InternalError => actix_web::http::StatusCode::INTERNAL_SERVER_ERROR,
            ErrorCode::NotImplemented => actix_web::http::StatusCode::NOT_IMPLEMENTED,
        };

        actix_web::HttpResponse::build(status).json(serde_json::json!({
            "error": self.message(),
            "code": self.code().to_string()
        }))
    }
}

// Serialize/Deserialize support
impl<'de> serde::Deserialize<'de> for AppError {
    fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(AppError::InternalError("Deserialized error".to_string()))
    }
}

impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.message())
    }
}
