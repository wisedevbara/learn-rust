//! Application Entry Point
//!
//! Main entry point for the Rust Backend Framework application.

#![allow(dead_code)]

use std::net::SocketAddr;
use std::sync::Arc;
use std::num::NonZeroU32;

use actix_web::{web, App, HttpServer, middleware};
use tower::ServiceBuilder;
use tower_http::rate_limit::RequestRateLimitLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod business;
mod config;
mod data;
mod error;
mod middleware as app_middleware;
mod services;

use config::app::AppConfig;
use config::app::JwtConfig;
use data::repositories::user_repository::{UserRepository, InMemoryUserRepository};
use error::app_error::AppError;
use app_middleware::cors::CorsConfig;
use app_middleware::auth::AuthMiddleware;
use app_middleware::security::SecurityHeaders;

/// Shared application state for all endpoints
pub struct AppState {
    pub user_repo: Arc<dyn UserRepository>,
    pub jwt_config: Arc<JwtConfig>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing/logging
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "info,rust_backend_framework=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting Rust Backend Framework...");

    // Load configuration
    let config = AppConfig::load().await.map_err(|e| {
        tracing::error!("Failed to load configuration: {}", e);
        std::io::Error::new(std::io::ErrorKind::InvalidData, e)
    })?;

    let bind_addr = SocketAddr::from(([0, 0, 0, 0], config.server.port));
    tracing::info!("Server binding to {}", bind_addr);

    // Create shared state with in-memory repository
    // TODO: In production, replace with database connection pool (e.g., sqlx::PgPool)
    // Database connections naturally provide shared state across all endpoints
    let user_repo: Arc<dyn UserRepository> = Arc::new(InMemoryUserRepository::new());
    let jwt_config = Arc::new(config.jwt.clone());
    let app_state = AppState {
        user_repo: user_repo.clone(),
        jwt_config: jwt_config.clone(),
    };

    // Start HTTP server
    // Rate limiting: 100 requests per minute per IP
    let rate_limit_layer = RequestRateLimitLayer::new(
        NonZeroU32::new(100).unwrap(),
        std::time::Duration::from_secs(60),
    );
    
    HttpServer::new(move || {
        App::new()
            // Rate limiting middleware (100 requests/minute per IP)
            .wrap(rate_limit_layer.clone())
            // Enable CORS middleware
            .wrap(CorsConfig::new().build())
            // Security headers middleware
            .wrap(SecurityHeaders::new())
            // Enable tracing/logger middleware
            .wrap(middleware::Logger::default())
            // Configure app data - shared state
            .app_data(web::Data::new(app_state.clone()))
            .app_data(web::Data::new(config.clone()))
            // Register routes
            .configure(api::v1::health::configure)
            .service(
                web::scope("/api/v1")
                    // Auth routes are public (no auth middleware)
                    .configure(|cfg: &mut web::ServiceConfig| {
                        api::v1::auth::configure(cfg, &app_state);
                    })
                    // User routes are protected with AuthMiddleware
                    .service(
                        web::scope("/users")
                            .wrap(AuthMiddleware::new(jwt_config.clone()))
                            .configure(|cfg: &mut web::ServiceConfig| {
                                api::v1::users::configure(cfg, &app_state);
                            })
                    ),
            )
            // Error handlers
            .default_service(web::route().to(error::error_handler::not_found))
    })
    .bind(bind_addr)?
    .run()
    .await
}
