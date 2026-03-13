//! User Repository
//!
//! Handles database operations for users.

#![allow(dead_code)]

use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{postgres::PgPool, Row};
use uuid::Uuid;

use crate::business::entities::user::{Role, User};
use crate::data::models::UserRow;
use crate::error::app_error::AppError;

/// User repository trait
#[async_trait]
pub trait UserRepository: Send + Sync {
    /// Create a new user
    async fn create(&self, user: &User) -> Result<User, AppError>;

    /// Find user by ID
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;

    /// Find user by email
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;

    /// Update user
    async fn update(&self, user: &User) -> Result<User, AppError>;

    /// Delete user
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;

    /// List all users
    async fn list_all(&self) -> Result<Vec<User>, AppError>;

    /// List users with pagination
    async fn list_paginated(&self, page: i64, per_page: i64) -> Result<(Vec<User>, i64), AppError>;

    /// Check if email exists
    async fn email_exists(&self, email: &str) -> Result<bool, AppError>;
}

/// PostgreSQL implementation of UserRepository
pub struct PgUserRepository {
    pool: Arc<PgPool>,
}

impl PgUserRepository {
    /// Create a new PostgreSQL user repository
    pub fn new(pool: Arc<PgPool>) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn create(&self, user: &User) -> Result<User, AppError> {
        let result = sqlx::query(
            r#"
            INSERT INTO users (id, email, password_hash, role, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6)
            RETURNING id, email, password_hash, role, created_at, updated_at
            "#,
        )
        .bind(user.id)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(user.role.to_string())
        .bind(user.created_at)
        .bind(user.updated_at)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let row = UserRow {
            id: result.get("id"),
            email: result.get("email"),
            password_hash: result.get("password_hash"),
            role: result.get("role"),
            created_at: result.get("created_at"),
            updated_at: result.get("updated_at"),
        };

        Ok(row.to_domain())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let result = sqlx::query(
            r#"
            SELECT id, email, password_hash, role, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match result {
            Some(row) => {
                let user_row = UserRow {
                    id: row.get("id"),
                    email: row.get("email"),
                    password_hash: row.get("password_hash"),
                    role: row.get("role"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                };
                Ok(Some(user_row.to_domain()))
            }
            None => Ok(None),
        }
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let result = sqlx::query(
            r#"
            SELECT id, email, password_hash, role, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        match result {
            Some(row) => {
                let user_row = UserRow {
                    id: row.get("id"),
                    email: row.get("email"),
                    password_hash: row.get("password_hash"),
                    role: row.get("role"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                };
                Ok(Some(user_row.to_domain()))
            }
            None => Ok(None),
        }
    }

    async fn update(&self, user: &User) -> Result<User, AppError> {
        let result = sqlx::query(
            r#"
            UPDATE users
            SET email = $1, password_hash = $2, role = $3, updated_at = $4
            WHERE id = $5
            RETURNING id, email, password_hash, role, created_at, updated_at
            "#,
        )
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(user.role.to_string())
        .bind(user.updated_at)
        .bind(user.id)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let row = UserRow {
            id: result.get("id"),
            email: result.get("email"),
            password_hash: result.get("password_hash"),
            role: result.get("role"),
            created_at: result.get("created_at"),
            updated_at: result.get("updated_at"),
        };

        Ok(row.to_domain())
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        sqlx::query(
            r#"
            DELETE FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .execute(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn list_all(&self) -> Result<Vec<User>, AppError> {
        let result = sqlx::query(
            r#"
            SELECT id, email, password_hash, role, created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let users: Vec<User> = result
            .into_iter()
            .map(|row| {
                let user_row = UserRow {
                    id: row.get("id"),
                    email: row.get("email"),
                    password_hash: row.get("password_hash"),
                    role: row.get("role"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                };
                user_row.to_domain()
            })
            .collect();

        Ok(users)
    }

    async fn email_exists(&self, email: &str) -> Result<bool, AppError> {
        let result = sqlx::query(
            r#"
            SELECT EXISTS(SELECT 1 FROM users WHERE email = $1) as exists
            "#,
        )
        .bind(email)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let exists: bool = result.get("exists");
        Ok(exists)
    }

    async fn list_paginated(&self, page: i64, per_page: i64) -> Result<(Vec<User>, i64), AppError> {
        let offset = (page - 1) * per_page;
        
        // Get total count
        let count_result = sqlx::query(
            r#"SELECT COUNT(*) as count FROM users"#,
        )
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;
        
        let total: i64 = count_result.get("count");
        
        // Get paginated users
        let result = sqlx::query(
            r#"
            SELECT id, email, password_hash, role, created_at, updated_at
            FROM users
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(per_page)
        .bind(offset)
        .fetch_all(&*self.pool)
        .await
        .map_err(|e| AppError::DatabaseError(e.to_string()))?;

        let users: Vec<User> = result
            .into_iter()
            .map(|row| {
                let user_row = UserRow {
                    id: row.get("id"),
                    email: row.get("email"),
                    password_hash: row.get("password_hash"),
                    role: row.get("role"),
                    created_at: row.get("created_at"),
                    updated_at: row.get("updated_at"),
                };
                user_row.to_domain()
            })
            .collect();

        Ok((users, total))
    }
}

/// In-memory implementation of UserRepository for testing
pub struct InMemoryUserRepository {
    users: std::sync::RwLock<std::collections::HashMap<Uuid, User>>,
    emails: std::sync::RwLock<std::collections::HashSet<String>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: std::sync::RwLock::new(std::collections::HashMap::new()),
            emails: std::sync::RwLock::new(std::collections::HashSet::new()),
        }
    }
}

impl Default for InMemoryUserRepository {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn create(&self, user: &User) -> Result<User, AppError> {
        let mut users = self.users.write().map_err(|_| AppError::InternalError("Lock error".to_string()))?;
        let mut emails = self.emails.write().map_err(|_| AppError::InternalError("Lock error".to_string()))?;
        
        if emails.contains(&user.email) {
            return Err(AppError::AlreadyExists(format!("Email {} already exists", user.email)));
        }
        
        emails.insert(user.email.clone());
        users.insert(user.id, user.clone());
        
        Ok(user.clone())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let users = self.users.read().map_err(|_| AppError::InternalError("Lock error".to_string()))?;
        Ok(users.get(&id).cloned())
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let users = self.users.read().map_err(|_| AppError::InternalError("Lock error".to_string()))?;
        Ok(users.values().find(|u| u.email == email).cloned())
    }

    async fn update(&self, user: &User) -> Result<User, AppError> {
        let mut users = self.users.write().map_err(|_| AppError::InternalError("Lock error".to_string()))?;
        
        if users.contains_key(&user.id) {
            users.insert(user.id, user.clone());
            Ok(user.clone())
        } else {
            Err(AppError::NotFound(format!("User {} not found", user.id)))
        }
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let mut users = self.users.write().map_err(|_| AppError::InternalError("Lock error".to_string()))?;
        let mut emails = self.emails.write().map_err(|_| AppError::InternalError("Lock error".to_string()))?;
        
        if let Some(user) = users.remove(&id) {
            emails.remove(&user.email);
            Ok(())
        } else {
            Err(AppError::NotFound(format!("User {} not found", id)))
        }
    }

    async fn list_all(&self) -> Result<Vec<User>, AppError> {
        let users = self.users.read().map_err(|_| AppError::InternalError("Lock error".to_string()))?;
        Ok(users.values().cloned().collect())
    }

    async fn email_exists(&self, email: &str) -> Result<bool, AppError> {
        let emails = self.emails.read().map_err(|_| AppError::InternalError("Lock error".to_string()))?;
        Ok(emails.contains(email))
    }

    async fn list_paginated(&self, page: i64, per_page: i64) -> Result<(Vec<User>, i64), AppError> {
        let users = self.users.read().map_err(|_| AppError::InternalError("Lock error".to_string()))?;
        
        let total = users.len() as i64;
        let offset = ((page - 1) * per_page) as usize;
        let limit = per_page as usize;
        
        let mut all_users: Vec<User> = users.values().cloned().collect();
        // Sort by created_at descending (most recent first)
        all_users.sort_by(|a, b| b.created_at.cmp(&a.created_at));
        
        let paginated_users: Vec<User> = all_users
            .into_iter()
            .skip(offset)
            .take(limit)
            .collect();
        
        Ok((paginated_users, total))
    }
}
