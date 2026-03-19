//! Rust Backend Framework Library
//!
//! A production-ready REST API built with Actix-web and Rust.
//!
//! ## Architecture
//!
//! This application follows a layered architecture:
//! - **API Layer** (`src/api/`) - HTTP handlers and route definitions
//! - **Business Layer** (`src/business/`) - Domain entities and business logic
//! - **Data Layer** (`src/data/`) - Database access and repositories
//! - **Services Layer** (`src/services/`) - External service integrations
//! - **Config** (`src/config/`) - Configuration management
//! - **Error** (`src/error/`) - Centralized error handling

#![allow(dead_code)]

use std::sync::Arc;

pub mod api;
pub mod business;
pub mod config;
pub mod data;
pub mod error;
pub mod services;

use config::app::JwtConfig;
use data::repositories::user_repository::UserRepository;

/// Shared application state for all endpoints
#[derive(Clone)]
pub struct AppState {
    pub user_repo: Arc<dyn UserRepository>,
    pub jwt_config: Arc<JwtConfig>,
}

// Re-export commonly used types
pub use actix_web;
pub use anyhow;
pub use serde;
pub use tokio;

/// Library version
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
