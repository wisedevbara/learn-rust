//! Application Entry Point
//!
//! Main entry point for the Rust Backend Framework application.

#![allow(dead_code)]

use std::net::SocketAddr;
use std::sync::Arc;

use actix_web::{web, App, HttpServer, middleware};
use actix_web::middleware::DefaultHeaders;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod business;
mod config;
mod data;
mod error;
mod services;

use rust_backend_framework::config::app::AppConfig;
use rust_backend_framework::AppState;
use rust_backend_framework::data::repositories::user_repository::{UserRepository, InMemoryUserRepository};

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
    let user_repo: Arc<dyn UserRepository> = Arc::new(InMemoryUserRepository::new());
    let jwt_config = Arc::new(config.jwt.clone());
    let app_state = AppState {
        user_repo,
        jwt_config,
    };

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            // Security headers using built-in middleware
            .wrap(DefaultHeaders::new()
                .add(("Strict-Transport-Security", "max-age=31536000; includeSubDomains"))
                .add(("X-Frame-Options", "DENY"))
                .add(("X-Content-Type-Options", "nosniff"))
                .add(("Referrer-Policy", "strict-origin-when-cross-origin"))
            )
            // Enable tracing/logger middleware
            .wrap(middleware::Logger::default())
            // Configure app data - shared state (clone for each worker)
            .app_data(web::Data::new(app_state.clone()))
            .app_data(web::Data::new(config.clone()))
            // Register routes
            .configure(api::v1::health::configure)
            .service(
                web::scope("/api/v1")
                    // Auth routes
                    .configure(|cfg: &mut web::ServiceConfig| {
                        api::v1::auth::configure(cfg, &app_state);
                    })
                    // User routes
                    .service(
                        web::scope("/users")
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
