# Architecture Documentation

## Overview

This document describes the architectural design of the Rust backend framework, providing module boundaries, layer separation, and system design principles based on the project requirements defined in PROJECT.md.

---

## 1. System Architecture

### 1.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                         Client Requests                         │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                         nginx (Reverse Proxy)                   │
│                    Port: 80/443 (HTTP/HTTPS)                    │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                    Actix-web (Application Server)               │
│                         Port: 8080                              │
│  ┌─────────────────────────────────────────────────────────┐   │
│  │                    Middleware Stack                       │   │
│  │  Tracing → Auth → CORS → Rate Limit → Request Logging  │   │
│  └─────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────┘
                                │
            ┌───────────────────┼───────────────────┐
            ▼                   ▼                   ▼
    ┌───────────────┐   ┌───────────────┐   ┌───────────────┐
    │  API Layer    │   │    Business   │   │   Services    │
    │   (Handlers)  │◄──│     Layer     │◄──│   (External)  │
    └───────────────┘   └───────────────┘   └───────────────┘
            │                   │                   │
            └───────────────────┼───────────────────┘
                                ▼
    ┌───────────────────────────────────────────────────────────────┐
    │                        Data Layer                              │
    │         (Repositories, Models, Migrations)                    │
    └───────────────────────────────────────────────────────────────┘
                                │
            ┌───────────────────┼───────────────────┐
            ▼                   ▼                   ▼
    ┌───────────────┐   ┌───────────────┐   ┌───────────────┐
    │  PostgreSQL   │   │     Redis     │   │    Metrics    │
    │   (Primary)   │   │    (Cache)    │   │  (Prometheus) │
    └───────────────┘   └───────────────┘   └───────────────┘
```

### 1.2 Technology Stack Summary

| Component | Technology | Version |
|-----------|------------|---------|
| Web Framework | Actix-web | 4.x |
| ORM (Primary) | sqlx | 0.8 |
| ORM (Optional) | Diesel | 2.2 |
| Database | PostgreSQL | 17.8 |
| Cache | Redis | 8.6 |
| Web Server | nginx | 1.29.5 |
| Async Runtime | tokio | 1.42 |
| Authentication | JWT (jsonwebtoken) | 9.3 |
| Password Hashing | Argon2 | 0.5 |
| API Documentation | utoipa | 4.2 |
| Metrics | prometheus-client | 0.22 |
| Logging | tracing | 0.1 |

---

## 2. Layer Architecture

### 2.1 Layer Definitions

The application follows a strict layered architecture with unidirectional dependencies:

```
┌────────────────────────────────────────────┐
│              API Layer (src/api/)          │
│  - HTTP Handlers                           │
│  - Request/Response Types                  │
│  - Route Definitions                       │
│  Depends on: Business Layer                │
└────────────────────────────────────────────┘
                    │
                    ▼
┌────────────────────────────────────────────┐
│         Business Layer (src/business/)     │
│  - Domain Entities                         │
│  - Business Logic                          │
│  - Service Implementations                 │
│  Depends on: Data Layer, Services Layer    │
└────────────────────────────────────────────┘
                    │
                    ▼
┌────────────────────────────────────────────┐
│           Data Layer (src/data/)           │
│  - Repository Implementations              │
│  - Database Models                         │
│  - Migration Scripts                       │
│  Depends on: External Databases only       │
└────────────────────────────────────────────┘
```

### 2.2 Layer Responsibilities

#### API Layer (`src/api/`)
- **Responsibility**: Handle HTTP requests/responses, input validation, authentication
- **Dependencies**: Business layer only
- **Key Components**:
  - Route handlers (endpoints)
  - Request DTOs (Data Transfer Objects)
  - Response formatters
  - OpenAPI annotations (utoipa)

#### Business Layer (`src/business/`)
- **Responsibility**: Execute core business logic, enforce business rules
- **Dependencies**: Data layer, Services layer
- **Key Components**:
  - Domain entities (User, Auth, etc.)
  - Business services
  - Validation logic

#### Data Layer (`src/data/`)
- **Responsibility**: Database operations, data persistence
- **Dependencies**: External systems only (PostgreSQL, Redis)
- **Key Components**:
  - Repository traits and implementations
  - Database models
  - SQL migrations

#### Services Layer (`src/services/`)
- **Responsibility**: External service integrations
- **Dependencies**: External APIs, Cache systems
- **Key Components**:
  - Cache service (Redis)
  - Email service (future)
  - External API clients

#### Middleware Layer (`src/middleware/`)
- **Responsibility**: Cross-cutting concerns
- **Dependencies**: None (applied at API boundary)
- **Key Components**:
  - Authentication middleware
  - Authorization middleware
  - Logging middleware
  - CORS middleware
  - Rate limiting middleware
  - Tracing middleware

---

## 3. Module Structure

### 3.1 Source Code Modules

```
src/
├── api/                          # API Layer
│   ├── mod.rs                    # Module exports
│   ├── v1/                       # API v1 routes
│   │   ├── mod.rs
│   │   ├── auth.rs               # /auth endpoints
│   │   ├── users.rs              # /users endpoints
│   │   └── mod.rs
│   └── mod.rs
│
├── business/                     # Business Layer
│   ├── mod.rs
│   ├── entities/                 # Domain entities
│   │   ├── mod.rs
│   │   ├── user.rs               # User entity
│   │   ├── auth.rs               # Auth entities
│   │   └── mod.rs
│   ├── services/                 # Business services
│   │   ├── mod.rs
│   │   ├── auth_service.rs       # Authentication service
│   │   ├── user_service.rs       # User management service
│   │   └── mod.rs
│   └── mod.rs
│
├── data/                         # Data Layer
│   ├── mod.rs
│   ├── repositories/             # Repository implementations
│   │   ├── mod.rs
│   │   ├── user_repository.rs    # User data access
│   │   └── mod.rs
│   ├── models/                   # Database models
│   │   ├── mod.rs
│   │   ├── user_model.rs
│   │   └── mod.rs
│   ├── migrations/              # SQL migrations
│   │   ├── 001_initial_schema.sql
│   │   └── mod.rs
│   └── mod.rs
│
├── services/                     # External Services Layer
│   ├── mod.rs
│   ├── cache/                   # Redis cache service
│   │   ├── mod.rs
│   │   ├── cache_service.rs
│   │   └── mod.rs
│   ├── email/                   # Email service (future)
│   │   ├── mod.rs
│   │   └── mod.rs
│   └── mod.rs
│
├── middleware/                   # Middleware Layer
│   ├── mod.rs
│   ├── auth.rs                  # JWT authentication
│   ├── logging.rs               # Request logging
│   ├── tracing.rs               # Distributed tracing
│   ├── cors.rs                  # CORS handling
│   ├── rate_limit.rs            # Rate limiting
│   └── mod.rs
│
├── config/                       # Configuration
│   ├── mod.rs
│   ├── app.rs                   # App configuration
│   ├── database.rs              # Database config
│   ├── cache.rs                 # Cache config
│   └── mod.rs
│
├── error/                        # Error Handling
│   ├── mod.rs
│   ├── app_error.rs             # Custom error types
│   ├── error_handler.rs         # Error handling logic
│   └── mod.rs
│
├── lib.rs                        # Library root
└── main.rs                      # Application entry point
```

### 3.2 Module Dependencies Graph

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│    api/     │────►│  business/  │────►│   data/     │
└─────────────┘     └─────────────┘     └─────────────┘
       │                   │                   │
       │                   ▼                   │
       │            ┌─────────────┐            │
       └──────────►│ middleware/  │◄───────────┘
                   └─────────────┘
                          │
                   ┌──────┴──────┐
                   ▼             ▼
            ┌───────────┐ ┌───────────┐
            │  services/│ │  config/  │
            └───────────┘ └───────────┘
                          │
                   ┌──────┴──────┐
                   ▼             ▼
            ┌───────────┐ ┌───────────┐
            │   error/  │ │  External │
            └───────────┘ │ (DB/Redis)│
                         └───────────┘
```

---

## 4. Key Design Patterns

### 4.1 Repository Pattern

```rust
// Repository trait (in data layer)
pub trait UserRepository {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, AppError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, AppError>;
    async fn create(&self, user: CreateUser) -> Result<User, AppError>;
    async fn update(&self, id: Uuid, user: UpdateUser) -> Result<User, AppError>;
    async fn delete(&self, id: Uuid) -> Result<(), AppError>;
}
```

### 4.2 Service Layer Pattern

```rust
// Service trait (in business layer)
pub trait AuthService {
    async fn register(&self, request: RegisterRequest) -> Result<AuthResponse, AppError>;
    async fn login(&self, request: LoginRequest) -> Result<AuthResponse, AppError>;
    async fn refresh_token(&self, token: &str) -> Result<AuthResponse, AppError>;
    async fn validate_token(&self, token: &str) -> Result<Claims, AppError>;
}
```

### 4.3 Error Handling Pattern

```rust
// Custom error type using thiserror
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Authentication failed: {0}")]
    Unauthorized(String),
    
    #[error("Resource not found: {0}")]
    NotFound(String),
    
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Validation error: {0}")]
    Validation(#[from] validator::ValidationErrors),
}

// Error response for API
impl Into<ApiError> for AppError {
    fn into(self) -> ApiError {
        ApiError {
            code: self.status_code(),
            message: self.to_string(),
        }
    }
}
```

### 4.4 Middleware Chain Pattern

```rust
// Middleware composition in main.rs
App::new()
    .wrap(TracingMiddleware::new())
    .wrap(CorsMiddleware::new())
    .wrap(RateLimitMiddleware::new())
    .wrap(LoggingMiddleware::new())
    .service(api::v1::auth::scope())
    .service(api::v1::users::scope())
```

---

## 5. Configuration Management

### 5.1 Configuration Structure

```rust
// config/app.rs
pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub cache: CacheConfig,
    pub security: SecurityConfig,
    pub logging: LoggingConfig,
}

pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub workers: usize,
}

pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

pub struct CacheConfig {
    pub url: String,
    pub ttl_seconds: u64,
}

pub struct SecurityConfig {
    pub jwt_secret: String,
    pub jwt_expiry_minutes: u64,
    pub refresh_token_expiry_days: u64,
    pub argon2: Argon2Config,
}
```

### 5.2 Environment-Based Configuration

| Environment | Database | Redis | Log Level |
|-------------|----------|-------|-----------|
| Development | localhost:5432 | localhost:6379 | debug |
| Staging | staging-db:5432 | staging-redis:6379 | info |
| Production | prod-db:5432 | prod-redis:6379 | warn |

---

## 6. Security Architecture

### 6.1 Authentication Flow

```
┌──────────┐     ┌──────────────┐     ┌────────────────┐     ┌─────────────┐
│  Client  │────►│   nginx      │────►│  Actix-web     │────►│   Auth      │
│          │     │  (TLS term)  │     │  (Validate)    │     │  Middleware │
└──────────┘     └──────────────┘     └────────────────┘     └─────────────┘
                                                                     │
       ┌──────────────────────────────────────────────────────────────┘
       │
       ▼
┌──────────────┐     ┌────────────────┐
│   Business   │◄────│   Generate     │
│   Handler    │     │   JWT Token    │
└──────────────┘     └────────────────┘
```

### 6.2 Authorization Model (RBAC)

```
Roles:
├── admin      # Full system access
├── user       # Standard user access  
└── guest      # Limited read-only access

Permission Matrix:
| Endpoint          | admin | user | guest |
|-------------------|-------|------|-------|
| POST /auth/login  |   ✓   |  ✓   |   ✓   |
| POST /auth/register|  ✓   |  ✓   |   ✓   |
| GET  /users       |   ✓   |  ✓   |   ✗   |
| POST /users       |   ✓   |  ✗   |   ✗   |
| DELETE /users/:id |   ✓   |  ✗   |   ✗   |
```

### 6.3 Security Headers

All responses include:
- `Strict-Transport-Security: max-age=31536000; includeSubDomains`
- `Content-Security-Policy: default-src 'self'`
- `X-Frame-Options: DENY`
- `X-Content-Type-Options: nosniff`
- `Referrer-Policy: strict-origin-when-cross-origin`

---

## 7. Observability

### 7.1 Logging Strategy

- **Framework**: tracing + tracing-subscriber
- **Format**: JSON for production, human-readable for development
- **Log Levels**:
  - ERROR: Application errors requiring immediate attention
  - WARN: Potential issues that should be investigated
  - INFO: Key application events (startup, shutdown, major operations)
  - DEBUG: Detailed debugging information

### 7.2 Metrics Collection

- **Metrics Library**: metrics + prometheus-client
- **Key Metrics**:
  - HTTP request duration (histogram)
  - HTTP request count (counter)
  - Active connections (gauge)
  - Database query duration (histogram)
  - Cache hit/miss ratio (counter)

### 7.3 Health Checks

```
GET /health
Response: 200 OK
{
    "status": "healthy",
    "checks": {
        "database": "ok",
        "cache": "ok"
    }
}
```

---

## 8. Database Design

### 8.1 Initial Schema

```sql
-- users table
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'user',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- refresh_tokens table
CREATE TABLE refresh_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create indexes
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_refresh_tokens_user_id ON refresh_tokens(user_id);
CREATE INDEX idx_refresh_tokens_token_hash ON refresh_tokens(token_hash);
```

---

## 9. API Design

### 9.1 API Versioning

- Base URL: `/api/v1`
- Version header: `Accept: application/vnd.app.v1+json`

### 9.2 Endpoints Summary

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| POST | /api/v1/auth/register | Register new user | No |
| POST | /api/v1/auth/login | User login | No |
| POST | /api/v1/auth/refresh | Refresh access token | No |
| GET | /api/v1/users | List users | Yes (admin) |
| GET | /api/v1/users/:id | Get user by ID | Yes (admin) |
| PUT | /api/v1/users/:id | Update user | Yes (admin) |
| DELETE | /api/v1/users/:id | Delete user | Yes (admin) |
| GET | /health | Health check | No |

---

## 10. Deployment Architecture

### 10.1 Production Deployment

```
                          ┌─────────────────┐
                          │   DNS / CDN     │
                          └────────┬────────┘
                                   │
                          ┌────────▼────────┐
                          │  nginx (TLS)    │
                          │  Load Balancer  │
                          └────────┬────────┘
                                   │
                    ┌──────────────┼──────────────┐
                    │              │              │
             ┌──────▼─────┐  ┌──────▼─────┐  ┌──────▼─────┐
             │  App Pod 1 │  │  App Pod 2 │  │  App Pod 3 │
             │ (actix-web)│  │(actix-web) │  │(actix-web) │
             └──────┬─────┘  └──────┬─────┘  └──────┬─────┘
                    │              │              │
                    └──────────────┼──────────────┘
                                   │
                    ┌──────────────┼──────────────┐
                    │              │              │
               ┌────▼────┐    ┌────▼────┐    ┌────▼────┐
               │Primary DB│    │  Redis  │    │Prometheus│
               │(Postgres)│    │ (Cache) │    │(Metrics) │
               └─────────┘    └─────────┘    └─────────┘
```

### 10.2 Docker Compose (Development)

```yaml
version: '3.8'
services:
  app:
    build: .
    ports:
      - "8080:8080"
    environment:
      - DATABASE_URL=postgresql://postgres:postgres@db:5432/app
      - REDIS_URL=redis://redis:6379
    depends_on:
      db:
        condition: service_healthy
      redis:
        condition: service_started
  
  db:
    image: postgres:17.8
    environment:
      POSTGRES_USER: postgres
      POSTGRES_PASSWORD: postgres
      POSTGRES_DB: app
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U postgres"]
      interval: 5s
      timeout: 5s
      retries: 5
  
  redis:
    image: redis:8.6
    command: redis-server --appendonly yes
  
  nginx:
    image: nginx:1.29.5
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
    depends_on:
      - app
```

---

## 11. Development Guidelines

### 11.1 Code Organization Rules

1. **Layer Dependency Rule**: Only import from layers below your current layer
2. **Module Visibility**: Use `pub(crate)` for internal module exports
3. **Error Propagation**: Use `?` operator with custom error types
4. **Async Conventions**: All database operations must be async

### 11.2 Naming Conventions

| Component | Convention | Example |
|-----------|------------|---------|
| Modules | snake_case | `user_repository` |
| Structs | PascalCase | `UserRepository` |
| Traits | PascalCase | `UserRepositoryTrait` |
| Functions | snake_case | `find_by_email` |
| Constants | SCREAMING_SNAKE_CASE | `MAX_CONNECTIONS` |

### 11.3 Testing Strategy

- **Unit Tests**: Test individual functions and methods in isolation
- **Integration Tests**: Test API endpoints with real HTTP requests
- **Property-Based Tests**: Use proptest for input fuzzing

---

*Document Version: 1.0*
*Last Updated: 2026-03-11*
