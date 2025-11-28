# ScrDesk PRO Enterprise

Profesyonel, yüksek güvenlikli, enterprise-grade uzak masaüstü çözümü. RustDesk'ten ilham alınarak sıfırdan geliştirilmiştir.

## 🎯 Özellikler

### Enterprise Seviye
- **Multi-Tenant Mimari**: Tam izolasyonlu organizasyon yönetimi
- **Mikroservis Yapısı**: Ölçeklenebilir, esnek backend
- **Yüksek Güvenlik**: AES-256, RSA-4096, TLS 1.3, mTLS
- **2FA Desteği**: TOTP ile iki faktörlü kimlik doğrulama
- **RBAC**: Rol tabanlı erişim kontrolü
- **Policy Engine**: Granüler erişim politikaları
- **Audit Logging**: SOC2 uyumlu tam denetim kaydı

### Platform Desteği
- Windows, macOS, Linux (Desktop)
- Android, iOS (Mobile) - Yapım aşamasında

## 🏗️ Mimari

### Backend Mikroservisler (Rust)
- `scrdesk-auth-service` - Kimlik doğrulama (login, 2FA, JWT)
- `scrdesk-device-manager` - Cihaz yönetimi ve onay
- `scrdesk-policy-engine` - Erişim politikaları
- `scrdesk-audit-service` - Denetim günlükleri
- `scrdesk-admin-backend` - Admin API
- `scrdesk-relay-cluster` - Bağlantı relay'i
- `scrdesk-notification-service` - Bildirimler
- `scrdesk-billing-service` - Faturalama (Stripe)
- `scrdesk-update-server` - Otomatik güncellemeler
- `scrdesk-core-server` - API Gateway

### Frontend
- **Admin Panel**: Next.js 15 + React 19 + TypeScript

### Altyapı
- PostgreSQL 16+ (Ana veritabanı)
- Redis 7+ (Cache & Sessions)
- MinIO / S3 (Session recordings)
- Docker & Docker Compose

## 🚀 Hızlı Başlangıç

### Gereksinimler
- Docker & Docker Compose
- (Opsiyonel) Rust 1.75+, Node.js 20+

### Kurulum

1. **Repository'yi klonla**
```bash
git clone https://github.com/shosgoren/scrdesk.git
cd scrdesk
```

2. **Environment variables**
```bash
cp .env.example .env
# .env dosyasını düzenleyin
```

3. **Docker ile başlat**
```bash
docker-compose up -d
```

4. **Servislere erişim**
- Admin Panel: http://localhost:3000
- Core API: http://localhost:8000
- Auth Service: http://localhost:8001
- Device Manager: http://localhost:8002
- Policy Engine: http://localhost:8003
- Audit Service: http://localhost:8005
- Admin Backend: http://localhost:8006
- Relay Server: tcp://localhost:21117

## 📝 API Dokümantasyonu

### Auth Service (Port 8001)
- `POST /api/v1/auth/register` - Kullanıcı kaydı
- `POST /api/v1/auth/login` - Giriş
- `POST /api/v1/auth/2fa/enable` - 2FA aktifleştir
- `POST /api/v1/auth/refresh` - Token yenile
- `POST /api/v1/auth/logout` - Çıkış

### Device Manager (Port 8002)
- `POST /api/v1/devices` - Cihaz kaydet
- `GET /api/v1/devices` - Cihazları listele
- `POST /api/v1/devices/:id/approve` - Cihaz onayla
- `POST /api/v1/devices/:id/heartbeat` - Heartbeat

### Policy Engine (Port 8003)
- `POST /api/v1/policies` - Policy oluştur
- `GET /api/v1/policies` - Policy'leri listele
- `POST /api/v1/policies/check` - Policy kontrol et

### Audit Service (Port 8005)
- `GET /api/v1/audit-logs` - Audit logları listele
- `GET /api/v1/audit-logs/export` - Logları dışa aktar

## 🔐 Güvenlik

- **Şifreleme**: AES-256, RSA-4096
- **Transport**: TLS 1.3, mTLS desteği
- **Authentication**: JWT (access + refresh tokens)
- **2FA**: TOTP (Google Authenticator uyumlu)
- **Password**: BCrypt hashing
- **SQL Injection**: Parameterized queries
- **Tenant Isolation**: Her tenant tamamen izole

## 📊 Veritabanı

PostgreSQL şeması:
- `tenants` - Organizasyonlar
- `users` - Kullanıcılar (2FA desteği)
- `devices` - Kayıtlı cihazlar
- `sessions` - Bağlantı oturumları
- `policies` - Erişim politikaları
- `groups` - Kullanıcı/Cihaz grupları
- `audit_logs` - Denetim kayıtları
- `refresh_tokens` - JWT refresh tokens

## 🧪 Geliştirme

### Backend
```bash
cd backend
cargo build
cargo test
cargo run --bin scrdesk-auth-service
```

### Admin Panel
```bash
cd admin-panel
npm install
npm run dev
```

## 📦 Deployment

### Docker
```bash
docker-compose up -d
```

### Kubernetes
```bash
# Coming soon
kubectl apply -f kubernetes/
```

## 🤝 Katkıda Bulunma

Bu proje AGPL-3.0 lisansı altındadır. Katkılarınızı bekliyoruz!

## 📄 Lisans

AGPL-3.0 - Detaylar için LICENSE dosyasına bakın.

## 🎯 Roadmap

- [x] Mikroservis mimarisi
- [x] Auth & 2FA
- [x] Device management
- [x] Policy engine
- [x] Audit logging
- [x] Admin panel temel
- [ ] Desktop clients (Windows, macOS, Linux)
- [ ] Mobile clients (Android, iOS)
- [ ] Session recording
- [ ] Real-time monitoring
- [ ] Stripe billing integration
- [ ] Kubernetes deployment

## 📞 İletişim

- GitHub: https://github.com/shosgoren/scrdesk
- Issues: https://github.com/shosgoren/scrdesk/issues

---

⭐ **Star** vererek projeyi destekleyebilirsiniz!

🤖 Built with [Claude Code](https://claude.ai/claude-code)
