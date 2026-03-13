//! Cache Service Module
//!
//! Redis cache service implementation.

#![allow(dead_code)]

use async_trait::async_trait;

use crate::error::app_error::AppError;

/// Cache service trait
#[async_trait]
pub trait CacheService: Send + Sync {
    /// Get value from cache
    async fn get(&self, key: &str) -> Result<Option<String>, AppError>;

    /// Set value in cache with TTL
    async fn set(&self, key: &str, value: &str, ttl_secs: u64) -> Result<(), AppError>;

    /// Delete value from cache
    async fn delete(&self, key: &str) -> Result<(), AppError>;

    /// Check if key exists
    async fn exists(&self, key: &str) -> Result<bool, AppError>;
}
