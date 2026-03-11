# Deployment Guide

## Rust Backend Framework

This guide covers deployment procedures for staging and production environments.

---

## Table of Contents

1. [Overview](#1-overview)
2. [Pre-Deployment Checklist](#2-pre-deployment-checklist)
3. [Staging Deployment](#3-staging-deployment)
4. [Production Deployment](#4-production-deployment)
5. [CI/CD Pipeline](#5-cicd-pipeline)
6. [Rollback Procedures](#6-rollback-procedures)
7. [Monitoring](#7-monitoring)
8. [Backup & Recovery](#8-backup--recovery)

---

## 1. Overview

### 1.1 Environments

| Environment | Purpose | URL |
|-------------|---------|-----|
| Development | Local development | localhost:8080 |
| Staging | Pre-production testing | staging-api.example.com |
| Production | Live service | api.example.com |

### 1.2 Deployment Methods

- **Docker Compose** - Development & Staging
- **Kubernetes** - Production (recommended)
- **Manual** - Emergency fixes

---

## 2. Pre-Deployment Checklist

### 2.1 Security Checklist

- [ ] All security tests pass
- [ ] No high/critical vulnerabilities in dependencies
- [ ] TLS certificates valid
- [ ] Environment variables secured
- [ ] Database credentials rotated

### 2.2 Functional Checklist

- [ ] All unit tests passing
- [ ] All integration tests passing
- [ ] API documentation updated
- [ ] Database migrations tested

### 2.3 Infrastructure Checklist

- [ ] Backup verified
- [ ] Monitoring configured
- [ ] Alerts configured
- [ ] Log retention configured

---

## 3. Staging Deployment

### 3.1 Build Docker Image

```bash
# Build staging image
docker build -t rust-backend:staging .

# Tag for registry
docker tag rust-backend:staging registry.example.com/rust-backend:staging

# Push to registry
docker push registry.example.com/rust-backend:staging
```

### 3.2 Configure Environment

```bash
# Create staging environment file
cp .env.example .env.staging

# Edit configuration
vim .env.staging
```

Required changes:
```bash
APP_ENV=staging
APP_LOG_LEVEL=info
DATABASE_URL=postgresql://user:pass@staging-db:5432/app
REDIS_URL=redis://:pass@staging-redis:6379
JWT_SECRET=<generate-strong-secret>
```

### 3.3 Deploy with Docker Compose

```bash
# Stop existing containers
docker-compose -f docker-compose.yml --env-file .env.staging down

# Pull latest images
docker-compose -f docker-compose.yml --env-file .env.staging pull

# Start services
docker-compose -f docker-compose.yml --env-file .env.staging up -d

# Verify deployment
curl https://staging-api.example.com/health
```

### 3.4 Verify Staging

```bash
# Health check
curl https://staging-api.example.com/health

# Test authentication
curl -X POST https://staging-api.example.com/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"Test123!"}'

# Check logs
docker-compose logs -f app
```

---

## 4. Production Deployment

### 4.1 Infrastructure Requirements

| Resource | Specification |
|----------|---------------|
| Compute | 16 vCPU, 32GB RAM |
| Storage | 500GB NVMe SSD |
| Network | Premium tier |
| Load Balancer | Application Load Balancer |

### 4.2 Production Environment

```bash
# Create production environment file
cp .env.example .env.production

# Configure production values
vim .env.production
```

Production configuration:
```bash
APP_ENV=production
APP_LOG_LEVEL=warn
APP_WORKERS=8

DATABASE_URL=postgresql://user:securepass@prod-db:5432/app?sslmode=require
DATABASE_MAX_CONNECTIONS=20
DATABASE_MIN_CONNECTIONS=5

REDIS_URL=redis://:redispass@prod-redis:6379
REDIS_TTL_SECONDS=1800

JWT_SECRET=<64-character-secret>
JWT_ACCESS_TOKEN_EXPIRY=900
JWT_REFRESH_TOKEN_EXPIRY=604800

ARGON2_MEMORY_COST=65536
ARGON2_TIME_COST=3
ARGON2_PARALLELISM=4

CORS_ALLOWED_ORIGINS=https://example.com
RATE_LIMIT_REQUESTS=100
RATE_LIMIT_WINDOW=60
```

### 4.3 TLS/SSL Configuration

```bash
# Install Let's Encrypt certificate
certbot certonly --nginx -d api.example.com -d www.example.com

# Verify certificate
ls -la /etc/letsencrypt/live/api.example.com/
```

### 4.4 Deploy with Docker Compose

```bash
# Build production image
docker build -t rust-backend:latest .

# Start services
docker-compose -f docker-compose.yml --env-file .env.production up -d --build

# Verify health
curl https://api.example.com/health

# Check logs
docker-compose logs -f
```

### 4.5 Kubernetes Deployment

```yaml
# kubernetes/deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: rust-backend
  labels:
    app: rust-backend
spec:
  replicas: 3
  selector:
    matchLabels:
      app: rust-backend
  template:
    metadata:
      labels:
        app: rust-backend
    spec:
      containers:
      - name: rust-backend
        image: rust-backend:latest
        ports:
        - containerPort: 8080
        env:
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: app-secrets
              key: database-url
        - name: JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: app-secrets
              key: jwt-secret
        resources:
          requests:
            memory: "256Mi"
            cpu: "250m"
          limits:
            memory: "512Mi"
            cpu: "500m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5

---
apiVersion: v1
kind: Service
metadata:
  name: rust-backend
spec:
  selector:
    app: rust-backend
  ports:
  - port: 80
    targetPort: 8080
  type: ClusterIP
```

Deploy:
```bash
kubectl apply -f kubernetes/
```

---

## 5. CI/CD Pipeline

### 5.1 GitHub Actions Workflow

```yaml
name: CI/CD Pipeline

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main, develop]

jobs:
  # ===========================================
  # CI - Continuous Integration
  # ===========================================
  ci:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Setup Rust
      uses: dtolnay/rust-toolchain@1.94.0
      with:
        components: clippy, rustfmt
    
    - name: Cache dependencies
      uses: Swatinem/rust-cache@v2
    
    - name: Run tests
      run: cargo test --all-features
    
    - name: Check formatting
      run: cargo fmt --check
    
    - name: Run clippy
      run: cargo clippy -- -D warnings
    
    - name: Security audit
      run: cargo audit

  # ===========================================
  # Build - Docker Image
  # ===========================================
  build:
    needs: ci
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v4
    
    - name: Set up Docker Buildx
      uses: docker/setup-buildx-action@v3
    
    - name: Build Docker image
      run: docker build -t rust-backend:${{ github.sha }} .
    
    - name: Run container health check
      run: |
        docker run -d --name test-app rust-backend:${{ github.sha }}
        sleep 5
        curl -f http://localhost:8080/health
        docker rm -f test-app
    
    - name: Push to registry
      if: github.ref == 'refs/heads/main'
      run: |
        docker tag rust-backend:${{ github.sha }} registry.example.com/rust-backend:latest
        docker push registry.example.com/rust-backend:latest

  # ===========================================
  # CD - Continuous Deployment
  # ===========================================
  deploy:
    needs: build
    if: github.ref == 'refs/heads/main'
    runs-on: ubuntu-latest
    
    steps:
    - name: Deploy to production
      run: |
        echo "Deploying to production..."
        # Add deployment commands here
    
    - name: Verify deployment
      run: curl -f https://api.example.com/health
```

### 5.2 Pipeline Stages

```
┌─────────┐    ┌─────────┐    ┌─────────┐    ┌─────────┐
│  Build  │───►│  Test   │───►│ Security│───►│ Deploy  │
└─────────┘    └─────────┘    └─────────┘    └─────────┘
   Code         Unit/Int      Audit         Staging/
   Checkout     Tests         Scan         Production
```

---

## 6. Rollback Procedures

### 6.1 Rollback Triggers

| Trigger | Detection | Time Limit |
|---------|-----------|------------|
| Health check failure | Automated | 5 minutes |
| Performance degradation | Metrics | 10 minutes |
| Security vulnerability | Audit alert | 15 minutes |
| Deployment failure | CI/CD | 5 minutes |

### 6.2 Docker Compose Rollback

```bash
# Get previous image tag
docker images | grep rust-backend

# Stop current deployment
docker-compose -f docker-compose.yml --env-file .env.production down

# Pull previous version
docker pull rust-backend:<previous-tag>

# Redeploy
docker-compose -f docker-compose.yml --env-file .env.production up -d

# Verify
curl https://api.example.com/health
```

### 6.3 Kubernetes Rollback

```bash
# Rollback to previous revision
kubectl rollout undo deployment/rust-backend

# Check status
kubectl rollout status deployment/rust-backend

# Verify
kubectl exec -it <pod-name> -- curl localhost:8080/health
```

---

## 7. Monitoring

### 7.1 Metrics Endpoints

| Metric | Endpoint | Description |
|--------|----------|-------------|
| Health | /health | Service health |
| Metrics | /metrics | Prometheus metrics |
| Logs | stdout/stderr | Application logs |

### 7.2 Prometheus Configuration

```yaml
# prometheus.yml
scrape_configs:
  - job_name: 'rust-backend'
    static_configs:
      - targets: ['app:8080']
    scrape_interval: 10s

  - job_name: 'postgres'
    static_configs:
      - targets: ['db:5432']
    scrape_interval: 30s

  - job_name: 'redis'
    static_configs:
      - targets: ['redis:6379']
    scrape_interval: 30s
```

### 7.3 Alerting Rules

```yaml
# alerts.yml
groups:
- name: rust-backend
  rules:
  - alert: HighErrorRate
    expr: rate(http_requests_total{status=~"5.."}[5m]) > 0.05
    for: 5m
    labels:
      severity: critical
    annotations:
      summary: High error rate detected

  - alert: HighResponseTime
    expr: histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m])) > 1
    for: 5m
    labels:
      severity: warning
```

---

## 8. Backup & Recovery

### 8.1 Backup Strategy

| Type | Frequency | Retention | Method |
|------|-----------|-----------|--------|
| Full Database | Daily | 30 days | pg_dump |
| Incremental | Hourly | 7 days | WAL |
| Redis AOF | Every 1s | N/A | AOF |
| Config | Weekly | 90 days | File backup |
| Logs | Daily | 90 days | Logrotate |

### 8.2 Backup Commands

```bash
# Database backup
docker exec postgres pg_dump -U postgres app > backup_$(date +%Y%m%d).sql

# Redis backup
docker exec redis redis-cli BGSAVE

# Configuration backup
tar -czf config_backup_$(date +%Y%m%d).tar.gz .env* *.yml *.toml
```

### 8.3 Recovery Procedures

```bash
# Restore database
docker exec -i postgres psql -U postgres app < backup_20260311.sql

# Verify restoration
curl https://api.example.com/health

# Check data integrity
curl https://api.example.com/api/v1/users
```

### 8.4 Disaster Recovery

| Scenario | RTO | RPO | Procedure |
|----------|-----|-----|-----------|
| Database failure | 1 hour | 1 hour | Restore from backup |
| Application crash | 15 min | 0 | Auto-restart |
| Redis failure | 30 min | 5 min | Restart with AOF |
| DC failure | 4 hours | 1 hour | Restore from backup |

---

## Quick Reference

### Essential Commands

```bash
# Deploy to staging
docker-compose -f docker-compose.yml --env-file .env.staging up -d --build

# Deploy to production
docker-compose -f docker-compose.yml --env-file .env.production up -d --build

# Check status
docker-compose ps

# View logs
docker-compose logs -f

# Rollback
docker-compose -f docker-compose.yml down
docker pull rust-backend:<previous-tag>
docker-compose -f docker-compose.yml up -d
```

---

## Emergency Contacts

| Role | Contact |
|------|---------|
| DevOps Lead | devops@example.com |
| Security | security@example.com |
| On-CCall | oncall@example.com |

---

**Last Updated: 2026-03-11**
