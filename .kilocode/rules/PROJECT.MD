# PROJECT.MD

## Deskripsi

Project ini untuk pengembangan framework rust untuk backend melalui docker. akan ada beberapa fitur yang akan ditambahkan seperti:

- Authentication
- Authorization
- Database
- Caching
- Testing
- Deployment

## Teknologi Utama

### Web Framework
- **Actix-web** (4.x) - Primary web framework untuk HTTP server

### Database & ORM
- **sqlx** (sqlx = { version = "0.8", default-features = false, features = ["runtime-tokio", "tls-rustls", "postgres", "chrono", "uuid", "json"] }) - Primary ORM (async-first, type-safe query building)
- Diesel (diesel = { version = "2.2", features = ["postgres", "r2d2", "chrono"] }) - Optional for complex queries
- PostgreSQL 17.8

### Cache & Session
- Redis 8.6 (redis = { version = "0.27", features = ["tokio-comp", "connection-manager", "rustls-tls"] })

### Web Server
- nginx 1.29.5

### Async Runtime
- tokio (tokio = { version = "1.42", features = ["full"] })

## Library Spesifik

### Logging & Tracing
- tracing (tracing = "0.1")
- tracing-subscriber (tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] })
- tracing-appender (tracing-appender = "0.2")

### Error Handling
- anyhow (anyhow = "1.0")
- thiserror (thiserror = "2.0")

### Serialization
- serde (serde = "1.0", features = ["derive"])
- serde_json (serde_json = "1.0")
- serde_urlencoded (serde_urlencoded = "0.7")

### Validation
- validator (validator = { version = "0.18", features = ["derive"] })

### Async Utilities
- futures (futures = "0.3")
- async-trait (async-trait = "0.1")

### Date/Time
- chrono (chrono = { version = "0.4", features = ["serde"] })

### UUID
- uuid (uuid = { version = "1.11", features = ["v4", "serde"] })

### Password & Crypto
- argon2 (argon2 = "0.5")
- ring (ring = "0.17")
- rustls (rustls = "0.23")
- base64 (base64 = "0.22")

### HTTP Client
- reqwest (reqwest = { version = "0.12", features = ["json", "rustls-tls", "cookies"] })

### API Documentation
- utoipa (utoipa = "4.2") - OpenAPI/Swagger documentation
- utoipa-swagger-ui (utoipa-swagger-ui = "7.1") - Swagger UI integration

### Metrics & Monitoring
- metrics (metrics = "0.22") - Metrics abstraction
- prometheus-client (prometheus-client = "0.22") - Prometheus exporter

### Middleware
- tower (tower = "0.5")
- tower-http (tower-http = { version = "0.6", features = ["cors", "trace", "compress", "敏感信息过滤"] })

### Image Processing
- image (image = "0.25")
- oxipng (oxipng = "9.0")

### Compression
- flate2 (flate2 = "1.0")
- zstd (zstd = "0.13")

### System Utilities
- once_cell (once_cell = "1.20")
- lazy_static (lazy_static = "1.5")
- parking_lot (parking_lot = "0.12")

### Build Tools
- cargo (bundled with Rust)
- cargo-watch (cargo-watch = "8.5")
- cargo-audit (cargo-audit = "0.20")
- cargo-clippy (included in rustup)

## Testing Library

### Unit & Integration Testing
- pretty_assertions (pretty_assertions = "1.4")
- proptest (proptest = "1.5")
- trybuild (trybuild = "1.0")

### Mocking
- mockall (mockall = "0.13")
- mockall_double (mockall_double = "0.3")

### Property-Based Testing
- quickcheck (quickcheck = "1.0")

### Database Testing
- testcontainers (testcontainers = "0.23")

### HTTP Testing
- wiremock (wiremock = "0.6")

## Dev Tools

### Code Quality
- rustfmt (included in rustup)
- clippy (included in rustup)
- cargo-udeps (cargo-udeps = "0.1")
- cargo-tarpaulin (cargo-tarpaulin = "0.27")

### Documentation
- cargo-doc (included in rustup)
- mdbook (mdbook = "0.4")

### Security
- cargo-sec-audit (cargo-audit = "0.20")
- cargo-deny (cargo-deny = "0.15")

## untuk pengembangan project ini pastikan disediakan :

- Docker
- Docker Compose

## Versi yang digunakan (wajib versi ini) :
- rust:1.94.0
- nginx:1.29.5
- redis:8.6
- postgres:17.8

## External Services

### Monitoring
- Prometheus (prom/prometheus:v3.5.1) - Metrics collection and storage

## Project Structure

```
src/
├── api/                    # HTTP handlers and route definitions
│   ├── mod.rs
│   ├── v1/                 # API v1 endpoints
│   │   ├── mod.rs
│   │   ├── auth.rs         # Authentication endpoints
│   │   ├── users.rs        # User management endpoints
│   │   └── mod.rs
│   └── mod.rs
├── business/               # Core business logic and domain models
│   ├── mod.rs
│   ├── entities/           # Domain entities
│   │   ├── mod.rs
│   │   ├── user.rs
│   │   └── mod.rs
│   ├── services/           # Business services
│   │   ├── mod.rs
│   │   ├── auth_service.rs
│   │   └── mod.rs
│   └── mod.rs
├── data/                   # Database access layer
│   ├── mod.rs
│   ├── repositories/       # Data repositories
│   │   ├── mod.rs
│   │   ├── user_repository.rs
│   │   └── mod.rs
│   ├── models/             # Database models
│   │   ├── mod.rs
│   │   └── mod.rs
│   ├── migrations/         # SQL migrations
│   │   └── mod.rs
│   └── mod.rs
├── services/               # External service integrations
│   ├── mod.rs
│   ├── cache/              # Redis cache service
│   │   ├── mod.rs
│   │   └── mod.rs
│   ├── email/              # Email service
│   │   ├── mod.rs
│   │   └── mod.rs
│   └── mod.rs
├── middleware/              # Cross-cutting concerns
│   ├── mod.rs
│   ├── auth.rs             # Authentication middleware
│   ├── logging.rs          # Logging middleware
│   ├── tracing.rs          # Tracing middleware
│   ├── cors.rs             # CORS middleware
│   └── mod.rs
├── config/                 # Configuration management
│   ├── mod.rs
│   ├── app.rs              # Application configuration
│   ├── database.rs         # Database configuration
│   ├── cache.rs            # Cache configuration
│   └── mod.rs
├── error/                  # Centralized error handling
│   ├── mod.rs
│   ├── app_error.rs        # Application error types
│   ├── error_handler.rs    # Error handler
│   └── mod.rs
├── lib.rs                  # Library root
└── main.rs                 # Application entry point

tests/
├── integration/            # Integration tests
│   ├── mod.rs
│   ├── auth_test.rs
│   ├── mod.rs
│   └── mod.rs
├── unit/                   # Unit tests
│   ├── mod.rs
│   └── mod.rs
└── mod.rs

migrations/                 # Database migrations (SQL)
├── 001_initial_schema.sql
└── mod.rs

scripts/                    # Build and deployment scripts
├── build.sh
├── deploy.sh
└── mod.rs

docker/
├── Dockerfile              # Multi-stage build
├── Dockerfile.dev         # Development container
├── nginx.conf              # Nginx configuration
└── mod.rs

.cargo/                     # Cargo configuration
├── config.toml
└── mod.rs

docs/                       # Documentation
├── API.md
├── ARCHITECTURE.md
├── SECURITY.md
└── DEPLOYMENT.md

Cargo.lock                  # Locked dependencies
Cargo.toml                  # Workspace manifest
rust-toolchain.toml         # Rust version specification
docker-compose.yml          # Docker Compose configuration
docker-compose.dev.yml      # Development Docker Compose
docker-compose.prod.yml     # Production Docker Compose
prometheus.yml             # Prometheus configuration
```

## Security Specification

### Authentication
- JWT-based authentication using jsonwebtoken (jsonwebtoken = "9.3")
- Access token expiry: 15 minutes
- Refresh token expiry: 7 days
- Password hashing: Argon2

### Authorization
- Role-Based Access Control (RBAC)
- Roles: admin, user, guest
- Permission system with middleware enforcement

### API Security
- TLS 1.3 required for production
- CORS policy with configurable allowed origins
- Rate limiting: 100 requests per minute per IP
- Input validation on all endpoints
- SQL injection prevention via parameterized queries
- XSS protection headers
- CSRF token validation

### Secrets Management
- Environment variables for configuration
- Docker secrets for production
- No hardcoded credentials

### Security Headers
- HSTS (HTTP Strict Transport Security)
- CSP (Content Security Policy)
- X-Frame-Options: DENY
- X-Content-Type-Options: nosniff
- Referrer-Policy: strict-origin-when-cross-origin

## CI/CD Specification

### GitHub Actions Workflow (.github/workflows/)

```yaml
# ci.yml - Continuous Integration
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main, develop]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@1.94.0
      - uses: Swatinem/rust-cache@v2
      - name: Run tests
        run: cargo test --all-features
      - name: Run clippy
        run: cargo clippy -- -D warnings
      - name: Run fmt
        run: cargo fmt --check

  security:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@1.94.0
      - name: Audit dependencies
        run: cargo audit
      - name: Check vulnerabilities
        run: cargo-deny check

# cd.yml - Continuous Deployment
name: CD

on:
  push:
    branches: [main]

jobs:
  build-and-push:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Build Docker image
        run: docker build -t app:${{ github.sha }} .
      - name: Run container health check
        run: docker run -d --health-cmd="curl -f http://localhost/health" app:${{ github.sha }}
```

### Docker Build Multi-Stage

```dockerfile
# Build stage
FROM rust:1.94.0 AS builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
COPY --from=builder /app/target/release/app /usr/local/bin/
CMD ["app"]
```

## Testing Strategy

### Unit Tests
- Minimum 80% code coverage
- Test business logic in isolation
- Use mockall for mocking dependencies

### Integration Tests
- Test API endpoints with real HTTP requests
- Use wiremock for external service mocking
- Use testcontainers for database testing

### Property-Based Tests
- Use proptest for input fuzzing
- Test edge cases and boundary conditions

### Test Configuration
```toml
[profile.test]
opt-level = 0
debug = true
incremental = true
overflow-checks = true

[lib]
test = true

[[test]]
name = "integration"
path = "tests/integration/main.rs"
```

## Development Workflow

### Prerequisites
- Rust 1.94.0
- Docker & Docker Compose
- PostgreSQL 17.8 (via Docker)
- Redis 8.6 (via Docker)

### Setup Development Environment
```bash
# Start infrastructure
docker-compose -f docker-compose.dev.yml up -d

# Run migrations
cargo sqlx migrate run

# Start development server
cargo run --watch
```

### Code Standards
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- Maximum line length: 100 characters
- All public APIs must have documentation
- Use semantic commit messages

### Branching Strategy
- `main` - Production-ready code
- `develop` - Integration branch
- `feature/*` - New features
- `fix/*` - Bug fixes
- `refactor/*` - Code refactoring
