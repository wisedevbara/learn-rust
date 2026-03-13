//! CORS Middleware
//!
//! Cross-Origin Resource Sharing configuration.

#![allow(dead_code)]

use actix_cors::Cors;
use actix_web::http;

/// CORS configuration
pub struct CorsConfig {
    allowed_origins: Vec<String>,
    allowed_methods: Vec<http::Method>,
    allowed_headers: Vec<String>,
    allow_credentials: bool,
    max_age: u64,
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self {
            allowed_origins: vec!["http://localhost:3000".to_string()],
            allowed_methods: vec![
                http::Method::GET,
                http::Method::POST,
                http::Method::PUT,
                http::Method::DELETE,
                http::Method::PATCH,
                http::Method::OPTIONS,
            ],
            allowed_headers: vec![
                "Authorization".to_string(),
                "Content-Type".to_string(),
                "X-Requested-With".to_string(),
            ],
            allow_credentials: true,
            max_age: 3600,
        }
    }
}

impl CorsConfig {
    /// Create a new CORS configuration
    pub fn new() -> Self {
        Self::default()
    }

    /// Set allowed origins
    pub fn with_origins(mut self, origins: Vec<String>) -> Self {
        self.allowed_origins = origins;
        self
    }

    /// Build the CORS middleware
    pub fn build(&self) -> Cors {
        let mut cors = Cors::default();

        // Configure allowed origins
        for origin in &self.allowed_origins {
            if origin == "*" {
                cors = cors.allowed_origin("*");
            } else {
                cors = cors.allowed_origin(origin.as_str());
            }
        }

        // Configure allowed methods
        for method in &self.allowed_methods {
            cors = cors.allowed_method(method.clone());
        }

        // Configure allowed headers
        for header in &self.allowed_headers {
            cors = cors.allowed_header(header.as_str());
        }

        // Configure credentials
        if self.allow_credentials {
            cors = cors.allow_credentials();
        }

        // Configure max age
        cors = cors.max_age(self.max_age);

        cors
    }
}

impl Default for CorsConfig {
    fn default() -> Self {
        Self::new()
    }
}
