# Technical Specification

## Rust Backend Framework

| Attribute | Value |
|-----------|-------|
| Version | 1.0.0 |
| Status | READY FOR IMPLEMENTATION |
| Created Date | 2026-03-11 |
| Based On | PRODUCTION-READINESS-ASSESSMENT.md (97%) |
| Assessed By | Security & Architecture Team |

---

## Table of Contents

1. [Overview](#1-overview)
2. [Functional Requirements](#2-functional-requirements)
3. [Non-Functional Requirements](#3-non-functional-requirements)
4. [System Architecture](#4-system-architecture)
5. [API Specification](#5-api-specification)
6. [Database Design](#6-database-design)
7. [Security Architecture](#7-security-architecture)
8. [Infrastructure Design](#8-infrastructure-design)
9. [Implementation Plan](#9-implementation-plan)
10. [Risk Assessment](#10-risk-assessment)
11. [Acceptance Criteria](#11-acceptance-criteria)

---

## 1. Overview

### 1.1 Project Summary

This document serves as the **Technical Specification** for the Rust Backend Framework project. It provides detailed technical requirements, design decisions, and implementation guidelines that will govern the development process. This specification is based on the Production Readiness Assessment (PRA) which achieved a 97% score, confirming the project's readiness for implementation.

### 1.2 Project Goals

| Goal | Description | Priority |
|------|-------------|----------|
| G1 | Provide a production-ready backend framework using Rust | Critical |
| G2 | Enable rapid development of secure web APIs | Critical |
| G3 | Implement robust authentication and authorization | Critical |
| G4 | Support scalable architecture with caching | High |
| G5 | Enable easy monitoring and observability | High |
| G6 | Provide comprehensive documentation | Medium |

### 1.3 Scope

**In Scope:**
- JWT-based authentication system
- Role-Based Access Control (RBAC)
- RESTful API development
- PostgreSQL database integration
- Redis caching
- Prometheus metrics
- Docker containerization

**Out of Scope:**
- GraphQL API (future consideration)
- Microservices architecture (v2)
- Mobile SDK (future consideration)
- Billing/Payment integration (external service)

---

## 2. Functional Requirements

### 2.1 Authentication System

#### FR-001: User Registration

| Requirement | Details |
|-------------|---------|
| Description | Allow new users to register with email and password |
| Input | email (string), password (string), role (optional string) |
| Validation | Email format validation, password strength (min 8 chars, 1 uppercase, 1 number) |
| Output | User object with generated UUID, confirmation message |
| Business Rules | - Email must be unique<br>- Default role is "user"<br>- Password must be hashed with Argon2 |

#### FR-002: User Login

| Requirement | Details |
|-------------|---------|
| Description | Authenticate users and issue JWT tokens |
| Input | email (string), password (string) |
| Validation | Email exists, password matches hash |
| Output | Access token (15 min), refresh token (7 days) |
| Business Rules | - Failed attempts should be logged<br>- Account lockout after 5 failed attempts |

#### FR-003: Token Refresh

| Requirement | Details |
|-------------|---------|
| Description | Refresh access token using valid refresh token |
| Input | refresh_token (string) |
| Validation | Token exists, not expired, belongs to user |
| Output | New access token |
| Business Rules | - Refresh token can be used once (rotation)<br>- Invalidate old refresh token |

#### FR-004: User Logout

| Requirement | Details |
|-------------|---------|
| Description | Invalidate user tokens on logout |
| Input | access_token (string) |
| Validation | Token is valid |
| Output | Success confirmation |
| Business Rules | - Blacklist access token<br>- Invalidate refresh token if provided |

### 2.2 User Management

#### FR-005: Get All Users

| Requirement | Details |
|-------------|---------|
| Description | Retrieve list of all users (admin only) |
| Input | pagination (page, per_page), sorting |
| Validation | Admin role required |
| Output | Array of user objects, total count, pagination info |
| Business Rules | - Pagination required (default 20 per page)<br>- Sort by created_at desc |

#### FR-006: Get User by ID

| Requirement | Details |
|-------------|---------|
| Description | Retrieve specific user by UUID |
| Input | user_id (UUID) |
| Validation | User exists, requester is admin or self |
| Output | User object |
| Business Rules | - Return 404 if not found |

#### FR-007: Update User

| Requirement | Details |
|-------------|---------|
| Description | Update user information |
| Input | user_id (UUID), updates (email, role) |
| Validation | User exists, requester is admin or self |
| Output | Updated user object |
| Business Rules | - Email uniqueness check<br>- Role change requires admin |

#### FR-008: Delete User

| Requirement | Details |
|-------------|---------|
| Description | Soft delete user (admin only) |
| Input | user_id (UUID) |
| Validation | User exists, requester is admin, cannot delete self |
| Output | 204 No Content |
| Business Rules | - Hard delete after 30 days (cleanup job) |

### 2.3 Health Monitoring

#### FR-009: Health Check

| Requirement | Details |
|-------------|---------|
| Description | Check application health status |
| Input | None |
| Validation | None |
| Output | Health status, database connection, cache connection |
| Business Rules | - No authentication required<br>- Response time < 100ms |

#### FR-010: Metrics Export

| Requirement | Details |
|-------------|---------|
| Description | Export Prometheus metrics |
| Input | None |
| Validation | None |
| Output | Prometheus format metrics |
| Business Rules | - Include HTTP request metrics<br>- Include database metrics<br>- Include custom business metrics |

---

## 3. Non-Functional Requirements

### 3.1 Performance Requirements

| Requirement | Target | Measurement |
|-------------|--------|-------------|
| API Response Time (p95) | < 200ms | Load testing |
| API Response Time (p99) | < 500ms | Load testing |
| Concurrent Users | 1000 | Load testing |
| Requests per Second | 500 | Load testing |
| Database Connection Pool | 10-20 connections | Configuration |
| Memory Usage | < 512MB (idle) | Production monitoring |

### 3.2 Availability Requirements

| Requirement | Target |
|-------------|--------|
| Uptime | 99.9% (8.76 hours downtime/year) |
| Planned Maintenance Window | 4 hours/month |
| Unplanned Recovery Time | < 1 hour |
| Data Recovery Point | < 1 hour |

### 3.3 Security Requirements

| Requirement | Implementation |
|-------------|----------------|
| Authentication | JWT with 15 min access / 7 days refresh |
| Password Storage | Argon2id (65536 KB, 3 iterations, 4 parallelism) |
| TLS Version | TLS 1.3 minimum |
| Rate Limiting | 100 requests/minute/IP |
| API Security | Input validation, parameterized queries |

### 3.4 Scalability Requirements

| Requirement | Design |
|-------------|--------|
| Horizontal Scaling | Stateless application design |
| Database Scaling | Connection pooling, read replicas |
| Cache Scaling | Redis with TTL |
| Load Balancing | nginx with round-robin |

---

## 4. System Architecture

### 4.1 High-Level Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Client Requests                           │
└─────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                    nginx Reverse Proxy                           │
│                 (TLS Termination, Load Balancing)                │
│                        Port: 80/443                              │
└─────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Actix-web Application                       │
│                        Port: 8080                                │
│  ┌───────────────────────────────────────────────────────────┐  │
│  │                   Middleware Stack                         │  │
│  │  Tracing → Auth → CORS → Rate Limit → Request Logging    │  │
│  └───────────────────────────────────────────────────────────┘  │
│  ┌─────────────┐ ┌─────────────┐ ┌─────────────┐ ┌───────────┐ │
│  │ API Layer   │ │ Business    │ │ Data Layer  │ │ Services  │ │
│  │ (Handlers)  │ │ Layer       │ │ (Repos)     │ │ (Cache)   │ │
│  └─────────────┘ └─────────────┘ └─────────────┘ └───────────┘ │
└─────────────────────────────────────────────────────────────────┘
                                 │
        ┌────────────────────────┼────────────────────────┐
        ▼                        ▼                        ▼
┌───────────────┐        ┌───────────────┐        ┌───────────────┐
│  PostgreSQL   │        │    Redis      │        │  Prometheus   │
│    17.8       │        │     8.6       │        │   v3.5.1      │
│   (Primary)   │        │    (Cache)    │        │  (Monitoring) │
└───────────────┘        └───────────────┘        └───────────────┘
```

### 4.2 Layer Architecture

```
┌────────────────────────────────────────────┐
│           API Layer (src/api/)             │
│  - HTTP Handlers                           │
│  - Request/Response DTOs                   │
│  - Route Definitions                       │
│  - OpenAPI Annotations                     │
│  Depends on: Business Layer                 │
└────────────────────────────────────────────┘
                    │
                    ▼
┌────────────────────────────────────────────┐
│       Business Layer (src/business/)       │
│  - Domain Entities                         │
│  - Business Logic                          │
│  - Service Traits & Implementations        │
│  - Validation Logic                        │
│  Depends on: Data Layer, Services Layer    │
└────────────────────────────────────────────┘
                    │
                    ▼
┌────────────────────────────────────────────┐
│         Data Layer (src/data/)             │
│  - Repository Traits                       │
│  - Database Models                         │
│  - SQL Migrations                          │
│  Depends on: External Systems (PostgreSQL) │
└────────────────────────────────────────────┘
                    │
                    ▼
┌────────────────────────────────────────────┐
│      Services Layer (src/services/)        │
│  - Cache Service (Redis)                   │
│  - External API Clients                    │
│  - Email Service (Future)                  │
│  Depends on: External Services             │
└────────────────────────────────────────────┘
```

### 4.3 Component Design

| Component | Responsibility | Public API |
|-----------|---------------|------------|
| AuthService | Authentication logic | register, login, refresh, logout |
| UserService | User management | create, read, update, delete |
| UserRepository | Database access | find_by_id, find_by_email, create, update, delete |
| CacheService | Redis operations | get, set, delete, exists |

---

## 5. API Specification

### 5.1 API Versioning

| Version | Base URL | Status |
|---------|----------|--------|
| v1 | /api/v1 | Active |

### 5.2 Authentication Endpoints

#### POST /api/v1/auth/register

Register a new user.

**Request:**
```json
{
  "email": "user@example.com",
  "password": "SecurePassword123!",
  "role": "user"
}
```

**Response (201):**
```json
{
  "message": "User registered successfully",
  "user": {
    "id": "550e8400-e29b-41d4-a716-446655440000",
    "email": "user@example.com",
    "role": "user",
    "created_at": "2026-03-11T00:00:00Z"
  }
}
```

#### POST /api/v1/auth/login

Authenticate user.

**Request:**
```json
{
  "email": "user@example.com",
  "password": "SecurePassword123!"
}
```

**Response (200):**
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIs...",
  "refresh_token": "eyJhbGciOiJIUzI1NiIs...",
  "expires_in": 900,
  "token_type": "Bearer"
}
```

#### POST /api/v1/auth/refresh

Refresh access token.

**Request:**
```json
{
  "refresh_token": "eyJhbGciOiJIUzI1NiIs..."
}
```

**Response (200):**
```json
{
  "access_token": "eyJhbGciOiJIUzI1NiIs...",
  "expires_in": 900,
  "token_type": "Bearer"
}
```

### 5.3 User Endpoints

| Method | Endpoint | Description | Auth Required | Roles |
|--------|----------|-------------|---------------|-------|
| GET | /api/v1/users | Get all users | Yes | admin |
| GET | /api/v1/users/:id | Get user by ID | Yes | admin, self |
| PUT | /api/v1/users/:id | Update user | Yes | admin, self |
| DELETE | /api/v1/users/:id | Delete user | Yes | admin |

### 5.4 Health & Metrics

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| GET | /health | Health check | No |
| GET | /metrics | Prometheus metrics | No |

### 5.5 Error Responses

| Status | Code | Description |
|--------|------|-------------|
| 400 | BAD_REQUEST | Invalid input |
| 401 | UNAUTHORIZED | Invalid or missing token |
| 403 | FORBIDDEN | Insufficient permissions |
| 404 | NOT_FOUND | Resource not found |
| 429 | RATE_LIMITED | Too many requests |
| 500 | INTERNAL_ERROR | Server error |

---

## 6. Database Design

### 6.1 Schema Overview

```sql
-- Main tables
users          -- User accounts
refresh_tokens -- JWT refresh tokens

-- Indexes
idx_users_email        -- Email lookup
idx_refresh_tokens_user_id -- User token lookup
```

### 6.2 Users Table

```sql
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    role VARCHAR(50) NOT NULL DEFAULT 'user',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Indexes
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_role ON users(role);
```

**Column Definitions:**

| Column | Type | Constraints | Description |
|--------|------|-------------|-------------|
| id | UUID | PRIMARY KEY | Auto-generated UUID |
| email | VARCHAR(255) | UNIQUE, NOT NULL | User email |
| password_hash | VARCHAR(255) | NOT NULL | Argon2 hash |
| role | VARCHAR(50) | NOT NULL, DEFAULT 'user' | RBAC role |
| created_at | TIMESTAMP | DEFAULT NOW() | Creation timestamp |
| updated_at | TIMESTAMP | DEFAULT NOW() | Last update timestamp |

### 6.3 Refresh Tokens Table

```sql
CREATE TABLE refresh_tokens (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_hash VARCHAR(255) NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Indexes
CREATE INDEX idx_refresh_tokens_user_id ON refresh_tokens(user_id);
CREATE INDEX idx_refresh_tokens_expires_at ON refresh_tokens(expires_at);
```

### 6.4 Role Definitions

| Role | Description | Permissions |
|------|-------------|-------------|
| admin | Administrator | All operations |
| user | Standard user | Read own profile, update own profile |
| guest | Guest user | Read public resources only |

---

## 7. Security Architecture

### 7.1 Authentication Flow

```
1. User submits credentials
       │
       ▼
2. Server validates credentials
       │
       ▼
3. Generate JWT access token (15 min)
       │
       ▼
4. Generate JWT refresh token (7 days)
       │
       ▼
5. Store refresh token hash in database
       │
       ▼
6. Return tokens to client
```

### 7.2 Authorization Matrix

| Resource | admin | user | guest |
|----------|-------|------|-------|
| POST /auth/register | ✓ | ✓ | ✓ |
| POST /auth/login | ✓ | ✓ | ✓ |
| GET /users | ✓ | ✗ | ✗ |
| GET /users/:id (self) | ✓ | ✓ | ✗ |
| PUT /users/:id (self) | ✓ | ✓ | ✗ |
| DELETE /users/:id | ✓ | ✗ | ✗ |

### 7.3 Security Headers

| Header | Value |
|--------|-------|
| Strict-Transport-Security | max-age=31536000; includeSubDomains; preload |
| Content-Security-Policy | default-src 'self' |
| X-Frame-Options | DENY |
| X-Content-Type-Options | nosniff |
| Referrer-Policy | strict-origin-when-cross-origin |

### 7.4 Encryption Standards

| Data Type | Method | Key Management |
|-----------|--------|----------------|
| Passwords | Argon2id (65536 KB, 3, 4) | N/A (one-way) |
| JWT Secrets | AES-256-GCM | Environment variable |
| Redis Keys | AES-256-GCM | Environment variable |
| Database Connection | TLS 1.3 | PostgreSQL config |
| API Communication | TLS 1.3 | nginx config |

---

## 8. Infrastructure Design

### 8.1 Environment Configurations

| Environment | Database | Redis | Log Level |
|-------------|----------|-------|-----------|
| Development | localhost:5432 | localhost:6379 | debug |
| Staging | staging-db:5432 | staging-redis:6379 | info |
| Production | prod-db:5432 | prod-redis:6379 | warn |

### 8.2 Resource Requirements

| Environment | Compute | Storage | Network |
|-------------|---------|---------|---------|
| Development | 4 vCPU, 8GB | 50GB SSD | Standard |
| Staging | 8 vCPU, 16GB | 100GB SSD | Standard |
| Production | 16 vCPU, 32GB | 500GB NVMe | Premium |

### 8.3 Docker Services

```yaml
services:
  app:
    build: .
    ports: [8080:8080]
    depends_on: [db, redis]
    environment:
      - DATABASE_URL
      - REDIS_URL
      - JWT_SECRET
    networks: [app-network]

  db:
    image: postgres:17.8
    volumes: [./data/postgres:/var/lib/postgresql/data]
    networks: [app-network]

  redis:
    image: redis:8.6
    volumes: [./data/redis:/data]
    networks: [app-network]

  prometheus:
    image: prom/prometheus:v3.5.1
    ports: [9090:9090]
    networks: [app-network]

  nginx:
    image: nginx:1.29.5
    ports: [80:80, 443:443]
    depends_on: [app]
    networks: [app-network]
```

---

## 9. Implementation Plan

### 9.1 Phase Breakdown

| Phase | Features | Estimated Duration |
|-------|----------|-------------------|
| Phase 1 | Project setup, Docker, basic structure | 1 week |
| Phase 2 | Database schema, migrations | 1 week |
| Phase 3 | Authentication (register, login, JWT) | 2 weeks |
| Phase 4 | User management CRUD | 1 week |
| Phase 5 | Middleware (CORS, rate limiting, logging) | 1 week |
| Phase 6 | Health check, metrics, monitoring | 1 week |
| Phase 7 | Testing (unit, integration) | 2 weeks |
| Phase 8 | Documentation, final review | 1 week |

**Total Estimated: 10 weeks**

### 9.2 Development Workflow

```
1. Create feature branch from develop
       │
       ▼
2. Implement feature according to spec
       │
       ▼
3. Write unit tests
       │
       ▼
4. Run cargo fmt, clippy, test
       │
       ▼
5. Create pull request
       │
       ▼
6. Code review
       │
       ▼
7. Merge to develop
       │
       ▼
8. Integration testing
       │
       ▼
9. Merge to main (release)
```

---

## 10. Risk Assessment

| Risk ID | Description | Probability | Impact | Mitigation |
|---------|-------------|-------------|--------|------------|
| R001 | Database migration failures | Medium | High | Test migrations in staging |
| R002 | Dependency conflicts | Low | Medium | Pin versions in Cargo.lock |
| R003 | Performance issues | Medium | Medium | Implement pagination, caching |
| R004 | Security vulnerabilities | Low | Critical | Regular cargo audit |
| R005 | JWT token leakage | Low | High | Short expiry, token rotation |

---

## 11. Acceptance Criteria

### 11.1 Functional Acceptance

| Feature | Criteria |
|---------|----------|
| User Registration | User can register with valid email/password |
| User Login | User receives valid JWT tokens |
| Token Refresh | New access token generated |
| User CRUD | Admin can manage all users |
| Health Check | Returns current status |
| Metrics | Prometheus format export |

### 11.2 Non-Functional Acceptance

| Criteria | Target |
|----------|--------|
| API Response (p95) | < 200ms |
| Uptime | 99.9% |
| Security | All checks pass |
| Code Coverage | > 80% |
| Documentation | Complete |

### 11.3 Security Acceptance

| Check | Criteria |
|-------|----------|
| Password Hashing | Argon2id with correct parameters |
| JWT Tokens | 15 min access, 7 days refresh |
| Rate Limiting | 100 req/min/IP |
| Security Headers | All present |
| TLS | 1.3 enforced |

---

## Appendix A: Version Compatibility

| Component | Version | Compatible With |
|-----------|---------|-----------------|
| Rust | 1.94.0 | tokio 1.42, actix-web 4.x |
| actix-web | 4.x | tower 0.5, tower-http 0.6 |
| sqlx | 0.8 | PostgreSQL 17.8 |
| tokio | 1.42 | All async code |
| PostgreSQL | 17.8 | sqlx 0.8 |
| Redis | 8.6 | redis crate 0.27 |
| nginx | 1.29.5 | TLS 1.3 |
| Prometheus | v3.5.1 | prometheus-client 0.22 |
| jsonwebtoken | 9.3 | JWT tokens |
| Argon2 | 0.5 | Password hashing |

---

## Appendix B: Quick Reference

### Commands

```bash
# Development
cargo run                              # Run application
cargo test                             # Run tests
cargo clippy                           # Lint
cargo fmt                              # Format

# Database
cargo sqlx migrate run                 # Run migrations
cargo sqlx migrate revert              # Revert

# Docker
docker-compose up -d                   # Start services
docker-compose down                    # Stop services
```

### Environment Variables

```bash
DATABASE_URL=postgresql://postgres:postgres@db:5432/app
REDIS_URL=redis://redis:6379
JWT_SECRET=<minimum-32-characters>
APP_ENV=development
```

---

**Document Status: APPROVED FOR IMPLEMENTATION**

*This Technical Specification is based on the Production Readiness Assessment (97%) and serves as the primary reference for implementation.*
