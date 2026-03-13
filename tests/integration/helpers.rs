//! Integration Test Helpers
//!
//! Test utilities and helper functions for integration tests.

#![allow(dead_code)]

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use actix_web::web;
use actix_web::App;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use uuid::Uuid;

use rust_backend_framework::business::entities::user::{Role, User};
use rust_backend_framework::business::services::auth_service::AuthServiceImpl;
use rust_backend_framework::config::app::JwtConfig;
use rust_backend_framework::data::models::RefreshTokenRow;
use rust_backend_framework::data::repositories::user_repository::UserRepository;
use rust_backend_framework::error::app_error::AppError;

/// In-memory user repository for testing
pub struct InMemoryUserRepository {
    users: Mutex<HashMap<Uuid, User>>,
    email_index: Mutex<HashMap<String, Uuid>>,
    refresh_tokens: Mutex<HashMap<String, RefreshTokenRow>>,
}

impl InMemoryUserRepository {
    pub fn new() -> Self {
        Self {
            users: Mutex::new(HashMap::new()),
            email_index: Mutex::new(HashMap::new()),
            refresh_tokens: Mutex::new(HashMap::new()),
        }
    }

    pub fn with_users(users: Vec<User>) -> Self {
        let mut repo = Self::new();
        for user in users {
            let id = user.id;
            let email = user.email.clone();
            repo.users.lock().unwrap().insert(id, user);
            repo.email_index.lock().unwrap().insert(email, id);
        }
        repo
    }
}

#[async_trait]
impl UserRepository for InMemoryUserRepository {
    async fn create(&self, user: &User) -> Result<User, AppError> {
        let mut users = self.users.lock().unwrap();
        let mut email_index = self.email_index.lock().unwrap();
        
        // Check if email exists
        if email_index.contains_key(&user.email) {
            return Err(AppError::AlreadyExists(format!(
                "Email {} already exists",
                user.email
            )));
        }
        
        users.insert(user.id, user.clone());
        email_index.insert(user.email.clone(), user.id);
        
        Ok(user.clone())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        let users = self.users.lock().unwrap();
        Ok(users.get(&id).cloned())
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        let email_index = self.email_index.lock().unwrap();
        let users = self.users.lock().unwrap();
        
        if let Some(id) = email_index.get(email) {
            Ok(users.get(id).cloned())
        } else {
            Ok(None)
        }
    }

    async fn update(&self, user: &User) -> Result<User, AppError> {
        let mut users = self.users.lock().unwrap();
        
        if !users.contains_key(&user.id) {
            return Err(AppError::NotFound(format!("User {} not found", user.id)));
        }
        
        users.insert(user.id, user.clone());
        Ok(user.clone())
    }

    async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let mut users = self.users.lock().unwrap();
        let mut email_index = self.email_index.lock().unwrap();
        
        if let Some(user) = users.remove(&id) {
            email_index.remove(&user.email);
        }
        
        Ok(())
    }

    async fn list_all(&self) -> Result<Vec<User>, AppError> {
        let users = self.users.lock().unwrap();
        Ok(users.values().cloned().collect())
    }

    async fn list_paginated(&self, page: i64, per_page: i64) -> Result<(Vec<User>, i64), AppError> {
        let users = self.users.lock().unwrap();
        let all_users: Vec<User> = users.values().cloned().collect();
        let total = all_users.len() as i64;
        
        let start = (page - 1) * per_page;
        let end = std::cmp::min(start + per_page, all_users.len() as i64);
        
        if start >= all_users.len() as i64 {
            return Ok((Vec::new(), total));
        }
        
        let page_users: Vec<User> = all_users[start as usize..end as usize].to_vec();
        Ok((page_users, total))
    }

    async fn email_exists(&self, email: &str) -> Result<bool, AppError> {
        let email_index = self.email_index.lock().unwrap();
        Ok(email_index.contains_key(email))
    }
}

/// Test configuration for JWT
pub fn test_jwt_config() -> JwtConfig {
    JwtConfig {
        secret: "test-secret-key-for-integration-tests-only".to_string(),
        access_token_expiry: 900,  // 15 minutes
        refresh_token_expiry: 604800,  // 7 days
        issuer: "test-issuer".to_string(),
    }
}

/// Test user data
#[derive(Clone)]
pub struct TestUser {
    pub id: Uuid,
    pub email: String,
    pub password: String,
    pub role: Role,
}

impl TestUser {
    pub fn new(email: &str, password: &str, role: Role) -> Self {
        Self {
            id: Uuid::new_v4(),
            email: email.to_string(),
            password: password.to_string(),
            role,
        }
    }

    pub fn admin() -> Self {
        Self::new("admin@test.com", "Admin1234", Role::Admin)
    }

    pub fn regular_user() -> Self {
        Self::new("user@test.com", "User1234", Role::User)
    }

    pub fn guest() -> Self {
        Self::new("guest@test.com", "Guest1234", Role::Guest)
    }

    pub fn to_user(&self) -> User {
        let password_hash = AuthServiceImpl::hash_password(&self.password).unwrap();
        User::with_id(
            self.id,
            self.email.clone(),
            password_hash,
            self.role.clone(),
            Utc::now(),
            Utc::now(),
        )
    }
}

/// Test helper for managing test users
pub struct TestUserBuilder {
    email: String,
    password: String,
    role: Role,
}

impl TestUserBuilder {
    pub fn new() -> Self {
        Self {
            email: format!("test_{}@example.com", Uuid::new_v4()),
            password: "Test1234".to_string(),
            role: Role::User,
        }
    }

    pub fn with_email(mut self, email: impl Into<String>) -> Self {
        self.email = email.into();
        self
    }

    pub fn with_password(mut self, password: impl Into<String>) -> Self {
        self.password = password.into();
        self
    }

    pub fn with_role(mut self, role: Role) -> Self {
        self.role = role;
        self
    }

    pub fn as_admin(self) -> Self {
        self.with_role(Role::Admin)
    }

    pub fn as_guest(self) -> Self {
        self.with_role(Role::Guest)
    }

    pub fn build(self) -> TestUser {
        TestUser {
            id: Uuid::new_v4(),
            email: self.email,
            password: self.password,
            role: self.role,
        }
    }
}

impl Default for TestUserBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Auth tokens for test users
#[derive(Clone)]
pub struct TestAuthTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: Uuid,
}

/// Helper to create authenticated user tokens
pub async fn create_test_auth_tokens(
    user: &TestUser,
    auth_service: &AuthServiceImpl,
) -> Result<TestAuthTokens, AppError> {
    // Create user in repository
    let user_entity = user.to_user();
    auth_service.find_user_by_email(&user.email).await; // This would normally be done through repository
    
    // Generate tokens
    let access_token = auth_service.generate_access_token(&user_entity).await?;
    let refresh_token = auth_service.generate_refresh_token(&user_entity).await?;
    
    Ok(TestAuthTokens {
        access_token,
        refresh_token,
        user_id: user.id,
    })
}

/// HTTP helper for making authenticated requests
pub struct TestHttpClient {
    pub access_token: Option<String>,
}

impl TestHttpClient {
    pub fn new() -> Self {
        Self { access_token: None }
    }

    pub fn with_token(token: impl Into<String>) -> Self {
        Self {
            access_token: Some(token.into()),
        }
    }

    pub fn auth_header(&self) -> Option<String> {
        self.access_token.as_ref().map(|t| format!("Bearer {}", t))
    }
}

impl Default for TestHttpClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Test app state
pub struct TestAppState {
    pub user_repository: Arc<InMemoryUserRepository>,
    pub auth_service: Arc<AuthServiceImpl>,
    pub jwt_config: JwtConfig,
}

impl TestAppState {
    pub fn new() -> Self {
        let jwt_config = test_jwt_config();
        let user_repository = Arc::new(InMemoryUserRepository::new());
        let auth_service = Arc::new(AuthServiceImpl::new(
            user_repository.clone(),
            jwt_config.clone(),
        ));

        Self {
            user_repository,
            auth_service,
            jwt_config,
        }
    }

    pub fn with_initial_users(users: Vec<TestUser>) -> Self {
        let jwt_config = test_jwt_config();
        let user_entities: Vec<User> = users.into_iter().map(|u| u.to_user()).collect();
        let user_repository = Arc::new(InMemoryUserRepository::with_users(user_entities));
        let auth_service = Arc::new(AuthServiceImpl::new(
            user_repository.clone(),
            jwt_config.clone(),
        ));

        Self {
            user_repository,
            auth_service,
            jwt_config,
        }
    }
}

impl Default for TestAppState {
    fn default() -> Self {
        Self::new()
    }
}

/// Register user payload
#[derive(serde::Serialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}

/// Login request payload
#[derive(serde::Serialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Refresh token request payload
#[derive(serde::Serialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
}

/// Update user request payload
#[derive(serde::Serialize, Deserialize)]
pub struct UpdateUserRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<String>,
}
