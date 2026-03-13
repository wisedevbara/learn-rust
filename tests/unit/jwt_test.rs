//! JWT Token Unit Tests
//!
//! Tests for JWT token generation, validation, and claims.

#![cfg(test)]

use std::sync::Arc;
use pretty_assertions::assert_eq;
use chrono::{Duration, Utc};

use rust_backend_framework::business::entities::user::{Role, User};
use rust_backend_framework::business::services::auth_service::{AuthServiceImpl, AccessClaims, RefreshClaims};
use rust_backend_framework::config::app::JwtConfig;
use rust_backend_framework::error::app_error::AppError;

mod tests {
    use super::*;

    // Helper function to create a test AuthServiceImpl
    fn create_test_auth_service() -> AuthServiceImpl {
        let jwt_config = JwtConfig {
            secret: "test-secret-key-for-testing-purposes-only".to_string(),
            access_token_expiry: 900,  // 15 minutes
            refresh_token_expiry: 604800,  // 7 days
            issuer: "test-issuer".to_string(),
        };
        
        // Create a mock repository (we'll use a simple in-memory implementation)
        // Since the trait requires async, we need to handle this differently
        // For unit tests of token generation/validation, we can test the static methods
        AuthServiceImpl::new(
            Arc::new(MockUserRepository::new()),
            jwt_config,
        )
    }

    // Simple mock repository for testing
    struct MockUserRepository {
        users: std::collections::HashMap<String, User>,
    }

    impl MockUserRepository {
        fn new() -> Self {
            Self {
                users: std::collections::HashMap::new(),
            }
        }
    }

    use async_trait::async_trait;
    use uuid::Uuid;
    use crate::data::repositories::user_repository::UserRepository;
    use crate::data::models::RefreshTokenRow;

    #[async_trait]
    impl UserRepository for MockUserRepository {
        async fn create(&self, _user: &User) -> Result<User, AppError> {
            Ok(_user.clone())
        }

        async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError> {
            Ok(self.users.get(&id.to_string()).cloned())
        }

        async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
            Ok(self.users.values().find(|u| u.email == email).cloned())
        }

        async fn update(&self, _user: &User) -> Result<User, AppError> {
            Ok(_user.clone())
        }

        async fn delete(&self, _id: Uuid) -> Result<(), AppError> {
            Ok(())
        }

        async fn list_all(&self) -> Result<Vec<User>, AppError> {
            Ok(self.users.values().cloned().collect())
        }

        async fn list_paginated(&self, _page: i64, _per_page: i64) -> Result<(Vec<User>, i64), AppError> {
            Ok((self.users.values().cloned().collect(), self.users.len() as i64))
        }

        async fn email_exists(&self, _email: &str) -> Result<bool, AppError> {
            Ok(false)
        }
    }

    // ===========================================================================
    // Access Token Tests
    // ===========================================================================

    #[test]
    fn test_generate_access_token() {
        // Arrange
        let auth_service = create_test_auth_service();
        let user = User::new(
            "test@example.com".to_string(),
            "hashed_password".to_string(),
            Role::User,
        );
        
        // Act
        let result = auth_service.generate_access_token(&user);
        
        // Assert
        assert!(result.is_ok());
        let token = result.unwrap();
        assert!(!token.is_empty());
    }

    #[test]
    fn test_generate_access_token_contains_claims() {
        // Arrange
        let auth_service = create_test_auth_service();
        let user = User::new(
            "test@example.com".to_string(),
            "hashed_password".to_string(),
            Role::Admin,
        );
        
        // Act
        let token = auth_service.generate_access_token(&user).unwrap();
        let claims = auth_service.validate_access_token(&token).unwrap();
        
        // Assert
        assert_eq!(claims.sub, user.id.to_string());
        assert_eq!(claims.email, user.email);
        assert_eq!(claims.role, "admin");
        assert_eq!(claims.token_type, "access");
    }

    #[test]
    fn test_validate_access_token_success() {
        // Arrange
        let auth_service = create_test_auth_service();
        let user = User::new(
            "user@example.com".to_string(),
            "hash".to_string(),
            Role::User,
        );
        let token = auth_service.generate_access_token(&user).unwrap();
        
        // Act
        let result = auth_service.validate_access_token(&token);
        
        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_access_token_invalid() {
        // Arrange
        let auth_service = create_test_auth_service();
        let invalid_token = "invalid.token.string";
        
        // Act
        let result = auth_service.validate_access_token(invalid_token);
        
        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_access_token_wrong_secret() {
        // Arrange
        let auth_service1 = create_test_auth_service();
        let user = User::new(
            "test@example.com".to_string(),
            "hash".to_string(),
            Role::User,
        );
        let token = auth_service1.generate_access_token(&user).unwrap();
        
        // Create another service with different secret
        let jwt_config2 = JwtConfig {
            secret: "different-secret".to_string(),
            access_token_expiry: 900,
            refresh_token_expiry: 604800,
            issuer: "test-issuer".to_string(),
        };
        let auth_service2 = AuthServiceImpl::new(
            Arc::new(MockUserRepository::new()),
            jwt_config2,
        );
        
        // Act
        let result = auth_service2.validate_access_token(&token);
        
        // Assert
        assert!(result.is_err());
    }

    // ===========================================================================
    // Refresh Token Tests
    // ===========================================================================

    #[test]
    fn test_generate_refresh_token() {
        // Arrange
        let auth_service = create_test_auth_service();
        let user = User::new(
            "test@example.com".to_string(),
            "hashed_password".to_string(),
            Role::User,
        );
        
        // Act
        let result = auth_service.generate_refresh_token(&user);
        
        // Assert
        assert!(result.is_ok());
        let token = result.unwrap();
        assert!(!token.is_empty());
    }

    #[test]
    fn test_generate_refresh_token_contains_claims() {
        // Arrange
        let auth_service = create_test_auth_service();
        let user = User::new(
            "test@example.com".to_string(),
            "hashed_password".to_string(),
            Role::User,
        );
        
        // Act
        let token = auth_service.generate_refresh_token(&user).unwrap();
        let claims = auth_service.validate_refresh_token(&token).unwrap();
        
        // Assert
        assert_eq!(claims.sub, user.id.to_string());
        assert_eq!(claims.token_type, "refresh");
        assert!(!claims.jti.is_empty());
    }

    #[test]
    fn test_validate_refresh_token_success() {
        // Arrange
        let auth_service = create_test_auth_service();
        let user = User::new(
            "user@example.com".to_string(),
            "hash".to_string(),
            Role::User,
        );
        let token = auth_service.generate_refresh_token(&user).unwrap();
        
        // Act
        let result = auth_service.validate_refresh_token(&token);
        
        // Assert
        assert!(result.is_ok());
    }

    // ===========================================================================
    // Token Expiry Tests
    // ===========================================================================

    #[test]
    fn test_access_token_has_expiry() {
        // Arrange
        let auth_service = create_test_auth_service();
        let user = User::new(
            "test@example.com".to_string(),
            "hashed_password".to_string(),
            Role::User,
        );
        
        // Act
        let token = auth_service.generate_access_token(&user).unwrap();
        let claims = auth_service.validate_access_token(&token).unwrap();
        
        // Assert
        let now = Utc::now().timestamp();
        assert!(claims.exp > now);
        assert!(claims.iat <= now);
    }

    #[test]
    fn test_refresh_token_has_expiry() {
        // Arrange
        let auth_service = create_test_auth_service();
        let user = User::new(
            "test@example.com".to_string(),
            "hashed_password".to_string(),
            Role::User,
        );
        
        // Act
        let token = auth_service.generate_refresh_token(&user).unwrap();
        let claims = auth_service.validate_refresh_token(&token).unwrap();
        
        // Assert
        let now = Utc::now().timestamp();
        assert!(claims.exp > now);
        // Refresh token should expire in approximately 7 days
        let seven_days = 7 * 24 * 60 * 60;
        assert!((claims.exp - claims.iat - seven_days as i64).abs() < 60); // Allow 60 second variance
    }

    // ===========================================================================
    // Token Type Validation Tests
    // ===========================================================================

    #[test]
    fn test_access_token_rejected_as_refresh() {
        // Arrange
        let auth_service = create_test_auth_service();
        let user = User::new(
            "test@example.com".to_string(),
            "hashed_password".to_string(),
            Role::User,
        );
        let access_token = auth_service.generate_access_token(&user).unwrap();
        
        // Act
        let result = auth_service.validate_refresh_token(&access_token);
        
        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_refresh_token_rejected_as_access() {
        // Arrange
        let auth_service = create_test_auth_service();
        let user = User::new(
            "test@example.com".to_string(),
            "hashed_password".to_string(),
            Role::User,
        );
        let refresh_token = auth_service.generate_refresh_token(&user).unwrap();
        
        // Act
        let result = auth_service.validate_access_token(&refresh_token);
        
        // Assert
        assert!(result.is_err());
    }

    // ===========================================================================
    // Claims Extraction Tests
    // ===========================================================================

    #[test]
    fn test_extract_user_id_from_claims() {
        // Arrange
        let auth_service = create_test_auth_service();
        let user = User::new(
            "test@example.com".to_string(),
            "hashed_password".to_string(),
            Role::Admin,
        );
        
        // Act
        let token = auth_service.generate_access_token(&user).unwrap();
        let claims = auth_service.validate_access_token(&token).unwrap();
        
        // Assert
        let user_id = Uuid::parse_str(&claims.sub);
        assert!(user_id.is_ok());
        assert_eq!(user_id.unwrap(), user.id);
    }

    #[test]
    fn test_extract_email_from_claims() {
        // Arrange
        let auth_service = create_test_auth_service();
        let user = User::new(
            "unique@example.com".to_string(),
            "hashed_password".to_string(),
            Role::User,
        );
        
        // Act
        let token = auth_service.generate_access_token(&user).unwrap();
        let claims = auth_service.validate_access_token(&token).unwrap();
        
        // Assert
        assert_eq!(claims.email, "unique@example.com");
    }

    #[test]
    fn test_extract_role_from_claims() {
        // Arrange
        let auth_service = create_test_auth_service();
        
        // Test Admin
        let admin_user = User::new(
            "admin@example.com".to_string(),
            "hash".to_string(),
            Role::Admin,
        );
        let admin_token = auth_service.generate_access_token(&admin_user).unwrap();
        let admin_claims = auth_service.validate_access_token(&admin_token).unwrap();
        assert_eq!(admin_claims.role, "admin");
        
        // Test User
        let regular_user = User::new(
            "user@example.com".to_string(),
            "hash".to_string(),
            Role::User,
        );
        let user_token = auth_service.generate_access_token(&regular_user).unwrap();
        let user_claims = auth_service.validate_access_token(&user_token).unwrap();
        assert_eq!(user_claims.role, "user");
        
        // Test Guest
        let guest_user = User::new(
            "guest@example.com".to_string(),
            "hash".to_string(),
            Role::Guest,
        );
        let guest_token = auth_service.generate_access_token(&guest_user).unwrap();
        let guest_claims = auth_service.validate_access_token(&guest_token).unwrap();
        assert_eq!(guest_claims.role, "guest");
    }

    // ===========================================================================
    // Token Hash Tests
    // ===========================================================================

    #[test]
    fn test_hash_token_sha256() {
        // Arrange
        let token = "test-token-string";
        
        // Act
        let hash = AuthServiceImpl::hash_token(token);
        
        // Assert
        assert!(!hash.is_empty());
        assert_eq!(hash.len(), 64); // SHA-256 produces 64 hex characters
    }

    #[test]
    fn test_hash_token_deterministic() {
        // Arrange
        let token = "same-token";
        
        // Act
        let hash1 = AuthServiceImpl::hash_token(token);
        let hash2 = AuthServiceImpl::hash_token(token);
        
        // Assert
        assert_eq!(hash1, hash2);
    }

    #[test]
    fn test_hash_token_different_tokens_different_hashes() {
        // Arrange
        let token1 = "token1";
        let token2 = "token2";
        
        // Act
        let hash1 = AuthServiceImpl::hash_token(token1);
        let hash2 = AuthServiceImpl::hash_token(token2);
        
        // Assert
        assert_ne!(hash1, hash2);
    }
}
