//! User Entity
//!
//! Core domain entity for user.

#![allow(dead_code)]

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// User role enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    Admin,
    User,
    Guest,
}

impl Default for Role {
    fn default() -> Self {
        Role::User
    }
}

impl std::fmt::Display for Role {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Role::Admin => write!(f, "admin"),
            Role::User => write!(f, "user"),
            Role::Guest => write!(f, "guest"),
        }
    }
}

impl Role {
    /// Parse role from string
    pub fn from_string(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "admin" => Role::Admin,
            "guest" => Role::Guest,
            _ => Role::User,
        }
    }
    
    /// Parse role from optional string (returns User if None)
    pub fn from_optional(s: Option<String>) -> Self {
        match s {
            Some(role) => Role::from_string(&role),
            None => Role::User,
        }
    }
}

/// User domain entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    /// Create a new user
    pub fn new(email: String, password_hash: String, role: Role) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            email,
            password_hash,
            role,
            created_at: now,
            updated_at: now,
        }
    }
    
    /// Create a new user with specific ID (for database retrieval)
    pub fn with_id(id: Uuid, email: String, password_hash: String, role: Role, created_at: DateTime<Utc>, updated_at: DateTime<Utc>) -> Self {
        Self {
            id,
            email,
            password_hash,
            role,
            created_at,
            updated_at,
        }
    }

    /// Check if user is admin
    pub fn is_admin(&self) -> bool {
        self.role == Role::Admin
    }
}
