# ScrDesk PRO Enterprise - Deployment Guide

## Table of Contents

1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Quick Start](#quick-start)
4. [Detailed Setup](#detailed-setup)
5. [Configuration](#configuration)
6. [SSL/TLS Setup](#ssltls-setup)
7. [Monitoring](#monitoring)
8. [Backup and Recovery](#backup-and-recovery)
9. [Troubleshooting](#troubleshooting)
10. [Production Best Practices](#production-best-practices)

---

## Overview

ScrDesk PRO Enterprise is deployed as a containerized microservices architecture using Docker and Docker Compose. This guide covers deployment on a Linux server (Ubuntu 20.04+ or Debian 11+ recommended).

### Architecture Components

- **11 Backend Microservices** (Rust/Axum)
- **PostgreSQL 16** (Primary database)
- **Redis 7** (Caching and sessions)
- **Nginx** (Reverse proxy)
- **Docker Compose** (Orchestration)

---

## Prerequisites

### System Requirements

**Minimum:**
- CPU: 4 cores
- RAM: 4GB
- Storage: 20GB SSD
- OS: Ubuntu 20.04+, Debian 11+, or CentOS 8+

**Recommended:**
- CPU: 8 cores
- RAM: 8GB
- Storage: 50GB SSD
- OS: Ubuntu 22.04 LTS

### Software Requirements

1. **Docker** (version 20.10+)
2. **Docker Compose** (version 2.0+)
3. **Git**
4. **OpenSSL** (for SSL certificates)

---

## Quick Start

For a quick deployment on a fresh server:

```bash
# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh
sudo usermod -aG docker $USER

# Clone repository
git clone https://github.com/shosgoren/scrdesk.git
cd scrdesk/backend

# Configure environment
cp .env.example .env
nano .env  # Update passwords and secrets

# Start services
docker compose up -d

# Check status
docker compose ps

# View logs
docker compose logs -f
```

Your services should now be running on:
- Auth Service: http://localhost:8081
- Device Manager: http://localhost:8082
- And so on...

---

## Detailed Setup

### Step 1: Server Preparation

#### Update System

```bash
sudo apt update && sudo apt upgrade -y
```

#### Install Dependencies

```bash
# Install required packages
sudo apt install -y git curl wget vim ufw

# Install Docker
curl -fsSL https://get.docker.com -o get-docker.sh
sudo sh get-docker.sh

# Add user to docker group
sudo usermod -aG docker $USER

# Log out and log back in for group changes to take effect
```

#### Configure Firewall

```bash
# Allow SSH
sudo ufw allow 22/tcp

# Allow HTTP and HTTPS
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp

# Allow RustDesk relay ports
sudo ufw allow 21115/tcp
sudo ufw allow 21116/tcp
sudo ufw allow 21117/tcp
sudo ufw allow 21118/tcp
sudo ufw allow 21119/tcp

# Enable firewall
sudo ufw enable
```

### Step 2: Clone Repository

```bash
# Create application directory
sudo mkdir -p /opt/scrdesk
sudo chown $USER:$USER /opt/scrdesk

# Clone repository
cd /opt
git clone https://github.com/shosgoren/scrdesk.git
cd scrdesk/backend
```

### Step 3: Environment Configuration

```bash
# Copy example environment file
cp .env.example .env

# Edit configuration
nano .env
```

**Important variables to configure:**

```bash
# Database
DATABASE_URL=postgresql://scrdesk_user:CHANGE_THIS_PASSWORD@postgres:5432/scrdesk
POSTGRES_PASSWORD=CHANGE_THIS_PASSWORD
POSTGRES_USER=scrdesk_user
POSTGRES_DB=scrdesk

# Redis
REDIS_URL=redis://redis:6379
REDIS_PASSWORD=  # Leave empty or set a password

# JWT
JWT_SECRET=CHANGE_THIS_TO_A_RANDOM_STRING_AT_LEAST_32_CHARS
JWT_ACCESS_EXPIRY=900  # 15 minutes
JWT_REFRESH_EXPIRY=604800  # 7 days

# Server Configuration
SERVER_HOST=0.0.0.0

# Email (for password reset)
SMTP_HOST=smtp.gmail.com
SMTP_PORT=587
SMTP_USERNAME=your-email@gmail.com
SMTP_PASSWORD=your-app-password
SMTP_FROM=noreply@yourdomain.com

# Environment
RUST_LOG=info
ENVIRONMENT=production
```

#### Generate Secure Secrets

```bash
# Generate JWT secret
openssl rand -base64 32

# Generate database password
openssl rand -base64 24
```

### Step 4: Build and Start Services

```bash
# Build images (first time only)
docker compose build

# Start services in detached mode
docker compose up -d

# Wait for services to be healthy (may take 2-5 minutes)
sleep 120

# Check status
docker compose ps
```

Expected output:
```
NAME                           STATUS              PORTS
scrdesk-admin-backend          Up                  0.0.0.0:8089->8089/tcp
scrdesk-analytics              Up                  0.0.0.0:8091->8091/tcp
scrdesk-audit-service          Up                  0.0.0.0:8086->8086/tcp
scrdesk-auth-service           Up                  0.0.0.0:8081->8081/tcp
scrdesk-billing-service        Up                  0.0.0.0:8088->8088/tcp
scrdesk-core-server            Up                  0.0.0.0:8084->8084/tcp
scrdesk-device-manager         Up                  0.0.0.0:8082->8082/tcp
scrdesk-logging                Up                  (internal)
scrdesk-notification-service   Up                  0.0.0.0:8087->8087/tcp
scrdesk-policy-engine          Up                  0.0.0.0:8083->8083/tcp
scrdesk-relay-cluster          Up                  0.0.0.0:21116->21116/tcp
scrdesk-update-server          Up                  0.0.0.0:8090->8090/tcp
postgres                       Up (healthy)        0.0.0.0:5432->5432/tcp
redis                          Up (healthy)        0.0.0.0:6379->6379/tcp
```

### Step 5: Verify Deployment

```bash
# Check health endpoints
curl http://localhost:8081/health  # Auth service
curl http://localhost:8082/health  # Device manager
curl http://localhost:8083/health  # Policy engine

# View logs
docker compose logs -f scrdesk-auth-service

# Check database
docker compose exec postgres psql -U scrdesk_user -d scrdesk -c "\dt"

# Check Redis
docker compose exec redis redis-cli ping
```

---

## Configuration

### Database Configuration

The PostgreSQL database is automatically initialized with migrations. To manually run migrations:

```bash
# Connect to database
docker compose exec postgres psql -U scrdesk_user -d scrdesk

# View tables
\dt

# View users
SELECT * FROM users;
```

### Redis Configuration

Redis is used for:
- Session storage
- JWT token blacklisting
- Rate limiting
- Caching

To monitor Redis:

```bash
# Connect to Redis CLI
docker compose exec redis redis-cli

# Check keys
KEYS *

# Monitor commands
MONITOR

# Get info
INFO
```

### Service Ports

| Service | Internal Port | External Port | Description |
|---------|---------------|---------------|-------------|
| auth-service | 8081 | 8081 | Authentication endpoints |
| device-manager | 8082 | 8082 | Device management |
| policy-engine | 8083 | 8083 | Policy enforcement |
| core-server | 8084 | 8084 | Core orchestration |
| relay-cluster | 21116 | 21116 | RustDesk relay |
| audit-service | 8086 | 8086 | Audit logging |
| notification-service | 8087 | 8087 | Notifications |
| billing-service | 8088 | 8088 | Billing and usage |
| admin-backend | 8089 | 8089 | Admin panel API |
| update-server | 8090 | 8090 | Client updates |
| analytics | 8091 | 8091 | Analytics API |
| postgres | 5432 | 5432 | PostgreSQL database |
| redis | 6379 | 6379 | Redis cache |

---

## SSL/TLS Setup

### Option 1: Nginx Reverse Proxy with Let's Encrypt

Install and configure Nginx:

```bash
# Install Nginx and Certbot
sudo apt install -y nginx certbot python3-certbot-nginx

# Create Nginx configuration
sudo nano /etc/nginx/sites-available/scrdesk
```

Add configuration:

```nginx
# HTTP - Redirect to HTTPS
server {
    listen 80;
    server_name api.yourdomain.com;

    location /.well-known/acme-challenge/ {
        root /var/www/html;
    }

    location / {
        return 301 https://$server_name$request_uri;
    }
}

# HTTPS
server {
    listen 443 ssl http2;
    server_name api.yourdomain.com;

    ssl_certificate /etc/letsencrypt/live/api.yourdomain.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/api.yourdomain.com/privkey.pem;

    # SSL configuration
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers HIGH:!aNULL:!MD5;
    ssl_prefer_server_ciphers on;

    # Security headers
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
    add_header X-Frame-Options "SAMEORIGIN" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;

    # Auth Service
    location /api/v1/auth {
        proxy_pass http://localhost:8081;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # Device Manager
    location /api/v1/devices {
        proxy_pass http://localhost:8082;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # Policy Engine
    location /api/v1/policies {
        proxy_pass http://localhost:8083;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }

    # Add more services as needed...
}
```

Enable and obtain certificate:

```bash
# Enable site
sudo ln -s /etc/nginx/sites-available/scrdesk /etc/nginx/sites-enabled/

# Test configuration
sudo nginx -t

# Obtain SSL certificate
sudo certbot --nginx -d api.yourdomain.com

# Reload Nginx
sudo systemctl reload nginx

# Setup auto-renewal
sudo systemctl enable certbot.timer
```

### Option 2: Cloudflare with Origin Certificates

1. Add your domain to Cloudflare
2. Generate origin certificate in Cloudflare dashboard
3. Configure Nginx with origin certificate
4. Enable "Full (strict)" SSL mode in Cloudflare

---

## Monitoring

### Health Check Script

Use the provided health check script:

```bash
# Run health check
./backend/scripts/health-check.sh

# Verbose output
./backend/scripts/health-check.sh --verbose

# JSON output (for monitoring tools)
./backend/scripts/health-check.sh --json

# Send alerts
./backend/scripts/health-check.sh --alert
```

### Real-time Dashboard

Launch the monitoring dashboard:

```bash
./backend/scripts/monitor-dashboard.sh
```

This displays:
- Service status (running/stopped)
- CPU and memory usage per service
- Container uptime
- Health check status
- System resources (disk, memory)

### Setup Cron Job for Health Checks

```bash
# Edit crontab
crontab -e

# Add health check every 5 minutes
*/5 * * * * /opt/scrdesk/backend/scripts/health-check.sh --json >> /var/log/scrdesk/health.log 2>&1

# Alert on failures every hour
0 * * * * /opt/scrdesk/backend/scripts/health-check.sh --alert
```

### Docker Logs

```bash
# View all logs
docker compose logs -f

# View specific service
docker compose logs -f scrdesk-auth-service

# View last 100 lines
docker compose logs --tail=100 scrdesk-device-manager

# Follow logs with timestamp
docker compose logs -f --timestamps
```

### Prometheus and Grafana (Advanced)

For production monitoring, consider setting up Prometheus and Grafana:

```bash
# Add to docker-compose.yml
prometheus:
  image: prom/prometheus
  volumes:
    - ./monitoring/prometheus.yml:/etc/prometheus/prometheus.yml
    - prometheus_data:/prometheus
  ports:
    - "9090:9090"

grafana:
  image: grafana/grafana
  volumes:
    - grafana_data:/var/lib/grafana
  ports:
    - "3000:3000"
  environment:
    - GF_SECURITY_ADMIN_PASSWORD=admin
```

---

## Backup and Recovery

### Database Backup

Create automated backups:

```bash
# Create backup script
cat > /opt/scrdesk/backup.sh << 'EOF'
#!/bin/bash
DATE=$(date +%Y%m%d_%H%M%S)
BACKUP_DIR="/opt/scrdesk/backups"
mkdir -p $BACKUP_DIR

# Backup PostgreSQL
docker compose exec -T postgres pg_dump -U scrdesk_user scrdesk | gzip > $BACKUP_DIR/scrdesk_db_$DATE.sql.gz

# Backup Redis
docker compose exec -T redis redis-cli --rdb /data/dump.rdb
docker cp scrdesk-redis:/data/dump.rdb $BACKUP_DIR/redis_$DATE.rdb

# Keep only last 7 days
find $BACKUP_DIR -name "*.sql.gz" -mtime +7 -delete
find $BACKUP_DIR -name "*.rdb" -mtime +7 -delete

echo "Backup completed: $DATE"
EOF

chmod +x /opt/scrdesk/backup.sh

# Schedule daily backups at 2 AM
crontab -e
# Add: 0 2 * * * /opt/scrdesk/backup.sh >> /var/log/scrdesk/backup.log 2>&1
```

### Restore from Backup

```bash
# Restore PostgreSQL
gunzip < backup.sql.gz | docker compose exec -T postgres psql -U scrdesk_user -d scrdesk

# Restore Redis
docker cp backup.rdb scrdesk-redis:/data/dump.rdb
docker compose restart redis
```

### Configuration Backup

```bash
# Backup .env and compose files
tar -czf config_backup_$(date +%Y%m%d).tar.gz .env docker-compose.yml
```

---

## Troubleshooting

### Services Won't Start

```bash
# Check logs
docker compose logs

# Check individual service
docker compose logs scrdesk-auth-service

# Restart service
docker compose restart scrdesk-auth-service

# Rebuild and restart
docker compose up -d --build scrdesk-auth-service
```

### Database Connection Issues

```bash
# Check database is running
docker compose ps postgres

# Test connection
docker compose exec postgres pg_isready -U scrdesk_user

# View connections
docker compose exec postgres psql -U scrdesk_user -d scrdesk -c "SELECT * FROM pg_stat_activity;"

# Check DATABASE_URL in .env
cat .env | grep DATABASE_URL
```

### Redis Connection Issues

```bash
# Check Redis is running
docker compose ps redis

# Test connection
docker compose exec redis redis-cli ping

# View Redis logs
docker compose logs redis

# Check memory
docker compose exec redis redis-cli info memory
```

### High Memory Usage

```bash
# Check container memory
docker stats

# Restart services
docker compose restart

# Clear Redis cache
docker compose exec redis redis-cli FLUSHALL
```

### Port Already in Use

```bash
# Find process using port
sudo lsof -i :8081

# Kill process
sudo kill -9 <PID>

# Or change port in docker-compose.yml
```

### SSL Certificate Issues

```bash
# Renew certificate
sudo certbot renew

# Test renewal
sudo certbot renew --dry-run

# View certificate info
sudo certbot certificates
```

### Build Failures

```bash
# Clean Docker cache
docker system prune -a

# Remove volumes
docker compose down -v

# Rebuild from scratch
docker compose build --no-cache
docker compose up -d
```

---

## Production Best Practices

### Security

1. **Change Default Passwords**
   - Update all passwords in `.env`
   - Use strong, unique passwords
   - Store secrets securely (consider HashiCorp Vault)

2. **Enable Firewall**
   - Only expose necessary ports
   - Use VPN for administrative access
   - Implement rate limiting

3. **SSL/TLS**
   - Always use HTTPS in production
   - Keep certificates up to date
   - Use TLS 1.2+ only

4. **Regular Updates**
   ```bash
   # Update Docker images
   docker compose pull
   docker compose up -d

   # Update system packages
   sudo apt update && sudo apt upgrade -y
   ```

5. **2FA for Admin Accounts**
   - Enable 2FA for all admin users
   - Enforce strong password policies
   - Regular security audits

### Performance

1. **Resource Limits**

   Add to `docker-compose.yml`:
   ```yaml
   services:
     scrdesk-auth-service:
       deploy:
         resources:
           limits:
             cpus: '1.0'
             memory: 512M
           reservations:
             cpus: '0.5'
             memory: 256M
   ```

2. **Database Optimization**
   ```sql
   -- Add indexes
   CREATE INDEX idx_devices_user_id ON devices(user_id);
   CREATE INDEX idx_audit_logs_timestamp ON audit_logs(timestamp);

   -- Regular vacuum
   VACUUM ANALYZE;
   ```

3. **Redis Configuration**
   ```bash
   # Set max memory
   docker compose exec redis redis-cli CONFIG SET maxmemory 2gb
   docker compose exec redis redis-cli CONFIG SET maxmemory-policy allkeys-lru
   ```

### Monitoring

1. **Set Up Alerts**
   - Disk space alerts (80% threshold)
   - Memory usage alerts (85% threshold)
   - Service down alerts
   - Failed login attempts

2. **Log Aggregation**
   - Consider ELK stack (Elasticsearch, Logstash, Kibana)
   - Or Loki + Grafana
   - Centralized logging for debugging

3. **Metrics Collection**
   - Prometheus for metrics
   - Grafana for dashboards
   - Custom business metrics

### Backup

1. **Automated Backups**
   - Daily database backups
   - Weekly full system backups
   - Test restore procedures regularly

2. **Off-site Backups**
   ```bash
   # Upload to S3
   aws s3 cp backup.sql.gz s3://your-bucket/backups/

   # Or use rsync to remote server
   rsync -avz backups/ user@backup-server:/backups/scrdesk/
   ```

### Scaling

1. **Horizontal Scaling**
   - Run multiple instances behind load balancer
   - Use Docker Swarm or Kubernetes
   - Separate read replicas for database

2. **Vertical Scaling**
   - Increase server resources
   - Optimize database queries
   - Add caching layers

### Documentation

1. **Keep Documentation Updated**
   - Document custom configurations
   - Maintain runbooks for common issues
   - Update API documentation

2. **Change Management**
   - Track all changes in git
   - Use semantic versioning
   - Maintain changelog

---

## Next Steps

After deployment:

1. **Create Admin User**
   ```bash
   curl -X POST http://your-domain/api/v1/auth/register \
     -H "Content-Type: application/json" \
     -d '{
       "email": "admin@yourdomain.com",
       "password": "SecurePassword123!",
       "name": "Admin User"
     }'
   ```

2. **Enable 2FA for Admin**
   - Login to admin panel
   - Navigate to Security settings
   - Enable 2FA

3. **Configure Policies**
   - Set up time restrictions
   - Configure IP whitelists
   - Define user roles

4. **Test Desktop Client**
   - Download client from releases
   - Configure server URL
   - Test connection

5. **Setup Monitoring**
   - Install monitoring tools
   - Configure alerts
   - Test notification channels

---

## Support

For issues and questions:

- **GitHub Issues**: https://github.com/shosgoren/scrdesk/issues
- **Documentation**: https://github.com/shosgoren/scrdesk/docs
- **Email**: support@scrdesk.com

---

## License

Copyright (c) 2024 ScrDesk PRO Enterprise. All rights reserved.
