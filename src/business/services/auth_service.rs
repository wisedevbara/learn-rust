//! Authentication Service
//!
//! Handles user authentication, registration, and token management.

#![allow(dead_code)]

use std::sync::Arc;

use argon2::{
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use async_trait::async_trait;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use regex::Regex;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::business::entities::user::{Role, User};
use crate::config::app::JwtConfig;
use crate::data::models::{RefreshTokenRow, UserRow};
use crate::data::repositories::user_repository::UserRepository;
use crate::error::app_error::AppError;

/// JWT claims structure for access tokens
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AccessClaims {
    pub sub: String,      // User ID
    pub email: String,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
    pub token_type: String,
}

/// Alias for AccessClaims
pub type Claims = AccessClaims;

/// JWT claims structure for refresh tokens
#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct RefreshClaims {
    pub sub: String,      // User ID
    pub jti: String,       // Token ID
    pub exp: i64,
    pub iat: i64,
    pub token_type: String,
}

/// Authentication service implementation
pub struct AuthServiceImpl {
    user_repository: Arc<dyn UserRepository>,
    jwt_config: JwtConfig,
}

impl AuthServiceImpl {
    /// Create a new authentication service
    pub fn new(user_repository: Arc<dyn UserRepository>, jwt_config: JwtConfig) -> Self {
        Self {
            user_repository,
            jwt_config,
        }
    }
    
    /// Validate password requirements: min 8 chars, 1 uppercase, 1 number
    pub fn validate_password(password: &str) -> Result<(), AppError> {
        let password_regex = Regex::new(r"^(?=.*[A-Z])(?=.*\d).{8,}$")
            .map_err(|e| AppError::InternalError(format!("Failed to create regex: {}", e)))?;
        
        if !password_regex.is_match(password) {
            return Err(AppError::ValidationError(
                "Password must be at least 8 characters with 1 uppercase letter and 1 number".to_string(),
            ));
        }
        Ok(())
    }
    
    /// Hash password using Argon2id
    pub fn hash_password(password: &str) -> Result<String, AppError> {
        // Use Argon2id with security parameters from SECURITY-BASELINE
        // memory_cost >= 65536 kB, time_cost >= 3, parallelism >= 4
        let argon2 = Argon2::new(
            argon2::Algorithm::Argon2id,
            argon2::Version::V0x13,
            argon2::Params::new(65536, 3, 4, Some(32)).map_err(|e| {
                AppError::InternalError(format!("Failed to create Argon2 params: {}", e))
            })?,
        );
        
        let salt = SaltString::generate(&mut rand::thread_rng());
        let password_hash = argon2
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| AppError::InternalError(format!("Failed to hash password: {}", e)))?
            .to_string();
        
        Ok(password_hash)
    }
    
    /// Verify password against hash
    pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
        let parsed_hash = PasswordHash::new(hash)
            .map_err(|e| AppError::InternalError(format!("Invalid password hash: {}", e)))?;
        
        let argon2 = Argon2::default();
        Ok(argon2.verify_password(password.as_bytes(), &parsed_hash).is_ok())
    }
    
    /// Generate access token
    pub fn generate_access_token(&self, user: &User) -> Result<String, AppError> {
        let now = Utc::now();
        let exp = now + Duration::seconds(self.jwt_config.access_token_expiry as i64);
        
        let claims = AccessClaims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            role: user.role.to_string(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
            token_type: "access".to_string(),
        };
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_config.secret.as_bytes()),
        )
        .map_err(|e| AppError::InternalError(format!("Failed to encode access token: {}", e)))?;
        
        Ok(token)
    }
    
    /// Generate refresh token
    pub fn generate_refresh_token(&self, user: &User) -> Result<String, AppError> {
        let now = Utc::now();
        let exp = now + Duration::seconds(self.jwt_config.refresh_token_expiry as i64);
        let jti = Uuid::new_v4().to_string();
        
        let claims = RefreshClaims {
            sub: user.id.to_string(),
            jti,
            exp: exp.timestamp(),
            iat: now.timestamp(),
            token_type: "refresh".to_string(),
        };
        
        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_config.secret.as_bytes()),
        )
        .map_err(|e| AppError::InternalError(format!("Failed to encode refresh token: {}", e)))?;
        
        Ok(token)
    }
    
    /// Hash refresh token for storage (SHA-256)
    pub fn hash_token(token: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(token.as_bytes());
        let result = hasher.finalize();
        hex::encode(result)
    }
    
    /// Validate access token and return claims
    pub fn validate_access_token(&self, token: &str) -> Result<AccessClaims, AppError> {
        let token_data: TokenData<AccessClaims> = decode(
            token,
            &DecodingKey::from_secret(self.jwt_config.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::TokenExpired,
            _ => AppError::InvalidToken(e.to_string()),
        })?;
        
        if token_data.claims.token_type != "access" {
            return Err(AppError::InvalidToken("Invalid token type".to_string()));
        }
        
        Ok(token_data.claims)
    }
    
    /// Validate refresh token and return claims
    pub fn validate_refresh_token(&self, token: &str) -> Result<RefreshClaims, AppError> {
        let token_data: TokenData<RefreshClaims> = decode(
            token,
            &DecodingKey::from_secret(self.jwt_config.secret.as_bytes()),
            &Validation::default(),
        )
        .map_err(|e| match e.kind() {
            jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::TokenExpired,
            _ => AppError::InvalidToken(e.to_string()),
        })?;
        
        if token_data.claims.token_type != "refresh" {
            return Err(AppError::InvalidToken("Invalid token type".to_string()));
        }
        
        Ok(token_data.claims)
    }
}

/// Authentication service trait
#[async_trait]
pub trait AuthService: Send + Sync {
    /// Register a new user
    async fn register(
        &self,
        email: String,
        password: String,
        role: Option<Role>,
    ) -> Result<User, AppError>;

    /// Authenticate user with email and password
    async fn login(&self, email: String, password: String) -> Result<User, AppError>;

    /// Generate access token for user
    async fn generate_access_token(&self, user: &User) -> Result<String, AppError>;

    /// Generate refresh token for user
    async fn generate_refresh_token(&self, user: &User) -> Result<String, AppError>;

    /// Validate access token and extract user ID
    async fn validate_access_token(&self, token: &str) -> Result<Uuid, AppError>;

    /// Refresh access token using refresh token
    async fn refresh_access_token(&self, refresh_token: &str) -> Result<(String, String), AppError>;

    /// Logout user (invalidate refresh token)
    async fn logout(&self, user_id: Uuid, refresh_token: &str) -> Result<(), AppError>;
    
    /// Save refresh token to database
    async fn save_refresh_token(&self, user_id: Uuid, token_hash: &str, expires_at: chrono::DateTime<Utc>) -> Result<(), AppError>;
    
    /// Find refresh token in database
    async fn find_refresh_token(&self, token_hash: &str) -> Result<Option<RefreshTokenRow>, AppError>;
    
    /// Delete refresh token from database
    async fn delete_refresh_token(&self, token_hash: &str) -> Result<(), AppError>;
    
    /// Find user by ID
    async fn find_user_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
    
    /// Find user by email
    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    
    /// Check if email exists
    async fn email_exists(&self, email: &str) -> Result<bool, AppError>;
}

#[async_trait]
impl AuthService for AuthServiceImpl {
    async fn register(
        &self,
        email: String,
        password: String,
        role: Option<Role>,
    ) -> Result<User, AppError> {
        // Validate password
        Self::validate_password(&password)?;
        
        // Check if email already exists
        if self.email_exists(&email).await? {
            return Err(AppError::AlreadyExists(format!("Email {} already registered", email)));
        }
        
        // Hash password
        let password_hash = Self::hash_password(&password)?;
        
        // Create user with role (default to User if None)
        let role = role.unwrap_or(Role::User);
        let user = User::new(email, password_hash, role);
        
        // Note: In a full implementation, this would call the repository
        // For now, we return the user object (repository integration would be done in API layer)
        Ok(user)
    }

    async fn login(&self, email: String, password: String) -> Result<User, AppError> {
        // Find user by email
        let user = self.find_user_by_email(&email).await?
            .ok_or(AppError::InvalidCredentials)?;
        
        // Verify password
        if !Self::verify_password(&password, &user.password_hash)? {
            return Err(AppError::InvalidCredentials);
        }
        
        Ok(user)
    }

    async fn generate_access_token(&self, user: &User) -> Result<String, AppError> {
        AuthServiceImpl::generate_access_token(self, user)
    }

    async fn generate_refresh_token(&self, user: &User) -> Result<String, AppError> {
        AuthServiceImpl::generate_refresh_token(self, user)
    }

    async fn validate_access_token(&self, token: &str) -> Result<Uuid, AppError> {
        let claims = AuthServiceImpl::validate_access_token(self, token)?;
        Uuid::parse_str(&claims.sub)
            .map_err(|e| AppError::InvalidToken(format!("Invalid user ID: {}", e)))
    }

    async fn refresh_access_token(&self, refresh_token: &str) -> Result<(String, String), AppError> {
        // Validate refresh token
        let claims = AuthServiceImpl::validate_refresh_token(self, refresh_token)?;
        
        // Check if token exists in database
        let token_hash = Self::hash_token(refresh_token);
        let stored_token = self.find_refresh_token(&token_hash).await?
            .ok_or(AppError::InvalidToken("Refresh token not found or revoked".to_string()))?;
        
        // Check if token is expired
        if stored_token.is_expired() {
            return Err(AppError::TokenExpired);
        }
        
        // Get user
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|e| AppError::InvalidToken(format!("Invalid user ID: {}", e)))?;
        let user = self.find_user_by_id(user_id).await?
            .ok_or(AppError::NotFound("User not found".to_string()))?;
        
        // Generate new tokens
        let new_access_token = self.generate_access_token(&user)?;
        let new_refresh_token = self.generate_refresh_token(&user)?;
        
        // Delete old refresh token (rotation)
        self.delete_refresh_token(&token_hash).await?;
        
        // Save new refresh token
        let new_token_hash = Self::hash_token(&new_refresh_token);
        let expires_at = Utc::now() + Duration::seconds(self.jwt_config.refresh_token_expiry as i64);
        self.save_refresh_token(user_id, &new_token_hash, expires_at).await?;
        
        Ok((new_access_token, new_refresh_token))
    }

    async fn logout(&self, user_id: Uuid, refresh_token: &str) -> Result<(), AppError> {
        // Hash the token and delete from database
        let token_hash = Self::hash_token(refresh_token);
        self.delete_refresh_token(&token_hash).await
    }
    
    async fn save_refresh_token(&self, user_id: Uuid, token_hash: &str, expires_at: chrono::DateTime<Utc>) -> Result<(), AppError> {
        // This would be implemented in the data layer
        // For now, return Ok as placeholder
        let _ = (user_id, token_hash, expires_at);
        Ok(())
    }
    
    async fn find_refresh_token(&self, token_hash: &str) -> Result<Option<RefreshTokenRow>, AppError> {
        // This would be implemented in the data layer
        // For now, return None as placeholder
        let _ = token_hash;
        Ok(None)
    }
    
    async fn delete_refresh_token(&self, token_hash: &str) -> Result<(), AppError> {
        // This would be implemented in the data layer
        // For now, return Ok as placeholder
        let _ = token_hash;
        Ok(())
    }
    
    async fn find_user_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
        // This would call the repository
        let _ = id;
        Ok(None)
    }
    
    async fn find_user_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        // This would call the repository
        let _ = email;
        Ok(None)
    }
    
    async fn email_exists(&self, email: &str) -> Result<bool, AppError> {
        // This would call the repository
        let _ = email;
        Ok(false)
    }
}
