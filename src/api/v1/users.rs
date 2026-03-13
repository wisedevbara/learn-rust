//! User Management API Endpoints
//!
//! Contains handlers for user CRUD operations.

use std::sync::Arc;

use actix_web::{web, HttpResponse, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::business::entities::user::Role;
use crate::data::repositories::user_repository::UserRepository;
use crate::error::app_error::AppError;
use crate::middleware::auth::{Claims, RequestClaims};

/// Query parameters for listing users
#[derive(Debug, Deserialize)]
pub struct UserListQuery {
    /// Page number (default: 1)
    #[serde(default = "default_page")]
    pub page: i64,
    /// Items per page (default: 20, max: 100)
    #[serde(default = "default_per_page")]
    pub per_page: i64,
}

fn default_page() -> i64 {
    1
}

fn default_per_page() -> i64 {
    20
}

/// User list response
#[derive(Debug, Serialize)]
pub struct UserListResponse {
    pub users: Vec<UserResponse>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
}

/// User response (no password_hash)
#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<&crate::business::entities::user::User> for UserResponse {
    fn from(user: &crate::business::entities::user::User) -> Self {
        Self {
            id: user.id,
            email: user.email.clone(),
            role: user.role.to_string(),
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}

/// Update user request
#[derive(Debug, Deserialize, Serialize)]
pub struct UpdateUserRequest {
    pub email: Option<String>,
    pub role: Option<String>,
}

/// Application state for user management
pub struct UserState {
    pub user_repo: Arc<dyn UserRepository>,
}

impl UserState {
    pub fn new(user_repo: Arc<dyn UserRepository>) -> Self {
        Self { user_repo }
    }
}

/// Configure user routes
/// Takes shared app_state from main.rs to ensure single repository instance
pub fn configure(cfg: &mut web::ServiceConfig, app_state: &crate::main::AppState) {
    let state = UserState::new(app_state.user_repo.clone());
    
    cfg.app_data(web::Data::new(state))
        .service(
            web::scope("/users")
                .service(list_users)
                .service(get_user)
                .service(update_user)
                .service(delete_user),
        );
}

/// Check if the current user has admin role
fn is_admin(claims: &Claims) -> bool {
    claims.role == "admin"
}

/// Check if the current user can access the target user (admin or same user)
fn can_access_user(claims: &Claims, target_user_id: Uuid) -> bool {
    if is_admin(claims) {
        return true;
    }
    
    if let Ok(user_id) = Uuid::parse_str(&claims.sub) {
        return user_id == target_user_id;
    }
    
    false
}

/// Check if the current user can update the target user
/// - Admin can update any user
/// - User can update their own profile but cannot change role
fn can_update_user(claims: &Claims, target_user_id: Uuid, new_role: &Option<String>) -> bool {
    if is_admin(claims) {
        return true;
    }
    
    // Non-admin trying to change role is not allowed
    if new_role.is_some() {
        return false;
    }
    
    // Check if it's the same user
    if let Ok(user_id) = Uuid::parse_str(&claims.sub) {
        return user_id == target_user_id;
    }
    
    false
}

/// List all users (admin only)
#[utoipa::path(
    get,
    path = "/api/v1/users",
    tag = "Users",
    security = ("bearerAuth" = []),
    params(UserListQuery),
    responses(
        (status = 200, description = "List of users", body = UserListResponse),
        (status = 401, description = "Not authenticated"),
        (status = 403, description = "Not authorized - admin only")
    )
)]
async fn list_users(
    http_req: actix_web::HttpRequest,
    query: web::Query<UserListQuery>,
    state: web::Data<UserState>,
) -> Result<HttpResponse> {
    // Check authentication
    let claims = match http_req.claims() {
        Some(c) => c,
        None => {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Not authenticated",
                "code": "UNAUTHORIZED"
            })));
        }
    };
    
    // Check admin role
    if !is_admin(claims) {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Admin access required",
            "code": "FORBIDDEN"
        })));
    }
    
    // Validate pagination parameters
    let page = if query.page < 1 { 1 } else { query.page };
    let per_page = if query.per_page < 1 { 20 } else { query.per_page.min(100) };
    
    // Fetch paginated users
    match state.user_repo.list_paginated(page, per_page).await {
        Ok((users, total)) => {
            let user_responses: Vec<UserResponse> = users
                .iter()
                .map(UserResponse::from)
                .collect();
            
            Ok(HttpResponse::Ok().json(UserListResponse {
                users: user_responses,
                total,
                page,
                per_page,
            }))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.message(),
                "code": e.code().to_string()
            })))
        }
    }
}

/// Get user by ID
#[utoipa::path(
    get,
    path = "/api/v1/users/{id}",
    tag = "Users",
    security = ("bearerAuth" = []),
    responses(
        (status = 200, description = "User found", body = UserResponse),
        (status = 401, description = "Not authenticated"),
        (status = 403, description = "Not authorized"),
        (status = 404, description = "User not found")
    )
)]
async fn get_user(
    http_req: actix_web::HttpRequest,
    path: web::Path<String>,
    state: web::Data<UserState>,
) -> Result<HttpResponse> {
    // Check authentication
    let claims = match http_req.claims() {
        Some(c) => c,
        None => {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Not authenticated",
                "code": "UNAUTHORIZED"
            })));
        }
    };
    
    // Parse user ID from path
    let user_id = match Uuid::parse_str(&path) {
        Ok(id) => id,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid user ID format",
                "code": "INVALID_INPUT"
            })));
        }
    };
    
    // Check authorization (admin or same user)
    if !can_access_user(claims, user_id) {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Access denied - you can only view your own profile",
            "code": "FORBIDDEN"
        })));
    }
    
    // Fetch user
    match state.user_repo.find_by_id(user_id).await {
        Ok(Some(user)) => {
            Ok(HttpResponse::Ok().json(UserResponse::from(&user)))
        }
        Ok(None) => {
            Ok(HttpResponse::NotFound().json(serde_json::json!({
                "error": "User not found",
                "code": "NOT_FOUND"
            })))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.message(),
                "code": e.code().to_string()
            })))
        }
    }
}

/// Update user
#[utoipa::path(
    put,
    path = "/api/v1/users/{id}",
    tag = "Users",
    security = ("bearerAuth" = []),
    request_body = UpdateUserRequest,
    responses(
        (status = 200, description = "User updated", body = UserResponse),
        (status = 401, description = "Not authenticated"),
        (status = 403, description = "Not authorized"),
        (status = 404, description = "User not found")
    )
)]
async fn update_user(
    http_req: actix_web::HttpRequest,
    path: web::Path<String>,
    req: web::Json<UpdateUserRequest>,
    state: web::Data<UserState>,
) -> Result<HttpResponse> {
    // Check authentication
    let claims = match http_req.claims() {
        Some(c) => c,
        None => {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Not authenticated",
                "code": "UNAUTHORIZED"
            })));
        }
    };
    
    // Parse user ID from path
    let user_id = match Uuid::parse_str(&path) {
        Ok(id) => id,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid user ID format",
                "code": "INVALID_INPUT"
            })));
        }
    };
    
    // Check authorization (admin or same user, with role change restrictions)
    if !can_update_user(claims, user_id, &req.role) {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Access denied - you can only update your own profile",
            "code": "FORBIDDEN"
        })));
    }
    
    // Fetch existing user
    let existing_user = match state.user_repo.find_by_id(user_id).await {
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
                "code": e.code().to_string()
            })));
        }
    };
    
    // Check if email is being changed and if it's already taken
    if let Some(ref new_email) = req.email {
        if new_email != &existing_user.email {
            if let Ok(exists) = state.user_repo.email_exists(new_email).await {
                if exists {
                    return Ok(HttpResponse::Conflict().json(serde_json::json!({
                        "error": "Email already in use",
                        "code": "ALREADY_EXISTS"
                    })));
                }
            }
        }
    }
    
    // Build updated user
    let updated_user = crate::business::entities::user::User {
        id: existing_user.id,
        email: req.email.clone().unwrap_or(existing_user.email),
        password_hash: existing_user.password_hash,
        role: if let Some(ref role_str) = req.role {
            // Only admin can change role
            if is_admin(claims) {
                Role::from_string(role_str)
            } else {
                existing_user.role
            }
        } else {
            existing_user.role
        },
        created_at: existing_user.created_at,
        updated_at: Utc::now(),
    };
    
    // Update user in repository
    match state.user_repo.update(&updated_user).await {
        Ok(user) => {
            Ok(HttpResponse::Ok().json(UserResponse::from(&user)))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.message(),
                "code": e.code().to_string()
            })))
        }
    }
}

/// Delete user (admin only)
#[utoipa::path(
    delete,
    path = "/api/v1/users/{id}",
    tag = "Users",
    security = ("bearerAuth" = []),
    responses(
        (status = 200, description = "User deleted"),
        (status = 401, description = "Not authenticated"),
        (status = 403, description = "Not authorized - admin only"),
        (status = 404, description = "User not found")
    )
)]
async fn delete_user(
    http_req: actix_web::HttpRequest,
    path: web::Path<String>,
    state: web::Data<UserState>,
) -> Result<HttpResponse> {
    // Check authentication
    let claims = match http_req.claims() {
        Some(c) => c,
        None => {
            return Ok(HttpResponse::Unauthorized().json(serde_json::json!({
                "error": "Not authenticated",
                "code": "UNAUTHORIZED"
            })));
        }
    };
    
    // Check admin role
    if !is_admin(claims) {
        return Ok(HttpResponse::Forbidden().json(serde_json::json!({
            "error": "Admin access required",
            "code": "FORBIDDEN"
        })));
    }
    
    // Parse user ID from path
    let user_id = match Uuid::parse_str(&path) {
        Ok(id) => id,
        Err(_) => {
            return Ok(HttpResponse::BadRequest().json(serde_json::json!({
                "error": "Invalid user ID format",
                "code": "INVALID_INPUT"
            })));
        }
    };
    
    // Check if user exists first
    let exists = match state.user_repo.find_by_id(user_id).await {
        Ok(Some(_)) => true,
        Ok(None) => false,
        Err(e) => {
            return Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.message(),
                "code": e.code().to_string()
            })));
        }
    };
    
    if !exists {
        return Ok(HttpResponse::NotFound().json(serde_json::json!({
            "error": "User not found",
            "code": "NOT_FOUND"
        })));
    }
    
    // Delete user
    match state.user_repo.delete(user_id).await {
        Ok(()) => {
            Ok(HttpResponse::Ok().json(serde_json::json!({
                "message": "User deleted successfully"
            })))
        }
        Err(e) => {
            Ok(HttpResponse::InternalServerError().json(serde_json::json!({
                "error": e.message(),
                "code": e.code().to_string()
            })))
        }
    }
}
