//! Email Service Module
//!
//! Email sending service implementation.

#![allow(dead_code)]

use async_trait::async_trait;

use crate::error::app_error::AppError;

/// Email service trait
#[async_trait]
pub trait EmailService: Send + Sync {
    /// Send email
    async fn send_email(
        &self,
        to: &str,
        subject: &str,
        body: &str,
    ) -> Result<(), AppError>;

    /// Send verification email
    async fn send_verification_email(
        &self,
        to: &str,
        token: &str,
    ) -> Result<(), AppError>;

    /// Send password reset email
    async fn send_password_reset_email(
        &self,
        to: &str,
        token: &str,
    ) -> Result<(), AppError>;
}
