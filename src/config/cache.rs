//! Cache Configuration
//!
//! Redis cache configuration.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// Cache configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct CacheConfig {
    pub url: String,
    pub max_connections: u32,
    pub default_ttl: u64,  // seconds
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            url: "redis://localhost:6379".to_string(),
            max_connections: 10,
            default_ttl: 3600,  // 1 hour
        }
    }
}
