//! Authentication Middleware
//!
//! JWT-based authentication middleware.

#![allow(dead_code)]

use actix_web::{dev::ServiceRequest, Error, HttpMessage, HttpResponse, Result};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use jsonwebtoken::{decode, DecodingKey, Validation};
use uuid::Uuid;

use crate::config::app::JwtConfig;
use crate::error::app_error::AppError;

/// JWT claims structure for access tokens
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Claims {
    pub sub: String,      // User ID
    pub email: String,
    pub role: String,
    pub exp: i64,
    pub iat: i64,
    pub token_type: String,
}

impl Claims {
    /// Get user ID as Uuid
    pub fn user_id(&self) -> Result<Uuid, uuid::UuidError> {
        Uuid::parse_str(&self.sub)
    }
}

/// Authentication middleware extractor
pub struct AuthMiddleware {
    jwt_config: Arc<JwtConfig>,
}

impl AuthMiddleware {
    /// Create new auth middleware with JWT config
    pub fn new(jwt_config: Arc<JwtConfig>) -> Self {
        Self { jwt_config }
    }
    
    /// Extract and validate token from Authorization header
    pub fn extract_token(req: &ServiceRequest) -> Result<String, Error> {
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|v| v.to_str().ok());

        match auth_header {
            Some(auth_header) => {
                if let Some(token) = auth_header.strip_prefix("Bearer ") {
                    Ok(token.to_string())
                } else {
                    Err(actix_web::error::ErrorUnauthorized("Invalid authorization header format"))
                }
            }
            None => Err(actix_web::error::ErrorUnauthorized("Missing authorization header")),
        }
    }
    
    /// Validate token and extract claims
    pub fn validate_token(&self, token: &str) -> Result<Claims, AppError> {
        let token_data = decode::<Claims>(
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
}

/// Actix-web middleware implementation for JWT authentication
impl actix_web::dev::Middleware<ServiceRequest> for AuthMiddleware {
    fn call(
        &self,
        req: ServiceRequest,
        ext: &mut dyn FnMut(
            &mut ServiceRequest,
        ) -> Pin<Box<dyn Future<Output = Result<HttpResponse, Error>> + Send>>,
    ) -> Pin<Box<dyn Future<Output = Result<HttpResponse, Error>> + Send>> {
        Box::pin(async move {
            // Extract token
            let token = match Self::extract_token(&req) {
                Ok(t) => t,
                Err(e) => return Err(e),
            };
            
            // Validate token
            match self.validate_token(&token) {
                Ok(claims) => {
                    // Extract user ID and add to request extensions
                    match claims.user_id() {
                        Ok(user_id) => {
                            req.extensions_mut().insert(user_id);
                            req.extensions_mut().insert(claims);
                        }
                        Err(_) => {
                            return Ok(HttpResponse::Unauthorized()
                                .json(serde_json::json!({
                                    "error": "Invalid user ID in token",
                                    "code": "INVALID_TOKEN"
                                })));
                        }
                    }
                }
                Err(e) => {
                    return Ok(HttpResponse::Unauthorized()
                        .json(serde_json::json!({
                            "error": e.message(),
                            "code": e.code().to_string()
                        })));
                }
            };
            
            // Continue to next middleware/handler
            ext(&mut req.clone()).await
        })
    }
}

/// Optional auth middleware - doesn't fail if no token is provided
pub struct OptionalAuthMiddleware {
    jwt_config: Arc<JwtConfig>,
}

impl OptionalAuthMiddleware {
    pub fn new(jwt_config: Arc<JwtConfig>) -> Self {
        Self { jwt_config }
    }
}

impl actix_web::dev::Middleware<ServiceRequest> for OptionalAuthMiddleware {
    fn call(
        &self,
        req: ServiceRequest,
        ext: &mut dyn FnMut(
            &mut ServiceRequest,
        ) -> Pin<Box<dyn Future<Output = Result<HttpResponse, Error>> + Send>>,
    ) -> Pin<Box<dyn Future<Output = Result<HttpResponse, Error>> + Send>> {
        Box::pin(async move {
            // Try to extract token (doesn't fail if missing)
            if let Ok(token) = Self::extract_token(&req) {
                // Try to validate token
                let token_data = decode::<Claims>(
                    &token,
                    &DecodingKey::from_secret(self.jwt_config.secret.as_bytes()),
                    &Validation::default(),
                );
                
                if let Ok(data) = token_data {
                    if data.claims.token_type == "access" {
                        if let Ok(user_id) = data.claims.user_id() {
                            req.extensions_mut().insert(user_id);
                            req.extensions_mut().insert(data.claims);
                        }
                    }
                }
            }
            
            // Continue regardless of whether token was valid
            ext(&mut req.clone()).await
        })
    }
}

/// Extension trait for Request to get user ID
pub trait RequestUserId {
    fn user_id(&self) -> Option<Uuid>;
}

impl RequestUserId for actix_web::HttpRequest {
    fn user_id(&self) -> Option<Uuid> {
        self.extensions().get::<Uuid>().copied()
    }
}

/// Extension trait for Request to get claims
pub trait RequestClaims {
    fn claims(&self) -> Option<&Claims>;
}

impl RequestClaims for actix_web::HttpRequest {
    fn claims(&self) -> Option<&Claims> {
        self.extensions().get::<Claims>()
    }
}

/// Data extractor for authenticated user ID
pub struct AuthenticatedUser;

impl<S, B> actix_web::dev::FromRequest<S, B> for AuthenticatedUser
where
    S: actix_web::dev::ServiceRequest,
    B: actix_web::body::MessageBody,
{
    type Future = Pin<Box<dyn Future<Output = Result<AuthenticatedUser, actix_web::Error>> + Send>>;
    type Config = actix_web::dev::ExtractDefaultConfig;

    async fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> Result<Self, actix_web::Error> {
        if let Some(user_id) = req.user_id() {
            Ok(AuthenticatedUser)
        } else {
            Err(actix_web::error::ErrorUnauthorized(
                "Authentication required",
            ))
        }
    }
}
