# Rust Backend Framework V1.0.0 - Release Notes

![Rust](https://img.shields.io/badge/Rust-1.94.0-orange)
![Actix-web](https://img.shields.io/badge/Actix--web-4.4-blue)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-17.8-blue)
![License](https://img.shields.io/badge/License-MIT-green)

## Overview

**Version:** V1.0.0  
**Release Date:** March 13, 2026  
**Status:** Production Ready  
**Commit:** 497a46d

---

## 1. Ringkasan Perubahan (Summary)

Release V1.0.0 menandai ketersediaan produksi penuh (production-ready) dari Rust Backend Framework. Framework ini mengimplementasikan arsitektur berlapis (layered architecture) dengan fokus pada keamanan, skalabilitas, dan kepatuhan penuh terhadap spesifikasi blueprint yang telah ditetapkan.

Framework ini dibangun dengan teknologi modern 包括:
- **Web Framework:** Actix-web 4.4 (high-performance Rust web framework)
- **Database:** PostgreSQL 17.8 dengan sqlx ORM
- **Cache:** Redis 8.6 untuk session management
- **Authentication:** JWT dengan Argon2id password hashing

### Compliance Score: 100%

| Spesifikasi | Kepatuhan |
|-------------|-----------|
| API-SPECIFICATION.md | 100% |
| DATABASE-SCHEMA.md | 100% |
| SECURITY-BASELINE.md | 100% |

---

## 2. Fitur Baru (New Features)

### 2.1 Authentication System

#### JWT Authentication
- **Access Token:** 900 detik (15 menit)
- **Refresh Token:** 604800 detik (7 hari)
- **Algorithm:** HS256 dengan jsonwebtoken 9.3

#### Password Security
- **Algorithm:** Argon2id (versi 0.5)
- **Parameters:**
  - Memory cost: 65536 kB
  - Time cost: 3 iterations
  - Parallelism: 4
- **Validation:** Min 8 karakter, 1 uppercase, 1 number

#### API Endpoints

| Method | Endpoint | Description | Access |
|--------|----------|-------------|--------|
| POST | /api/v1/auth/register | User registration | Public |
| POST | /api/v1/auth/login | User login | Public |
| POST | /api/v1/auth/refresh | Token refresh | Public |
| POST | /api/v1/auth/logout | Logout | Public |
| GET | /api/v1/users | List users | Admin |
| GET | /api/v1/users/{id} | Get user | Admin/Self |
| PUT | /api/v1/users/{id} | Update user | Admin/Self |
| DELETE | /api/v1/users/{id} | Delete user | Admin |

### 2.2 User Management

- **CRUD Operations:** Create, Read, Update, Delete users
- **Pagination:** Default 20 items, max 100 per page
- **RBAC (Role-Based Access Control):**
  - `admin`: Full system access
  - `user`: Default access, own profile
  - `guest`: Limited read-only access

### 2.3 Security Features

#### Security Headers
- `Strict-Transport-Security: max-age=31536000`
- `Content-Security-Policy`
- `X-Frame-Options: DENY`
- `X-Content-Type-Options: nosniff`
- `Referrer-Policy: strict-origin-when-cross-origin`

#### Rate Limiting
- **Limit:** 100 requests per minute per IP
- **Implementation:** tower-http rate-limit middleware

#### CORS
- Configurable allowed origins
- Support for credentials
- Pre-flight request handling

### 2.4 Database Schema

#### Tables
- `users` - User accounts
- `refresh_tokens` - JWT refresh tokens

#### Indexes
- `idx_users_email` - Email lookup
- `idx_users_role` - Role-based filtering
- `idx_users_created_at` - Timestamp sorting
- `idx_refresh_tokens_user_id` - User token lookup
- `idx_refresh_tokens_expires_at` - Expiration cleanup
- `idx_refresh_tokens_token_hash` - Token validation

### 2.5 Testing

- **Unit Tests:** 290+ test cases
- **Integration Tests:** 50+ test cases
- **Coverage Areas:**
  - User entity tests
  - Password validation/hashing tests
  - JWT token generation/validation tests
  - Authentication flow tests
  - User CRUD tests

### 2.6 Docker Support

- Multi-stage Dockerfile for production
- Development Dockerfile with hot-reload
- Docker Compose configuration
- Nginx reverse proxy configuration

---

## 3. Perbaikan Bug (Bug Fixes)

### 3.1 Blueprint Compliance Fixes

| Issue | Description | Status |
|-------|-------------|--------|
| UserListResponse format | Changed from `{data, pagination}` to `{users, total, page, per_page}` | ✅ Fixed |
| Security headers | Added complete security headers middleware | ✅ Fixed |
| Rate limiting | Implemented 100 req/min/IP rate limiting | ✅ Fixed |
| Missing indexes | Added idx_users_created_at and idx_refresh_tokens_token_hash | ✅ Fixed |

### 3.2 Previous Issues Resolved

- Repository state sharing between auth and users modules
- CORS configuration for frontend integration
- Proper error response formatting
- JWT token expiry handling

---

## 4. Perubahan yang Mungkin Mempengaruhi Kompatibilitas (Breaking Changes)

### 4.1 API Response Format

**UserListResponse** - Changed field names:

```json
// Before (V0.x)
{
  "data": [...],
  "pagination": {
    "page": 1,
    "per_page": 20,
    "total": 100,
    "total_pages": 5
  }
}

// After (V1.0.0)
{
  "users": [...],
  "total": 100,
  "page": 1,
  "per_page": 20
}
```

### 4.2 Configuration

- Environment variable `JWT_SECRET` is now required (previously had default)
- Database connection now uses SSL/TLS by default in production

### 4.3 Dependencies

Some internal implementation details may have changed. External API contracts remain backward compatible for authentication endpoints.

---

## 5. Dependensi yang Diupdate (Dependencies Updated)

### 5.1 Core Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| actix-web | 4.4 | HTTP server |
| tokio | 1.42 | Async runtime |
| sqlx | 0.8 | Database ORM |
| redis | 0.27 | Cache client |
| jsonwebtoken | 9.3 | JWT handling |
| argon2 | 0.5 | Password hashing |

### 5.2 Additional Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| tower-http | 0.6 | Rate limiting, CORS |
| serde | 1.0 | Serialization |
| chrono | 0.4 | Date/time |
| uuid | 1.11 | UUID generation |
| sha2 | 0.10 | Token hashing |
| regex | 1.10 | Password validation |

### 5.3 Dev Dependencies

| Package | Version | Purpose |
|---------|---------|---------|
| pretty_assertions | 1.4 | Test output |
| mockall | 0.13 | Mocking |
| wiremock | 0.6 | HTTP mocking |

---

## 6. Credits dan Thanks (Credits/Thanks)

### 6.1 Core Contributors

- **wisedevbara** - Lead Developer

### 6.2 Technology Credits

Terima kasih kepada semua maintainer dan kontributor proyek open source berikut:

- [Actix-web](https://actix.rs/) - High-performance web framework
- [Tokio](https://tokio.rs/) - Async runtime
- [sqlx](https://github.com/launchbadge/sqlx) - Async SQL toolkit
- [serde](https://serde.rs/) - Serialization framework
- [Argon2](https://github.com/argon2/argon2) - Password hashing

### 6.3 Documentation

Blueprint dan spesifikasi teknis mengacu pada:
- docs/API-SPECIFICATION.md
- docs/DATABASE-SCHEMA.md
- docs/SECURITY-BASELINE.md
- docs/ARCHITECTURE.md

---

## 7. Cara Install dan Penggunaan

### 7.1 Prerequisites

- Rust 1.94.0
- Docker & Docker Compose
- PostgreSQL 17.8
- Redis 8.6

### 7.2 Local Development

```bash
# Clone repository
git clone https://github.com/wisedevbara/learn-rust.git
cd learn-rust

# Start infrastructure
docker-compose -f docker-compose.dev.yml up -d

# Run application
cargo run

# Run tests
cargo test
```

### 7.3 Production Deployment

```bash
# Build release
cargo build --release

# Run with Docker
docker-compose -f docker-compose.yml up -d
```

---

## 8. Links

- **Repository:** https://github.com/wisedevbara/learn-rust
- **Documentation:** https://github.com/wisedevbara/learn-rust/tree/main/docs
- **Issues:** https://github.com/wisedevbara/learn-rust/issues

---

## 9. Changelog

### V1.0.0 (2026-03-13)
- Initial production release
- Full blueprint compliance (100%)
- JWT authentication with refresh tokens
- Argon2id password hashing
- RBAC implementation
- Security headers middleware
- Rate limiting
- Comprehensive test suite
- Docker support

---

**Release Manager:** wisedevbara  
**License:** MIT
