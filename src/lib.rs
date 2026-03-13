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
//! - **Middleware** (`src/middleware/`) - Cross-cutting concerns
//! - **Config** (`src/config/`) - Configuration management
//! - **Error** (`src/error/`) - Centralized error handling

#![allow(dead_code)]

pub mod api;
pub mod business;
pub mod config;
pub mod data;
pub mod error;
pub mod middleware;
pub mod services;

// Re-export commonly used types
pub use actix_web;
pub use anyhow;
pub use serde;
pub use tokio;
