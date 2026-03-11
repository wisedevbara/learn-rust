# OpenAPI 3.0 Specification

## Rust Backend API

This document defines the complete REST API specification for the Rust Backend Framework.

---

## OpenAPI YAML

```yaml
openapi: 3.0.3
info:
  title: Rust Backend API
  description: |
    Production-ready REST API built with Actix-web and Rust.
    
    ## Authentication
    This API uses JWT (JSON Web Token) for authentication. 
    Include the access token in the Authorization header:
    ```
    Authorization: Bearer <access_token>
    ```
    
    ## Rate Limiting
    - 100 requests per minute per IP address
    - Rate limit headers included in response
    
    ## Error Responses
    All errors follow a consistent format:
    ```json
    {
      "error": "Error message",
      "code": "ERROR_CODE"
    }
    ```
  version: 1.0.0
  contact:
    name: Development Team
    email: dev@example.com
  license:
    name: MIT
    url: https://opensource.org/licenses/MIT

servers:
  - url: http://localhost:8080
    description: Development server
  - url: https://staging-api.example.com
    description: Staging server
  - url: https://api.example.com
    description: Production server

tags:
  - name: Authentication
    description: User authentication endpoints
  - name: Users
    description: User management endpoints
  - name: Health
    description: Health check and monitoring

components:
  securitySchemes:
    bearerAuth:
      type: http
      scheme: bearer
      bearerFormat: JWT
      description: |
        JWT access token. 
        - Access token expires in 15 minutes
        - Use /api/v1/auth/refresh to get new access token

  schemas:
    # Request Schemas
    RegisterRequest:
      type: object
      required:
        - email
        - password
      properties:
        email:
          type: string
          format: email
          example: user@example.com
          description: Valid email address
        password:
          type: string
          format: password
          example: SecurePassword123!
          description: |
            Password must be:
            - At least 8 characters
            - At least 1 uppercase letter
            - At least 1 number
        role:
          type: string
          enum: [admin, user, guest]
          default: user
          example: user
          description: User role (default: user)

    LoginRequest:
      type: object
      required:
        - email
        - password
      properties:
        email:
          type: string
          format: email
          example: user@example.com
        password:
          type: string
          format: password
          example: SecurePassword123!

    RefreshTokenRequest:
      type: object
      required:
        - refresh_token
      properties:
        refresh_token:
          type: string
          example: eyJhbGciOiJIUzI1NiIs...
          description: Valid refresh token

    UpdateUserRequest:
      type: object
      properties:
        email:
          type: string
          format: email
          example: newemail@example.com
        role:
          type: string
          enum: [admin, user, guest]

    # Response Schemas
    User:
      type: object
      properties:
        id:
          type: string
          format: uuid
          example: 550e8400-e29b-41d4-a716-446655440000
        email:
          type: string
          format: email
          example: user@example.com
        role:
          type: string
          enum: [admin, user, guest]
          example: user
        created_at:
          type: string
          format: date-time
          example: 2026-03-11T00:00:00Z
        updated_at:
          type: string
          format: date-time
          example: 2026-03-11T00:00:00Z

    UserListResponse:
      type: object
      properties:
        users:
          type: array
          items:
            $ref: '#/components/schemas/User'
        total:
          type: integer
          example: 100
        page:
          type: integer
          example: 1
        per_page:
          type: integer
          example: 20

    RegisterResponse:
      type: object
      properties:
        message:
          type: string
          example: User registered successfully
        user:
          $ref: '#/components/schemas/User'

    LoginResponse:
      type: object
      properties:
        access_token:
          type: string
          example: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
        refresh_token:
          type: string
          example: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
        expires_in:
          type: integer
          example: 900
          description: Access token expiry in seconds (900 = 15 minutes)
        token_type:
          type: string
          example: Bearer

    RefreshResponse:
      type: object
      properties:
        access_token:
          type: string
          example: eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...
        expires_in:
          type: integer
          example: 900
        token_type:
          type: string
          example: Bearer

    HealthResponse:
      type: object
      properties:
        status:
          type: string
          enum: [healthy, unhealthy]
          example: healthy
        checks:
          type: object
          properties:
            database:
              type: string
              example: ok
            cache:
              type: string
              example: ok

    ErrorResponse:
      type: object
      properties:
        error:
          type: string
          example: Invalid email format
        code:
          type: string
          example: BAD_REQUEST

  parameters:
    userId:
      name: id
      in: path
      required: true
      description: User UUID
      schema:
        type: string
        format: uuid
        example: 550e8400-e29b-41d4-a716-446655440000

    page:
      name: page
      in: query
      required: false
      description: Page number (default: 1)
      schema:
        type: integer
        default: 1
        minimum: 1
        example: 1

    perPage:
      name: per_page
      in: query
      required: false
      description: Items per page (default: 20, max: 100)
      schema:
        type: integer
        default: 20
        minimum: 1
        maximum: 100
        example: 20

paths:
  # ===========================================
  # Authentication Endpoints
  # ===========================================
  /api/v1/auth/register:
    post:
      tags:
        - Authentication
      summary: Register a new user
      description: |
        Register a new user account.
        
        ## Access Control
        - Public endpoint (no authentication required)
        
        ## Validation
        - Email must be unique
        - Password: min 8 chars, 1 uppercase, 1 number
        - Default role: user
      operationId: registerUser
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/RegisterRequest'
      responses:
        '201':
          description: User successfully registered
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/RegisterResponse'
        '400':
          description: Bad request - validation error
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ErrorResponse'
              examples:
                invalid_email:
                  summary: Invalid email format
                  value:
                    error: Invalid email format
                    code: BAD_REQUEST
                weak_password:
                  summary: Weak password
                  value:
                    error: Password must be at least 8 characters with 1 uppercase and 1 number
                    code: BAD_REQUEST
                email_exists:
                  summary: Email already exists
                  value:
                    error: Email already registered
                    code: BAD_REQUEST

  /api/v1/auth/login:
    post:
      tags:
        - Authentication
      summary: User login
      description: |
        Authenticate user and receive JWT tokens.
        
        ## Access Control
        - Public endpoint (no authentication required)
        
        ## Response
        Returns access token (15 min) and refresh token (7 days)
      operationId: loginUser
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/LoginRequest'
      responses:
        '200':
          description: Login successful
          headers:
            X-RateLimit-Limit:
              schema:
                type: integer
              description: Maximum requests per minute
            X-RateLimit-Remaining:
              schema:
                type: integer
              description: Remaining requests in current window
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/LoginResponse'
        '401':
          description: Invalid credentials
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ErrorResponse'
              examples:
                wrong_password:
                  summary: Wrong password
                  value:
                    error: Invalid credentials
                    code: UNAUTHORIZED
                user_not_found:
                  summary: User not found
                  value:
                    error: Invalid credentials
                    code: UNAUTHORIZED

  /api/v1/auth/refresh:
    post:
      tags:
        - Authentication
      summary: Refresh access token
      description: |
        Refresh access token using a valid refresh token.
        
        ## Access Control
        - Public endpoint (no authentication required)
        
        ## Notes
        - Refresh token is single-use (will be rotated)
        - Old refresh token will be invalidated
      operationId: refreshToken
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/RefreshTokenRequest'
      responses:
        '200':
          description: Token refreshed successfully
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/RefreshResponse'
        '401':
          description: Invalid or expired refresh token
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ErrorResponse'
              examples:
                expired_token:
                  summary: Token expired
                  value:
                    error: Refresh token expired
                    code: UNAUTHORIZED
                used_token:
                  summary: Token already used
                  value:
                    error: Refresh token already used
                    code: UNAUTHORIZED

  # ===========================================
  # User Management Endpoints
  # ===========================================
  /api/v1/users:
    get:
      tags:
        - Users
      summary: Get all users
      description: |
        Retrieve a paginated list of all users.
        
        ## Access Control
        - Requires authentication
        - Requires admin role
      operationId: getUsers
      security:
        - bearerAuth: []
      parameters:
        - $ref: '#/components/parameters/page'
        - $ref: '#/components/parameters/perPage'
      responses:
        '200':
          description: Users retrieved successfully
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/UserListResponse'
        '401':
          description: Unauthorized - missing or invalid token
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ErrorResponse'
        '403':
          description: Forbidden - admin role required
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ErrorResponse'

  /api/v1/users/{id}:
    parameters:
      - $ref: '#/components/parameters/userId'

    get:
      tags:
        - Users
      summary: Get user by ID
      description: |
        Retrieve a specific user by UUID.
        
        ## Access Control
        - Requires authentication
        - Admin can view any user
        - User can view their own profile
      operationId: getUserById
      security:
        - bearerAuth: []
      responses:
        '200':
          description: User found
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/User'
        '401':
          description: Unauthorized
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ErrorResponse'
        '403':
          description: Forbidden - not admin or self
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ErrorResponse'
        '404':
          description: User not found
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ErrorResponse'

    put:
      tags:
        - Users
      summary: Update user
      description: |
        Update user information.
        
        ## Access Control
        - Requires authentication
        - Admin can update any user
        - User can update their own profile (except role)
      operationId: updateUser
      security:
        - bearerAuth: []
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/UpdateUserRequest'
      responses:
        '200':
          description: User updated successfully
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/User'
        '400':
          description: Bad request
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ErrorResponse'
        '401':
          description: Unauthorized
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ErrorResponse'
        '403':
          description: Forbidden
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ErrorResponse'
        '404':
          description: User not found
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ErrorResponse'

    delete:
      tags:
        - Users
      summary: Delete user
      description: |
        Delete a user account (admin only).
        
        ## Access Control
        - Requires authentication
        - Requires admin role
        - Cannot delete own account
      operationId: deleteUser
      security:
        - bearerAuth: []
      responses:
        '204':
          description: User deleted successfully
        '401':
          description: Unauthorized
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ErrorResponse'
        '403':
          description: Forbidden - admin role required or cannot delete self
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ErrorResponse'
        '404':
          description: User not found
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/ErrorResponse'

  # ===========================================
  # Health & Monitoring Endpoints
  # ===========================================
  /health:
    get:
      tags:
        - Health
      summary: Health check
      description: |
        Check application health status.
        
        ## Access Control
        - Public endpoint (no authentication required)
        
        ## Response Time
        - Should respond within 100ms
      operationId: healthCheck
      responses:
        '200':
          description: Application is healthy
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/HealthResponse'
              example:
                status: healthy
                checks:
                  database: ok
                  cache: ok
        '503':
          description: Application is unhealthy
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/HealthResponse'
              example:
                status: unhealthy
                checks:
                  database: error
                  cache: ok

  /metrics:
    get:
      tags:
        - Health
      summary: Prometheus metrics
      description: |
        Export Prometheus-compatible metrics.
        
        ## Access Control
        - Public endpoint (no authentication required)
        
        ## Metrics Available
        - HTTP request duration (histogram)
        - HTTP request count (counter)
        - Active connections (gauge)
        - Database query duration (histogram)
        - Cache hit/miss ratio (counter)
      operationId: getMetrics
      responses:
        '200':
          description: Metrics in Prometheus format
          content:
            text/plain:
              schema:
                type: string
              example: |
                # HELP http_requests_total Total HTTP requests
                # TYPE http_requests_total counter
                http_requests_total{method="GET",status="200"} 1234
                
                # HELP http_request_duration_seconds HTTP request duration
                # TYPE http_request_duration_seconds histogram
                http_request_duration_seconds_bucket{le="0.005"} 100

# ===========================================
# Security
# ===========================================
security:
  - bearerAuth: []

# ===========================================
# External Documentation
# ===========================================
externalDocs:
  description: Project Documentation
  url: https://github.com/your-repo/rust-backend
```

---

## API Endpoint Summary

### Authentication

| Method | Endpoint | Description | Auth Required | Roles |
|--------|----------|-------------|---------------|-------|
| POST | /api/v1/auth/register | Register new user | No | - |
| POST | /api/v1/auth/login | User login | No | - |
| POST | /api/v1/auth/refresh | Refresh token | No | - |

### Users

| Method | Endpoint | Description | Auth Required | Roles |
|--------|----------|-------------|---------------|-------|
| GET | /api/v1/users | Get all users | Yes | admin |
| GET | /api/v1/users/:id | Get user by ID | Yes | admin, self |
| PUT | /api/v1/users/:id | Update user | Yes | admin, self |
| DELETE | /api/v1/users/:id | Delete user | Yes | admin |

### Health & Monitoring

| Method | Endpoint | Description | Auth Required |
|--------|----------|-------------|---------------|
| GET | /health | Health check | No |
| GET | /metrics | Prometheus metrics | No |

---

## Error Codes

| Code | HTTP Status | Description |
|------|-------------|-------------|
| BAD_REQUEST | 400 | Invalid input data |
| UNAUTHORIZED | 401 | Missing or invalid token |
| FORBIDDEN | 403 | Insufficient permissions |
| NOT_FOUND | 404 | Resource not found |
| RATE_LIMITED | 429 | Too many requests |
| INTERNAL_ERROR | 500 | Server error |

---

## Rate Limiting

| Header | Description |
|--------|-------------|
| X-RateLimit-Limit | Maximum requests per minute |
| X-RateLimit-Remaining | Remaining requests in window |
| X-RateLimit-Reset | Unix timestamp when limit resets |

---

**API Version: 1.0.0**

**Last Updated: 2026-03-11**
