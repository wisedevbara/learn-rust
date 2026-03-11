# Development Guide

## Rust Backend Framework

This guide provides comprehensive instructions for developers working on the Rust Backend Framework project.

---

## Table of Contents

1. [Prerequisites](#1-prerequisites)
2. [Environment Setup](#2-environment-setup)
3. [Project Structure](#3-project-structure)
4. [Coding Standards](#4-coding-standards)
5. [Development Workflow](#5-development-workflow)
6. [Testing](#6-testing)
7. [Debugging](#7-debugging)
8. [Common Tasks](#8-common-tasks)
9. [FAQ](#9-faq)

---

## 1. Prerequisites

### 1.1 Required Software

| Software | Version | Purpose |
|----------|---------|---------|
| Rust | 1.94.0 | Programming language |
| Docker | Latest | Container runtime |
| Docker Compose | Latest | Service orchestration |
| PostgreSQL Client | 17.x | Database client |
| Git | Latest | Version control |

### 1.2 Installing Rust

```bash
# Install Rust (all platforms)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install specific version
rustup install 1.94.0
rustup default 1.94.0

# Verify installation
rustc --version
cargo --version
```

### 1.3 Installing Additional Tools

```bash
# Install sqlx-cli for database operations
cargo install sqlx-cli

# Install cargo-watch for hot reload
cargo install cargo-watch

# Install cargo-audit for security
cargo install cargo-audit

# Install cargo-tarpaulin for coverage
cargo install cargo-tarpaulin
```

---

## 2. Environment Setup

### 2.1 Clone Repository

```bash
git clone https://github.com/your-repo/rust-backend.git
cd rust-backend
```

### 2.2 Environment Variables

```bash
# Copy example environment file
cp .env.example .env

# Edit configuration
vim .env
```

Required variables:
```bash
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/app
REDIS_URL=redis://localhost:6379
JWT_SECRET=your-super-secret-key-minimum-32-characters
```

### 2.3 Start Services

```bash
# Start all services with Docker
docker-compose up -d

# Verify services are running
docker-compose ps

# View logs
docker-compose logs -f
```

### 2.4 Database Setup

```bash
# Create database
cargo sqlx database create

# Run migrations
cargo sqlx migrate run

# Verify database
cargo sqlx database reset
```

### 2.5 First Run

```bash
# Run application
cargo run

# Access at http://localhost:8080
# Health check: curl http://localhost:8080/health
```

---

## 3. Project Structure

### 3.1 Directory Overview

```
src/
├── api/              # API Layer - HTTP handlers
├── business/         # Business Layer - Logic & entities
├── data/             # Data Layer - Repositories
├── services/         # External services (Redis, Email)
├── middleware/       # Cross-cutting concerns
├── config/           # Configuration
├── error/            # Error handling
├── lib.rs
└── main.rs
```

### 3.2 Layer Responsibilities

| Layer | Directory | Purpose |
|-------|-----------|---------|
| API | `src/api/` | HTTP handlers, DTOs, routes |
| Business | `src/business/` | Domain logic, services |
| Data | `src/data/` | Database operations |
| Services | `src/services/` | External integrations |
| Middleware | `src/middleware/` | Auth, logging, etc. |

---

## 4. Coding Standards

### 4.1 Code Formatting

```bash
# Format code
cargo fmt

# Check formatting
cargo fmt --check
```

### 4.2 Linting

```bash
# Run clippy
cargo clippy

# Fix warnings
cargo clippy --fix --allow-dirty
```

### 4.3 Line Length

- Maximum line length: **100 characters**
- Exception: URLs, long strings (keep as-is)

### 4.4 Documentation

```rust
/// Register a new user.
///
/// # Arguments
/// * `request` - Registration request containing email and password
///
/// # Returns
/// * `Ok(User)` - Created user
/// * `Err(AppError)` - Error during registration
///
/// # Example
/// ```rust
/// let user = auth_service.register(request).await?;
/// ```
pub async fn register(&self, request: RegisterRequest) -> Result<User, AppError>
```

### 4.5 Naming Conventions

| Type | Convention | Example |
|------|------------|---------|
| Variables | snake_case | `user_id`, `created_at` |
| Functions | snake_case | `get_user_by_id` |
| Types/Structs | PascalCase | `User`, `AuthService` |
| Constants | SCREAMING_SNAKE_CASE | `MAX_CONNECTIONS` |
| Modules | snake_case | `user_repository` |

### 4.6 Error Handling

```rust
// Use thiserror for custom errors
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("User not found: {0}")]
    NotFound(String),
    
    #[error("Authentication failed: {0}")]
    Unauthorized(String),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
}

// Return Result<T, AppError>
pub async fn get_user(&self, id: Uuid) -> Result<User, AppError> {
    // ...
}
```

---

## 5. Development Workflow

### 5.1 Branching Strategy

```
main          ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ►
              │                 ▲
              │                 │
develop       ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ►
              │       ▲
              │       │
feature/xxx   ─ ─ ─ ─ ─
fix/xxx       ─ ─ ─ ─ ─
refactor/xxx  ─ ─ ─ ─ ─
```

### 5.2 Creating a Feature

```bash
# 1. Update develop branch
git checkout develop
git pull origin develop

# 2. Create feature branch
git checkout -b feature/user-management

# 3. Make changes
# ... implement feature ...

# 4. Commit changes
git add .
git commit -m "feat: add user management CRUD operations"

# 5. Push to remote
git push origin feature/user-management

# 6. Create pull request
# (via GitHub UI)
```

### 5.3 Commit Message Format

```
<type>(<scope>): <description>

[optional body]

[optional footer]
```

**Types:**
| Type | Description |
|------|-------------|
| feat | New feature |
| fix | Bug fix |
| docs | Documentation |
| style | Formatting |
| refactor | Code refactoring |
| test | Testing |
| chore | Maintenance |

**Examples:**
```
feat(auth): add user registration endpoint

fix(database): resolve connection pool timeout

docs(api): update endpoint documentation
```

### 5.4 Code Review

Before creating PR:

```bash
# Run tests
cargo test

# Run clippy
cargo clippy

# Format code
cargo fmt

# Check security
cargo audit
```

---

## 6. Testing

### 6.1 Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run with coverage
cargo tarpaulin --all-features
```

### 6.2 Writing Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_user_creation() {
        // Arrange
        let email = "test@example.com";
        
        // Act
        let user = User::new(email, "password");
        
        // Assert
        assert_eq!(user.email, email);
    }
}
```

### 6.3 Writing Integration Tests

```rust
#[cfg(test)]
mod integration {
    use actix_web::test;
    
    #[actix_web::test]
    async fn test_health_check() {
        let app = test::init_service(App::new()).await;
        let req = test::TestRequest::get()
            .uri("/health")
            .to_request();
        
        let resp = test::call_service(&app, req).await;
        assert!(resp.status().is_success());
    }
}
```

---

## 7. Debugging

### 7.1 Logging

```rust
use tracing::{info, warn, error};

fn process_request(user_id: Uuid) {
    info!("Processing request for user: {}", user_id);
    
    if let Err(e) = do_something() {
        error!("Error processing request: {}", e);
    }
}
```

### 7.2 Environment Variables for Debugging

```bash
# Enable debug logging
RUST_LOG=debug cargo run

# Trace SQL queries
DATABASE_URL=postgresql://... cargo run

# Enable SQLx logging
SQLX_LOG=debug cargo run
```

### 7.3 Docker Debugging

```bash
# View container logs
docker-compose logs -f app

# Access container shell
docker-compose exec app sh

# Check container status
docker-compose ps

# Restart service
docker-compose restart app
```

---

## 8. Common Tasks

### 8.1 Creating a New Endpoint

1. **Add route in `src/api/v1/mod.rs`:**
```rust
pub fn configure(cfg: &mut ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .service(auth::scope())
            .service(users::scope())
    );
}
```

2. **Create handler in `src/api/v1/users.rs`:**
```rust
#[utoipa::path(
    get,
    path = "/users/{id}",
    responses(
        (status = 200, body = User),
        (status = 404, body = ErrorResponse)
    )
)]
pub async fn get_user(
    web::Path(id): web::Path<Uuid>,
    State(state): State<AppState>,
) -> Result<HttpResponse, AppError> {
    let user = state.user_service.get_by_id(id).await?;
    Ok(HttpResponse::Ok().json(user))
}
```

### 8.2 Adding a Database Model

1. **Define in `src/data/models/user_model.rs`:**
```rust
#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct UserModel {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

2. **Add repository method:**
```rust
pub async fn find_by_id(&self, id: Uuid) -> Result<Option<UserModel>, AppError> {
    Ok(sqlx::query_as::<_, UserModel>(
        "SELECT * FROM users WHERE id = $1"
    )
    .bind(id)
    .fetch_optional(&self.pool)
    .await?)
}
```

### 8.3 Running Migrations

```bash
# Create new migration
cargo sqlx migrate add create_users_table

# Run migrations
cargo sqlx migrate run

# Revert last migration
cargo sqlx migrate revert

# Reset database (drop all, re-run)
cargo sqlx database reset
```

---

## 9. FAQ

### Q: How do I add a new environment variable?

1. Add to `.env.example`
2. Add to `src/config/mod.rs`
3. Document in README.md

### Q: How do I run a single test?

```bash
cargo test test_name
```

### Q: How do I see SQL queries?

```bash
RUST_LOG=sqlx=debug cargo run
```

### Q: How do I reset the database?

```bash
cargo sqlx database reset
```

### Q: How do I add a new dependency?

1. Edit `Cargo.toml`
2. Run `cargo update`
3. Commit changes

---

## Quick Reference

### Essential Commands

```bash
# Development
cargo run                    # Run app
cargo watch -x run          # Hot reload
cargo test                   # Run tests

# Code quality
cargo fmt                    # Format
cargo clippy                 # Lint
cargo audit                 # Security

# Database
cargo sqlx migrate run      # Run migrations
cargo sqlx migrate revert   # Revert

# Docker
docker-compose up -d        # Start services
docker-compose logs -f      # View logs
```

---

**Last Updated: 2026-03-11**
