//! Password Validation Unit Tests
//!
//! Tests for password validation, hashing, and verification.

#![cfg(test)]

use pretty_assertions::assert_eq;

use rust_backend_framework::business::services::auth_service::AuthServiceImpl;
use rust_backend_framework::error::app_error::AppError;

mod tests {
    use super::*;

    // ===========================================================================
    // Password Validation Tests
    // ===========================================================================

    #[test]
    fn test_password_validation_valid_simple() {
        // Arrange
        let password = "Password1";
        
        // Act
        let result = AuthServiceImpl::validate_password(password);
        
        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_password_validation_valid_longer() {
        // Arrange
        let password = "MySecurePass123";
        
        // Act
        let result = AuthServiceImpl::validate_password(password);
        
        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_password_validation_valid_complex() {
        // Arrange
        let password = "Str0ng#P@ssw0rd!";
        
        // Act
        let result = AuthServiceImpl::validate_password(password);
        
        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_password_validation_too_short() {
        // Arrange
        let password = "Pass1";
        
        // Act
        let result = AuthServiceImpl::validate_password(password);
        
        // Assert
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, AppError::ValidationError(_)));
    }

    #[test]
    fn test_password_validation_exactly_7_chars() {
        // Arrange
        let password = "Pass123"; // 7 chars
        
        // Act
        let result = AuthServiceImpl::validate_password(password);
        
        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_password_validation_exactly_8_chars() {
        // Arrange
        let password = "Password1"; // 8 chars
        
        // Act
        let result = AuthServiceImpl::validate_password(password);
        
        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_password_validation_no_uppercase() {
        // Arrange
        let password = "password1";
        
        // Act
        let result = AuthServiceImpl::validate_password(password);
        
        // Assert
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, AppError::ValidationError(msg)) 
            if msg.contains("uppercase"));
    }

    #[test]
    fn test_password_validation_no_number() {
        // Arrange
        let password = "Password";
        
        // Act
        let result = AuthServiceImpl::validate_password(password);
        
        // Assert
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, AppError::ValidationError(msg)) 
            if msg.contains("number"));
    }

    #[test]
    fn test_password_validation_only_lowercase_and_number() {
        // Arrange
        let password = "password1";
        
        // Act
        let result = AuthServiceImpl::validate_password(password);
        
        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_password_validation_only_uppercase_and_number() {
        // Arrange
        let password = "PASSWORD1";
        
        // Act
        let result = AuthServiceImpl::validate_password(password);
        
        // Assert
        assert!(result.is_ok());
    }

    #[test]
    fn test_password_validation_empty() {
        // Arrange
        let password = "";
        
        // Act
        let result = AuthServiceImpl::validate_password(password);
        
        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_password_validation_whitespace() {
        // Arrange
        let password = "    ";
        
        // Act
        let result = AuthServiceImpl::validate_password(password);
        
        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_password_validation_special_chars() {
        // Arrange
        let password = "P@ssw0rd!";
        
        // Act
        let result = AuthServiceImpl::validate_password(password);
        
        // Assert
        assert!(result.is_ok());
    }

    // ===========================================================================
    // Password Hashing Tests
    // ===========================================================================

    #[test]
    fn test_password_hash_success() {
        // Arrange
        let password = "TestPassword123";
        
        // Act
        let result = AuthServiceImpl::hash_password(password);
        
        // Assert
        assert!(result.is_ok());
        let hash = result.unwrap();
        assert!(!hash.is_empty());
        // Argon2 hash starts with $argon2
        assert!(hash.starts_with("$argon2"));
    }

    #[test]
    fn test_password_hash_different_hashes() {
        // Arrange
        let password = "TestPassword123";
        
        // Act
        let hash1 = AuthServiceImpl::hash_password(password).unwrap();
        let hash2 = AuthServiceImpl::hash_password(password).unwrap();
        
        // Assert
        // Same password should produce different hashes due to random salt
        assert_ne!(hash1, hash2);
    }

    #[test]
    fn test_password_verify_success() {
        // Arrange
        let password = "TestPassword123";
        let hash = AuthServiceImpl::hash_password(password).unwrap();
        
        // Act
        let result = AuthServiceImpl::verify_password(password, &hash);
        
        // Assert
        assert!(result.is_ok());
        assert!(result.unwrap());
    }

    #[test]
    fn test_password_verify_wrong_password() {
        // Arrange
        let password = "TestPassword123";
        let wrong_password = "WrongPassword456";
        let hash = AuthServiceImpl::hash_password(password).unwrap();
        
        // Act
        let result = AuthServiceImpl::verify_password(wrong_password, &hash);
        
        // Assert
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_password_verify_invalid_hash() {
        // Arrange
        let password = "TestPassword123";
        let invalid_hash = "invalid_hash";
        
        // Act
        let result = AuthServiceImpl::verify_password(password, invalid_hash);
        
        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_password_verify_empty_password() {
        // Arrange
        let password = "";
        let hash = AuthServiceImpl::hash_password("TestPassword123").unwrap();
        
        // Act
        let result = AuthServiceImpl::verify_password(password, &hash);
        
        // Assert
        assert!(result.is_ok());
        assert!(!result.unwrap());
    }

    #[test]
    fn test_password_hash_and_verify_roundtrip() {
        // Arrange
        let passwords = [
            "Simple1",
            "ComplexP@ssw0rd",
            "VeryLongPassword123456789",
            "P@ssw0rd!#$%^&*()",
        ];
        
        // Act & Assert
        for password in passwords.iter() {
            let hash = AuthServiceImpl::hash_password(password).unwrap();
            let verified = AuthServiceImpl::verify_password(password, &hash).unwrap();
            assert!(verified, "Failed to verify password: {}", password);
        }
    }
}
