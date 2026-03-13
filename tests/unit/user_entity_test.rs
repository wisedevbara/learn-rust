//! User Entity Unit Tests
//!
//! Tests for User entity and Role enum.

#![cfg(test)]

use pretty_assertions::assert_eq;
use uuid::Uuid;
use chrono::Utc;

use rust_backend_framework::business::entities::user::{Role, User};

mod tests {
    use super::*;

    // ===========================================================================
    // Role Enum Tests
    // ===========================================================================

    #[test]
    fn test_role_from_string_admin() {
        // Arrange
        let input = "admin";
        
        // Act
        let result = Role::from_string(input);
        
        // Assert
        assert_eq!(result, Role::Admin);
    }

    #[test]
    fn test_role_from_string_user() {
        // Arrange
        let input = "user";
        
        // Act
        let result = Role::from_string(input);
        
        // Assert
        assert_eq!(result, Role::User);
    }

    #[test]
    fn test_role_from_string_guest() {
        // Arrange
        let input = "guest";
        
        // Act
        let result = Role::from_string(input);
        
        // Assert
        assert_eq!(result, Role::Guest);
    }

    #[test]
    fn test_role_from_string_case_insensitive() {
        // Arrange & Act
        let admin_upper = Role::from_string("ADMIN");
        let admin_mixed = Role::from_string("Admin");
        let user_upper = Role::from_string("USER");
        
        // Assert
        assert_eq!(admin_upper, Role::Admin);
        assert_eq!(admin_mixed, Role::Admin);
        assert_eq!(user_upper, Role::User);
    }

    #[test]
    fn test_role_from_string_unknown_returns_user() {
        // Arrange
        let input = "unknown_role";
        
        // Act
        let result = Role::from_string(input);
        
        // Assert
        assert_eq!(result, Role::User);
    }

    #[test]
    fn test_role_from_optional_some() {
        // Arrange
        let input = Some("admin".to_string());
        
        // Act
        let result = Role::from_optional(input);
        
        // Assert
        assert_eq!(result, Role::Admin);
    }

    #[test]
    fn test_role_from_optional_none() {
        // Arrange
        let input: Option<String> = None;
        
        // Act
        let result = Role::from_optional(input);
        
        // Assert
        assert_eq!(result, Role::User);
    }

    #[test]
    fn test_role_default() {
        // Arrange & Act
        let default_role = Role::default();
        
        // Assert
        assert_eq!(default_role, Role::User);
    }

    #[test]
    fn test_role_display() {
        // Arrange & Act & Assert
        assert_eq!(format!("{}", Role::Admin), "admin");
        assert_eq!(format!("{}", Role::User), "user");
        assert_eq!(format!("{}", Role::Guest), "guest");
    }

    // ===========================================================================
    // User Entity Tests
    // ===========================================================================

    #[test]
    fn test_user_new() {
        // Arrange
        let email = "test@example.com".to_string();
        let password_hash = "hashed_password".to_string();
        let role = Role::User;
        
        // Act
        let user = User::new(email.clone(), password_hash.clone(), role.clone());
        
        // Assert
        assert!(!user.id.is_nil());
        assert_eq!(user.email, email);
        assert_eq!(user.password_hash, password_hash);
        assert_eq!(user.role, role);
        assert!(user.created_at <= Utc::now());
        assert!(user.updated_at <= Utc::now());
    }

    #[test]
    fn test_user_with_id() {
        // Arrange
        let id = Uuid::new_v4();
        let email = "test@example.com".to_string();
        let password_hash = "hashed_password".to_string();
        let role = Role::Admin;
        let created_at = Utc::now();
        let updated_at = Utc::now();
        
        // Act
        let user = User::with_id(
            id,
            email.clone(),
            password_hash.clone(),
            role.clone(),
            created_at,
            updated_at,
        );
        
        // Assert
        assert_eq!(user.id, id);
        assert_eq!(user.email, email);
        assert_eq!(user.password_hash, password_hash);
        assert_eq!(user.role, role);
        assert_eq!(user.created_at, created_at);
        assert_eq!(user.updated_at, updated_at);
    }

    #[test]
    fn test_user_is_admin_true() {
        // Arrange
        let user = User::new(
            "admin@example.com".to_string(),
            "hash".to_string(),
            Role::Admin,
        );
        
        // Act & Assert
        assert!(user.is_admin());
    }

    #[test]
    fn test_user_is_admin_false_for_user() {
        // Arrange
        let user = User::new(
            "user@example.com".to_string(),
            "hash".to_string(),
            Role::User,
        );
        
        // Act & Assert
        assert!(!user.is_admin());
    }

    #[test]
    fn test_user_is_admin_false_for_guest() {
        // Arrange
        let user = User::new(
            "guest@example.com".to_string(),
            "hash".to_string(),
            Role::Guest,
        );
        
        // Act & Assert
        assert!(!user.is_admin());
    }

    // ===========================================================================
    // Role Comparison Tests
    // ===========================================================================

    #[test]
    fn test_role_comparison_equal() {
        // Arrange
        let role1 = Role::Admin;
        let role2 = Role::Admin;
        
        // Act & Assert
        assert_eq!(role1, role2);
    }

    #[test]
    fn test_role_comparison_not_equal() {
        // Arrange
        let admin = Role::Admin;
        let user = Role::User;
        
        // Act & Assert
        assert_ne!(admin, user);
    }

    #[test]
    fn test_role_serialize() {
        // Arrange
        let role = Role::Admin;
        
        // Act
        let serialized = serde_json::to_string(&role).unwrap();
        
        // Assert
        assert_eq!(serialized, "\"admin\"");
    }

    #[test]
    fn test_role_deserialize() {
        // Arrange
        let json = "\"guest\"";
        
        // Act
        let role: Role = serde_json::from_str(json).unwrap();
        
        // Assert
        assert_eq!(role, Role::Guest);
    }

    #[test]
    fn test_user_serialize_deserialize() {
        // Arrange
        let user = User::new(
            "test@example.com".to_string(),
            "secret_hash".to_string(),
            Role::User,
        );
        
        // Act
        let serialized = serde_json::to_string(&user).unwrap();
        let deserialized: User = serde_json::from_str(&serialized).unwrap();
        
        // Assert
        assert_eq!(deserialized.id, user.id);
        assert_eq!(deserialized.email, user.email);
        assert_eq!(deserialized.role, user.role);
        // password_hash should be skipped in serialization
        assert!(serialized.contains("\"password_hash\":null") || !serialized.contains("secret_hash"));
    }
}
