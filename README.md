# ScrDesk PRO Enterprise

[![Version](https://img.shields.io/badge/version-1.0.0-blue.svg)](https://github.com/shosgoren/scrdesk/releases)
[![License](https://img.shields.io/badge/license-Proprietary-red.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/rust-1.91%2B-orange.svg)](https://www.rust-lang.org/)
[![Docker](https://img.shields.io/badge/docker-ready-brightgreen.svg)](https://www.docker.com/)

Professional, high-security, enterprise-grade remote desktop solution. Inspired by RustDesk, built from scratch with modern technologies.

## 🎯 Features

### Enterprise Level
- **Multi-Tenant Architecture**: Full tenant isolation
- **Microservices**: Scalable, flexible backend (11 services)
- **High Security**: JWT + 2FA, Argon2 password hashing, TLS 1.3 ready
- **2FA Support**: TOTP-based two-factor authentication
- **RBAC**: Role-based access control (Admin, User, Viewer)
- **Policy Engine**: Granular access policies (time restrictions, IP whitelist)
- **Audit Logging**: SOC2-compliant comprehensive audit trails

### Platform Support
- **Desktop**: macOS (ARM64 & Intel), Windows, Linux (x86_64)
- **Mobile**: Android, iOS (Planned for v2.0.0)
- **Web**: Admin panel (Next.js + React)

## 🏗️ Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     Desktop Clients                         │
│          (Windows, macOS, Linux)                           │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│                    Nginx (Reverse Proxy)                    │
│                    SSL/TLS Termination                      │
└────────────────────┬────────────────────────────────────────┘
                     │
        ┌────────────┼────────────┐
        ▼            ▼            ▼
┌──────────────┬──────────────┬──────────────┐
│   Auth       │   Device     │   Policy     │
│   Service    │   Manager    │   Engine     │
│   :8081      │   :8082      │   :8083      │
└──────┬───────┴──────┬───────┴──────┬───────┘
       │              │              │
       └──────────────┼──────────────┘
                      │
        ┌─────────────┼─────────────┐
        ▼             ▼             ▼
┌──────────────┬──────────────┬──────────────┐
│  PostgreSQL  │    Redis     │    Relay     │
│     :5432    │    :6379     │   Cluster    │
│              │              │    :21116    │
└──────────────┴──────────────┴──────────────┘
```

### Backend Microservices (Rust + Axum)
- **scrdesk-auth-service** (:8081) - Authentication (login, 2FA, JWT)
- **scrdesk-device-manager** (:8082) - Device management and approval
- **scrdesk-policy-engine** (:8083) - Access policies
- **scrdesk-core-server** (:8084) - Core orchestration
- **scrdesk-relay-cluster** (:8085) - Connection relay (RustDesk protocol)
- **scrdesk-audit-service** (:8086) - Audit logs
- **scrdesk-notification-service** (:8087) - Notifications
- **scrdesk-billing-service** (:8088) - Billing and usage tracking
- **scrdesk-admin-backend** (:8089) - Admin API
- **scrdesk-update-server** (:8090) - Automatic updates
- **scrdesk-analytics** (:8091) - Analytics and reporting
- **scrdesk-logging** (Internal) - Centralized logging

### Frontend
- **Admin Panel**: Next.js 15 + React 19 + TypeScript (Coming soon)
- **Desktop Client**: Rust + egui/eframe (v1.0.0 released)

### Infrastructure
- **PostgreSQL 16**: Primary database
- **Redis 7**: Caching and session management
- **Docker & Docker Compose**: Container orchestration
- **Nginx**: Reverse proxy and SSL termination

## 🚀 Quick Start

### Prerequisites
- **Docker** (20.10+) & **Docker Compose** (2.0+)
- **Git**
- **4GB RAM minimum** (8GB recommended)
- **Linux server** (Ubuntu 20.04+, Debian 11+, or CentOS 8+)

### Installation

#### 1. Clone Repository
```bash
git clone https://github.com/shosgoren/scrdesk.git
cd scrdesk/backend
```

#### 2. Configure Environment
```bash
cp .env.example .env
nano .env  # Update passwords, JWT secret, and other configurations
```

**Important:** Change these variables:
- `DATABASE_URL` - PostgreSQL connection string
- `POSTGRES_PASSWORD` - Database password
- `JWT_SECRET` - Random string (32+ characters)
- `SMTP_*` - Email configuration for password reset

#### 3. Start Services
```bash
# Build and start all services
docker compose up -d

# Wait for services to be healthy (2-5 minutes)
sleep 120

# Check status
docker compose ps

# View logs
docker compose logs -f
```

#### 4. Verify Deployment
```bash
# Check health endpoints
curl http://localhost:8081/health  # Auth service
curl http://localhost:8082/health  # Device manager
curl http://localhost:8083/health  # Policy engine

# Or use the health check script
./scripts/health-check.sh --verbose
```

#### 5. Access Services
- **Auth Service**: http://localhost:8081
- **Device Manager**: http://localhost:8082
- **Policy Engine**: http://localhost:8083
- **Core Server**: http://localhost:8084
- **Relay Cluster**: tcp://localhost:21116
- **Admin Backend**: http://localhost:8089

For production deployment with SSL/TLS, see [Deployment Guide](docs/DEPLOYMENT.md).

## 📝 Documentation

- **[API Documentation](docs/API.md)** - Complete API reference with examples
- **[Deployment Guide](docs/DEPLOYMENT.md)** - Production deployment instructions
- **[Release Notes](RELEASE_NOTES_v1.0.0.md)** - v1.0.0 release details

### Quick API Example

```bash
# Register user
curl -X POST http://localhost:8081/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "SecurePassword123!",
    "name": "John Doe"
  }'

# Login
curl -X POST http://localhost:8081/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "user@example.com",
    "password": "SecurePassword123!"
  }'

# Response
{
  "access_token": "eyJhbGci...",
  "refresh_token": "eyJhbGci...",
  "user": {
    "id": "uuid",
    "email": "user@example.com",
    "name": "John Doe"
  }
}

# List devices
curl -X GET http://localhost:8082/api/v1/devices \
  -H "Authorization: Bearer <access_token>"
```

For complete API documentation, see [docs/API.md](docs/API.md).

## 🖥️ Desktop Client

Download the latest desktop client from [Releases](https://github.com/shosgoren/scrdesk/releases).

### Installation

**macOS (Apple Silicon):**
```bash
curl -LO https://github.com/shosgoren/scrdesk/releases/download/v1.0.0/scrdesk-macos-arm64
chmod +x scrdesk-macos-arm64
./scrdesk-macos-arm64
```

**macOS (Intel):**
```bash
curl -LO https://github.com/shosgoren/scrdesk/releases/download/v1.0.0/scrdesk-macos-intel
chmod +x scrdesk-macos-intel
./scrdesk-macos-intel
```

**Windows:**
- Download `scrdesk-windows-x86_64.exe` from releases
- Run the installer

**Linux:**
```bash
curl -LO https://github.com/shosgoren/scrdesk/releases/download/v1.0.0/scrdesk-linux-x86_64
chmod +x scrdesk-linux-x86_64
./scrdesk-linux-x86_64
```

### Features
- Modern GUI with egui/eframe
- Full API integration (login, 2FA, device management)
- Connection state management
- Cross-platform support

## 🔐 Security

- **Encryption**: Argon2 password hashing
- **Transport**: TLS 1.3 ready, rustls
- **Authentication**: JWT (access + refresh tokens, RS256)
- **2FA**: TOTP (Google Authenticator compatible)
- **SQL Injection Protection**: SQLx compile-time verified queries
- **Rate Limiting**: Built-in on all endpoints
- **CORS**: Configurable cross-origin policies
- **Tenant Isolation**: Full tenant data isolation

## 📊 Database

PostgreSQL schema:
- `organizations` - Multi-tenant organizations
- `users` - Users with 2FA support
- `devices` - Registered devices
- `sessions` - Connection sessions
- `policies` - Access policies
- `groups` - User/Device groups
- `audit_logs` - Audit trail
- `refresh_tokens` - JWT refresh tokens

## 🧪 Development

### Backend
```bash
cd backend
cargo build
cargo test
cargo run --bin scrdesk-auth-service
```

### Desktop Client
```bash
cd client/desktop
cargo build --release

# Cross-platform builds
./build-release.sh
```

## 📦 Deployment

### Docker Compose (Recommended)
```bash
cd backend
docker compose up -d
```

See [Deployment Guide](docs/DEPLOYMENT.md) for production setup.

### Kubernetes
```bash
# Coming in v1.2.0
kubectl apply -f kubernetes/
```

## 🛠️ Monitoring & Health Checks

### Health Check Script

```bash
# Run health check
./backend/scripts/health-check.sh

# Verbose output
./backend/scripts/health-check.sh --verbose

# JSON output (for monitoring tools)
./backend/scripts/health-check.sh --json

# Send alerts on failures
./backend/scripts/health-check.sh --alert
```

### Real-time Dashboard

```bash
# Launch monitoring dashboard
./backend/scripts/monitor-dashboard.sh
```

Displays:
- Service status and health
- CPU and memory usage per container
- Container uptime
- System resources (disk, memory)
- Real-time updates every 2 seconds

## 🎯 Roadmap

### v1.0.0 (Released - December 2024) ✅
- [x] Microservices architecture (11 services)
- [x] Authentication & 2FA (JWT + TOTP)
- [x] Device management with approval workflow
- [x] Policy engine (time restrictions, IP whitelist)
- [x] Audit logging system
- [x] Desktop client (macOS ARM64 & Intel)
- [x] Complete REST API
- [x] Docker Compose deployment
- [x] Health check and monitoring tools
- [x] Comprehensive documentation

### v1.1.0 (Q1 2025)
- [ ] Screen recording and playback
- [ ] File transfer between devices
- [ ] Session history and analytics
- [ ] Desktop clients for Windows and Linux
- [ ] Advanced policy templates
- [ ] Multi-language support

### v1.2.0 (Q2 2025)
- [ ] Mobile clients (iOS, Android)
- [ ] WebRTC support
- [ ] Clipboard synchronization
- [ ] Advanced monitoring dashboard
- [ ] Kubernetes deployment

### v2.0.0 (Q3 2025)
- [ ] Multi-tenancy enhancements
- [ ] LDAP/Active Directory integration
- [ ] SAML SSO
- [ ] Advanced analytics and reporting
- [ ] Auto-scaling support

## 📞 Support

- **GitHub Issues**: https://github.com/shosgoren/scrdesk/issues
- **Discussions**: https://github.com/shosgoren/scrdesk/discussions
- **Documentation**: https://github.com/shosgoren/scrdesk/tree/main/docs
- **Email**: support@scrdesk.com

## 📄 License

Copyright (c) 2024 ScrDesk PRO Enterprise. All rights reserved.

This software is proprietary and confidential.

---

⭐ **Star this repository** to support the project!

🤖 Built with [Claude Code](https://claude.com/claude-code)
