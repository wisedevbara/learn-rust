//! Application Configuration
//!
//! Main application configuration structure.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

use super::cache::CacheConfig;
use super::database::DatabaseConfig;

/// Server configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "0.0.0.0".to_string(),
            port: 8080,
        }
    }
}

/// JWT configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct JwtConfig {
    pub secret: String,
    pub access_token_expiry: u64,  // seconds
    pub refresh_token_expiry: u64,  // seconds
    pub issuer: String,
}

impl Default for JwtConfig {
    fn default() -> Self {
        Self {
            secret: "change-me-in-production".to_string(),
            access_token_expiry: 900,  // 15 minutes
            refresh_token_expiry: 604800,  // 7 days
            issuer: "rust-backend-framework".to_string(),
        }
    }
}

/// Application configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AppConfig {
    #[serde(default)]
    pub server: ServerConfig,
    
    #[serde(default)]
    pub database: DatabaseConfig,
    
    #[serde(default)]
    pub cache: CacheConfig,
    
    #[serde(default)]
    pub jwt: JwtConfig,
    
    #[serde(default = "default_log_level")]
    pub log_level: String,
}

fn default_log_level() -> String {
    "info".to_string()
}

impl AppConfig {
    /// Load configuration from environment
    pub async fn load() -> Result<Self, anyhow::Error> {
        // Load from environment variables
        let mut config = Self::default();

        // Override with environment variables if set
        if let Ok(host) = std::env::var("SERVER_HOST") {
            config.server.host = host;
        }
        if let Ok(port) = std::env::var("SERVER_PORT") {
            config.server.port = port.parse().unwrap_or(8080);
        }
        if let Ok(db_url) = std::env::var("DATABASE_URL") {
            config.database.url = db_url;
        }
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            config.cache.url = redis_url;
        }
        if let Ok(jwt_secret) = std::env::var("JWT_SECRET") {
            config.jwt.secret = jwt_secret;
        }
        if let Ok(log_level) = std::env::var("LOG_LEVEL") {
            config.log_level = log_level;
        }

        Ok(config)
    }
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig::default(),
            database: DatabaseConfig::default(),
            cache: CacheConfig::default(),
            jwt: JwtConfig::default(),
            log_level: "info".to_string(),
        }
    }
}
