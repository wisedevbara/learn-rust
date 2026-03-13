//! Error Handling Module
//!
//! Centralized error handling for the application.

#![allow(dead_code)]

pub mod app_error;
pub mod error_handler;

pub use app_error::AppError;
pub use error_handler::not_found;
