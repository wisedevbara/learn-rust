//! API Layer - HTTP Handlers and Route Definitions
//!
//! This module contains all HTTP handlers and route definitions organized by API version.

#![allow(dead_code)]

pub mod v1;

// Re-export commonly used types
pub use actix_web::{web, HttpResponse, Result};
