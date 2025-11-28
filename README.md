# ScrDesk PRO Enterprise

Profesyonel, yüksek güvenlikli, dünya çapında ölçeklenebilir uzak masaüstü çözümü.

## Özellikler

- **Enterprise Seviye Yönetim**: Çoklu organizasyon (multi-tenant) desteği
- **Gelişmiş Relay Cluster**: Dağıtık relay node'ları ile düşük gecikme
- **Güçlü Güvenlik**: AES-256, RSA-4096, TLS 1.3, mTLS
- **Çapraz Platform**: Windows, macOS, Linux, Android, iOS
- **SaaS + Self-hosted**: Hibrit deployment modeli
- **Otomatik Güncelleme**: Zero-downtime güncellemeler
- **Session Recording**: Oturum kayıt ve oynatma
- **RBAC**: Rol tabanlı erişim kontrolü
- **Compliance**: GDPR, KVKK, SOC2 ready

## Mimari

### Backend Mikroservisler
- `scrdesk-core-server`: Ana sunucu
- `scrdesk-auth-service`: Kimlik doğrulama
- `scrdesk-device-manager`: Cihaz yönetimi
- `scrdesk-policy-engine`: Policy motoru
- `scrdesk-relay-cluster`: Relay yönetimi
- `scrdesk-audit-service`: Audit logging
- `scrdesk-notification-service`: Bildirimler
- `scrdesk-billing-service`: Faturalama
- `scrdesk-admin-backend`: Admin API
- `scrdesk-update-server`: Güncelleme sunucusu

### Frontend
- **Admin Panel**: Next.js 15 tabanlı
- **Desktop Client**: Rust + Flutter
- **Mobile Client**: Flutter (Android/iOS)

### Veritabanı
- PostgreSQL (Ana veritabanı)
- Redis (Cache & Sessions)
- S3/R2 (Session recordings)

## Kurulum

### Gereksinimler
- Docker & Docker Compose
- Kubernetes (production)
- PostgreSQL 15+
- Redis 7+
- Node.js 20+
- Rust 1.75+

### Geliştirme Ortamı

```bash
# Repository'yi klonla
git clone https://github.com/shosgoren/scrdesk.git
cd scrdesk

# Docker compose ile servisleri başlat
docker-compose up -d

# Backend servisleri başlat
cd backend
cargo run

# Admin panel'i başlat
cd admin-panel
npm install
npm run dev
```

## Lisans

AGPL-3.0

## Güvenlik

Güvenlik açıkları için lütfen: security@scrdesk.com
