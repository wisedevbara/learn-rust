//! Database Models Module
//!
//! Contains database models for SQLx.

#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::business::entities::user::{Role, User};

/// Database model for users table
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserRow {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserRow {
    /// Convert to domain User entity
    pub fn to_domain(&self) -> User {
        User {
            id: self.id,
            email: self.email.clone(),
            password_hash: self.password_hash.clone(),
            role: Role::from_string(&self.role),
            created_at: self.created_at,
            updated_at: self.updated_at,
        }
    }
    
    /// Create from domain User entity
    pub fn from_domain(user: &User) -> Self {
        Self {
            id: user.id,
            email: user.email.clone(),
            password_hash: user.password_hash.clone(),
            role: user.role.to_string(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

/// Database model for refresh_tokens table
#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct RefreshTokenRow {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token_hash: String,
    pub expires_at: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

impl RefreshTokenRow {
    /// Create a new refresh token row
    pub fn new(user_id: Uuid, token_hash: String, expires_at: DateTime<Utc>) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            token_hash,
            expires_at,
            created_at: Utc::now(),
        }
    }
    
    /// Check if token is expired
    pub fn is_expired(&self) -> bool {
        Utc::now() > self.expires_at
    }
}
