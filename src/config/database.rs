//! Database Configuration
//!
//! PostgreSQL database configuration.

#![allow(dead_code)]

use serde::{Deserialize, Serialize};

/// Database configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub acquire_timeout: u64,
    pub idle_timeout: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            url: "postgres://postgres:postgres@localhost:5432/app".to_string(),
            max_connections: 10,
            min_connections: 2,
            acquire_timeout: 30,
            idle_timeout: 600,
        }
    }
}
