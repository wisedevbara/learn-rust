//! Authentication API Endpoints
//!
//! Contains handlers for user registration, login, token refresh, and logout.

#![allow(dead_code)]

use std::sync::Arc;

use actix_web::{web, HttpResponse, Result};
use chrono::{Duration, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::business::entities::user::Role;
use crate::business::services::auth_service::{AuthService, AuthServiceImpl, RefreshClaims};
use crate::config::app::JwtConfig;
use crate::data::models::RefreshTokenRow;
use crate::data::repositories::user_repository::UserRepository;
use crate::error::app_error::AppError;
use crate::middleware::auth::{Claims, RequestUserId};

/// Register request payload
#[derive(Debug, Deserialize, Serialize, Validator)]
#[validator(schema(rules(
    email = "required,email",
    password = "required,minLength(8)"
)))]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
    pub role: Option<String>,
}

/// Login request payload
#[derive(Debug, Deserialize, Serialize, Validator)]
#[validator(schema(rules(
    email = "required,email",
    password = "required"
)))]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

/// Token refresh request payload
#[derive(Debug, Deserialize, Serialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

/// Logout request payload
#[derive(Debug, Deserialize, Serialize)]
pub struct LogoutRequest {
    pub refresh_token: Option<String>,
}

/// Authentication response
#[derive(Debug, Serialize)]
pub struct AuthResponse {
    pub access_token: String,
    pub refresh_token: String,
    #[serde(rename = "token_type")]
    pub token_type: String,
    #[serde(rename = "expires_in")]
    pub expires_in: u64,
}

/// Application state for authentication
pub struct AuthState {
    pub auth_service: Arc<dyn AuthService>,
    pub jwt_config: Arc<JwtConfig>,
}

impl AuthState {
    pub fn new(auth_service: Arc<dyn AuthService>, jwt_config: Arc<JwtConfig>) -> Self {
        Self {
            auth_service,
            jwt_config,
        }
    }
}

/// Configure authentication routes
/// Takes shared app_state from main.rs to ensure single repository instance
pub fn configure(cfg: &mut web::ServiceConfig, app_state: &crate::main::AppState) {
    let jwt_config = app_state.jwt_config.clone();
    let auth_service: Arc<dyn AuthService> = Arc::new(
        AuthServiceImpl::new(app_state.user_repo.clone(), jwt_config.clone())
    );
    
    let state = AuthState::new(auth_service, jwt_config);
    
    cfg.app_data(web::Data::new(state))
        .service(
            web::scope("/auth")
                .service(register)
                .service(login)
                .service(refresh)
                .service(logout),
        );
}

/// Register new user
#[utoipa::path(
    post,
    path = "/api/v1/auth/register",
    tag = "Authentication",
    request_body = RegisterRequest,
    responses(
        (status = 201, description = "User registered successfully", body = AuthResponse),
        (status = 400, description = "Invalid request"),
        (status = 409, description = "User already exists")
    )
)]
async fn register(
    req: web::Json<RegisterRequest>,
    state: web::Data<AuthState>,
) -> Result<HttpResponse> {
    // Validate password requirements
    if let Err(e) = AuthServiceImpl::validate_password(&req.password) {
        return Ok(HttpResponse::BadRequest().json(serde_json::json!({
            "error": e.message(),
            "code": e.code().to_string()
        })));
    }
    
    // Register user
    match state.auth_service.register(
        req.email.clone(),
        req.password.clone(),
        req.role.as_ref().map(|r| Role::from_string(r)),
    ).await {
        Ok(user) => {
            // Generate tokens
            let access_token = state.auth_service.generate_access_token(&user).await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.message()))?;
            let refresh_token = state.auth_service.generate_refresh_token(&user).await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.message()))?;
            
            // Save refresh token to database
            let token_hash = AuthServiceImpl::hash_token(&refresh_token);
            let expires_at = Utc::now() + Duration::seconds(state.jwt_config.refresh_token_expiry as i64);
            state.auth_service.save_refresh_token(user.id, &token_hash, expires_at).await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.message()))?;
            
            Ok(HttpResponse::Created().json(AuthResponse {
                access_token,
                refresh_token,
                token_type: "Bearer".to_string(),
                expires_in: state.jwt_config.access_token_expiry,
            }))
        }
        Err(e) => {
            let status = match e.code() {
                crate::error::app_error::ErrorCode::AlreadyExists => 
                    actix_web::http::StatusCode::CONFLICT,
                _ => actix_web::http::StatusCode::BAD_REQUEST,
            };
            Ok(HttpResponse::build(status).json(serde_json::json!({
                "error": e.message(),
                "code": e.code().to_string()
            })))
        }
    }
}

/// User login
#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    tag = "Authentication",
    request_body = LoginRequest,
    responses(
        (status = 200, description = "Login successful", body = AuthResponse),
        (status = 401, description = "Invalid credentials")
    )
)]
async fn login(
    req: web::Json<LoginRequest>,
    state: web::Data<AuthState>,
) -> Result<HttpResponse> {
    // Authenticate user
    match state.auth_service.login(req.email.clone(), req.password.clone()).await {
        Ok(user) => {
            // Generate tokens
            let access_token = state.auth_service.generate_access_token(&user).await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.message()))?;
            let refresh_token = state.auth_service.generate_refresh_token(&user).await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.message()))?;
            
            // Save refresh token to database
            let token_hash = AuthServiceImpl::hash_token(&refresh_token);
            let expires_at = Utc::now() + Duration::seconds(state.jwt_config.refresh_token_expiry as i64);
            state.auth_service.save_refresh_token(user.id, &token_hash, expires_at).await
                .map_err(|e| actix_web::error::ErrorInternalServerError(e.message()))?;
            
            Ok(HttpResponse::Ok().json(AuthResponse {
                access_token,
                refresh_token,
                token_type: "Bearer".to_string(),
                expires_in: state.jwt_config.access_token_expiry,
            }))
        }
        Err(e) => {
            Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Invalid credentials",
                "code": "INVALID_CREDENTIALS"
            })))
        }
    }
}

/// Refresh access token
#[utoipa::path(
    post,
    path = "/api/v1/auth/refresh",
    tag = "Authentication",
    request_body = RefreshRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = AuthResponse),
        (status = 401, description = "Invalid refresh token")
    )
)]
async fn refresh(
    req: web::Json<RefreshRequest>,
    state: web::Data<AuthState>,
) -> Result<HttpResponse> {
    // Validate refresh token
    let claims = match validate_refresh_token(&req.refresh_token, &state.jwt_config) {
        Ok(c) => c,
        Err(e) => {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": e.message(),
                "code": e.code().to_string()
            })));
        }
    };
    
    // Check if token exists in database
    let token_hash = AuthServiceImpl::hash_token(&req.refresh_token);
    let stored_token = match state.auth_service.find_refresh_token(&token_hash).await {
        Ok(Some(token)) => token,
        Ok(None) => {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Refresh token not found or revoked",
                "code": "INVALID_TOKEN"
            })));
        }
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.message(),
                "code": "INTERNAL_ERROR"
            })));
        }
    };
    
    // Check if token is expired
    if stored_token.is_expired() {
        return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
            "error": "Refresh token expired",
            "code": "TOKEN_EXPIRED"
        })));
    }
    
    // Get user
    let user_id = match Uuid::parse_str(&claims.sub) {
        Ok(id) => id,
        Err(_) => {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Invalid user ID in token",
                "code": "INVALID_TOKEN"
            })));
        }
    };
    
    let user = match state.auth_service.find_user_by_id(user_id).await {
        Ok(Some(u)) => u,
        Ok(None) => {
            return Ok(HttpResponse::NotFound().json(serde_json::json!({
                "error": "User not found",
                "code": "NOT_FOUND"
            })));
        }
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.message(),
                "code": "INTERNAL_ERROR"
            })));
        }
    };
    
    // Generate new tokens
    let access_token = match state.auth_service.generate_access_token(&user).await {
        Ok(t) => t,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.message(),
                "code": "INTERNAL_ERROR"
            })));
        }
    };
    
    let refresh_token = match state.auth_service.generate_refresh_token(&user).await {
        Ok(t) => t,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.message(),
                "code": "INTERNAL_ERROR"
            })));
        }
    };
    
    // Delete old refresh token (rotation)
    if let Err(e) = state.auth_service.delete_refresh_token(&token_hash).await {
        tracing::warn!("Failed to delete old refresh token: {}", e);
    }
    
    // Save new refresh token
    let new_token_hash = AuthServiceImpl::hash_token(&refresh_token);
    let expires_at = Utc::now() + Duration::seconds(state.jwt_config.refresh_token_expiry as i64);
    if let Err(e) = state.auth_service.save_refresh_token(user.id, &new_token_hash, expires_at).await {
        return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
            "error": e.message(),
            "code": "INTERNAL_ERROR"
        })));
    }
    
    Ok(HttpResponse::Ok().json(AuthResponse {
        access_token,
        refresh_token,
        token_type: "Bearer".to_string(),
        expires_in: state.jwt_config.access_token_expiry,
    }))
}

/// User logout
#[utoipa::path(
    post,
    path = "/api/v1/auth/logout",
    tag = "Authentication",
    security = ("bearerAuth" = []),
    request_body = LogoutRequest,
    responses(
        (status = 200, description = "Logout successful"),
        (status = 401, description = "Not authenticated")
    )
)]
async fn logout(
    req: web::Json<LogoutRequest>,
    http_req: actix_web::HttpRequest,
    state: web::Data<AuthState>,
) -> Result<HttpResponse> {
    // Get user ID from request (set by auth middleware)
    let user_id = match http_req.user_id() {
        Some(id) => id,
        None => {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Not authenticated",
                "code": "UNAUTHORIZED"
            })));
        }
    };
    
    // Invalidate refresh token if provided
    if let Some(refresh_token) = &req.refresh_token {
        // Hash and delete from database
        let token_hash = AuthServiceImpl::hash_token(refresh_token);
        if let Err(e) = state.auth_service.delete_refresh_token(&token_hash).await {
            tracing::warn!("Failed to invalidate refresh token: {}", e);
        }
    }
    
    // Note: In a complete implementation, we might also want to invalidate
    // the access token (using a token blacklist stored in Redis)
    
    Ok(HttpResponse::Ok().json(serde_json::json!({
        "message": "Logged out successfully"
    })))
}

/// Validate refresh token
fn validate_refresh_token(token: &str, config: &JwtConfig) -> Result<RefreshClaims, AppError> {
    use jsonwebtoken::{decode, DecodingKey, TokenData, Validation};
    
    let token_data: TokenData<RefreshClaims> = decode(
        token,
        &DecodingKey::from_secret(config.secret.as_bytes()),
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
