# SCRDESK Development Roadmap & Progress Tracker

## 📊 Proje Durumu

**Son Güncelleme**: 2025-11-17
**Mevcut Versiyon**: v1.0.0-alpha
**Durum**: Initial Fork & Rebranding Tamamlandı

---

## ✅ Tamamlanan Görevler (v1.0.0-alpha)

### 2025-11-17: Initial Setup
- [x] RustDesk kaynak kodunu fork etme
- [x] Proje adını "SCRDESK" olarak değiştirme
- [x] Cargo.toml'da tüm metadata güncelleme
  - Package name: `scrdesk`
  - Library name: `libscrdesk`
  - Bundle identifier: `com.scrdesk.scrdesk`
  - Copyright bilgileri
- [x] Türkçe dokümantasyon oluşturma
  - README.md (İngilizce)
  - QUICKSTART.md (Türkçe)
  - SERVER_SETUP.md (Türkçe)
  - BUILD_GUIDE.md (Türkçe)
  - PROJECT_SUMMARY.md (Türkçe)
- [x] Docker Compose yapılandırması
- [x] .gitignore güncellemeleri
- [x] GitHub repository kurulumu

---

## 🚧 Devam Eden Görevler

### Hiçbiri (Şu an için)

---

## 📋 Öncelikli Yapılacaklar (v1.0.0-beta)

### 1. Branding & UI Değişiklikleri
**Öncelik**: 🔴 Yüksek
**Tahmini Süre**: 1-2 gün

- [ ] **Logo ve Icon Değişiklikleri**
  - [ ] `res/32x32.png` - Yeni SCRDESK logosu
  - [ ] `res/128x128.png` - Yeni SCRDESK logosu
  - [ ] `res/128x128@2x.png` - Yeni SCRDESK logosu (retina)
  - [ ] `res/icon.svg` - Vector logo
  - [ ] Windows icon (.ico dosyası)
  - [ ] macOS icon (.icns dosyası)

  **Not**: Logo dosyaları hazırlanmalı. Grafik tasarımcı ile çalışılabilir.

- [ ] **Uygulama İçi Metin Değişiklikleri**
  - [ ] Kaynak kodda "RustDesk" string'lerini bul ve değiştir
  - [ ] Özellikle kontrol edilmesi gereken dosyalar:
    - `src/main.rs`
    - `src/ui/` altındaki tüm dosyalar
    - `flutter/lib/` altındaki Dart dosyaları
    - `src/lang/` dil dosyaları

  **Komut**:
  ```bash
  # "RustDesk" geçen tüm yerleri bul
  grep -r "RustDesk" src/ flutter/lib/ --exclude-dir=target
  ```

- [ ] **Başlangıç Splash Screen**
  - [ ] Flutter splash screen güncelleme
  - [ ] Android: `flutter/android/app/src/main/res/drawable/launch_background.xml`
  - [ ] iOS: `flutter/ios/Runner/Assets.xcassets/LaunchImage.imageset/`

### 2. Default Server Yapılandırması
**Öncelik**: 🟡 Orta
**Tahmini Süre**: 2-3 saat

- [ ] **Varsayılan Sunucu Ayarları**
  - [ ] `src/config.rs` dosyasında default server URL'si
  - [ ] Compile-time'da sunucu adresi embed etme
  - [ ] Environment variable ile override seçeneği

  **Örnek**:
  ```rust
  // src/config.rs
  pub const DEFAULT_ID_SERVER: &str = env!("SCRDESK_ID_SERVER", "your-server.com:21116");
  pub const DEFAULT_RELAY_SERVER: &str = env!("SCRDESK_RELAY_SERVER", "your-server.com:21117");
  ```

- [ ] **Public Key Embedding**
  - [ ] Sunucu public key'ini binary'e gömme
  - [ ] Compile sırasında otomatik key alma

### 3. Kaynak Kod İyileştirmeleri
**Öncelik**: 🟢 Düşük
**Tahmini Süre**: 3-5 gün

- [ ] **Namespace Değişiklikleri**
  - [ ] `libs/hbb_common/` içinde "rustdesk" string'lerini değiştir
  - [ ] Package name'leri güncelle
  - [ ] Import path'leri düzelt

- [ ] **Gereksiz Özellikleri Kaldırma** (Opsiyonel)
  - [ ] RustDesk Pro özelliklerini kaldır
  - [ ] Ticari sunucu bağlantılarını temizle
  - [ ] Gereksiz telemetry kodlarını çıkar

- [ ] **Test Suite Güncelleme**
  - [ ] Mevcut testleri çalıştır: `cargo test`
  - [ ] Başarısız testleri düzelt
  - [ ] SCRDESK'e özel test senaryoları ekle

### 4. Build Otomasyonu
**Öncelik**: 🟡 Orta
**Tahmini Süre**: 1 gün

- [ ] **GitHub Actions CI/CD**
  - [ ] `.github/workflows/build-linux.yml`
  - [ ] `.github/workflows/build-windows.yml`
  - [ ] `.github/workflows/build-macos.yml`
  - [ ] `.github/workflows/build-android.yml`
  - [ ] `.github/workflows/build-ios.yml`

- [ ] **Automated Release**
  - [ ] Tag push'ta otomatik build
  - [ ] GitHub Releases'a binary yükleme
  - [ ] Changelog otomatik oluşturma

- [ ] **Docker Build Image**
  - [ ] Multi-platform build container
  - [ ] Cross-compilation desteği

### 5. Sunucu Yönetim Araçları
**Öncelik**: 🟡 Orta
**Tahmini Süre**: 3-4 gün

- [ ] **Kurulum Scriptleri**
  - [ ] `scripts/install-server.sh` - Ubuntu/Debian
  - [ ] `scripts/install-server-centos.sh` - CentOS/RHEL
  - [ ] `scripts/update-server.sh` - Sunucu güncelleme
  - [ ] `scripts/backup-keys.sh` - Key yedekleme

- [ ] **Monitoring Dashboard** (Gelecek Versiyon)
  - [ ] Web-based admin panel
  - [ ] Aktif bağlantıları gösterme
  - [ ] Log viewer
  - [ ] Kullanıcı yönetimi

### 6. Dokümantasyon İyileştirmeleri
**Öncelik**: 🟢 Düşük
**Tahmini Süre**: 1-2 gün

- [ ] **Video Eğitimler**
  - [ ] YouTube: Sunucu kurulum videosu
  - [ ] YouTube: Client kullanım videosu
  - [ ] YouTube: Build rehberi videosu

- [ ] **Wiki Sayfaları**
  - [ ] GitHub Wiki kurulumu
  - [ ] FAQ bölümü
  - [ ] Troubleshooting guide
  - [ ] Security best practices

- [ ] **Çoklu Dil Desteği**
  - [ ] İngilizce dokümantasyon
  - [ ] Türkçe (mevcut)
  - [ ] README çevirilerini güncelle

---

## 🔮 Gelecek Versiyonlar

### v1.1.0 - Enhanced Features
**Tahmini Tarih**: 2025 Q1

- [ ] Web client desteği
- [ ] Mobile push notifications
- [ ] Dark mode theme
- [ ] Custom branding options (runtime)
- [ ] Multi-user session support

### v1.2.0 - Enterprise Features
**Tahmini Tarih**: 2025 Q2

- [ ] LDAP/Active Directory entegrasyonu
- [ ] SSO (Single Sign-On) desteği
- [ ] Audit logging
- [ ] Role-based access control (RBAC)
- [ ] Session recording

### v2.0.0 - Major Rewrite
**Tahmini Tarih**: 2025 Q3

- [ ] Microservices architecture
- [ ] Kubernetes deployment
- [ ] gRPC protocol desteği
- [ ] WebRTC peer-to-peer modu
- [ ] End-to-end encryption upgrade

---

## 🐛 Bilinen Sorunlar & Bug Tracker

### Kritik
- Hiçbiri (şu an için)

### Orta Öncelikli
- [ ] iOS build test edilmeli (App Store politikaları)
- [ ] Android APK signing yapılandırması gerekli
- [ ] macOS notarization süreci kurulmalı

### Düşük Öncelikli
- [ ] Bazı dil dosyalarında RustDesk referansları kalıyor
- [ ] Build süreleri uzun (optimize edilebilir)

---

## 📝 Değişiklik Geçmişi (Changelog)

### [1.0.0-alpha] - 2025-11-17

#### Added
- İlk SCRDESK fork'u oluşturuldu
- Türkçe dokümantasyon ekletildi (QUICKSTART, SERVER_SETUP, BUILD_GUIDE)
- Docker Compose yapılandırması
- GitHub repository kurulumu
- Development roadmap

#### Changed
- Proje adı "rustdesk" → "scrdesk"
- Package metadata güncellendi
- Bundle identifier değiştirildi
- Copyright bilgileri güncellendi

#### Removed
- Upstream RustDesk git remote'u kaldırıldı

---

## 🎯 Katkı Rehberi

### Yeni Özellik Eklemeden Önce Kontrol Listesi

1. **Mevcut Kodu Analiz Et**
   ```bash
   # İlgili dosyaları bul
   find src/ -name "*.rs" | xargs grep -l "ilgili_konu"

   # Test et
   cargo test --lib --bins
   ```

2. **Değişiklikleri Belge**
   - Bu dosyayı (DEVELOPMENT_ROADMAP.md) güncelle
   - CHANGELOG.md'ye ekle
   - Kod içine comment ekle

3. **Test Yaz**
   - Unit tests
   - Integration tests
   - Manual test senaryoları

4. **Build Kontrol**
   ```bash
   cargo build --release
   cargo clippy -- -D warnings
   cargo fmt --check
   ```

5. **Commit Kuralları**
   ```
   <type>(<scope>): <subject>

   Types: feat, fix, docs, style, refactor, test, chore
   Örnek: feat(ui): add custom SCRDESK logo
   ```

---

## 📊 İlerleme Metrikleri

### Tamamlanma Oranları
- **Rebranding**: ████████░░ 80%
- **Dokümantasyon**: ██████████ 100%
- **Build System**: ████░░░░░░ 40%
- **Testing**: ██░░░░░░░░ 20%
- **Deployment**: ███░░░░░░░ 30%

**Genel İlerleme**: █████░░░░░ 50%

---

## 🔗 Faydalı Linkler

- **GitHub Repository**: https://github.com/shosgoren/scrdesk
- **RustDesk Upstream**: https://github.com/rustdesk/rustdesk
- **RustDesk Server**: https://github.com/rustdesk/rustdesk-server
- **Rust Docs**: https://doc.rust-lang.org/
- **Flutter Docs**: https://flutter.dev/docs

---

## 📞 Geliştirici Notları

### Önemli Dosyalar (Değişiklik Yaparken Dikkat!)

| Dosya | Açıklama | Kritiklik |
|-------|----------|-----------|
| `Cargo.toml` | Proje metadata, bağımlılıklar | 🔴 Yüksek |
| `src/main.rs` | Ana giriş noktası | 🔴 Yüksek |
| `src/server/` | Sunucu servisleri | 🔴 Yüksek |
| `src/client.rs` | Client bağlantı yönetimi | 🔴 Yüksek |
| `flutter/lib/` | UI kodu | 🟡 Orta |
| `libs/hbb_common/` | Ortak kütüphaneler | 🔴 Yüksek |
| `build.py` | Build scripti | 🟡 Orta |
| `res/` | Logo, icon dosyaları | 🟢 Düşük |

### Build Zamanı Değişkenler

```bash
# Custom server ile build
SCRDESK_ID_SERVER="myserver.com:21116" cargo build --release

# Version override
SCRDESK_VERSION="1.0.0-custom" cargo build --release
```

### Test Komutları

```bash
# Tüm testler
cargo test

# Sadece unit tests
cargo test --lib

# Sadece integration tests
cargo test --test '*'

# Belirli bir test
cargo test test_connection

# Flutter tests
cd flutter && flutter test
```

---

**Son Güncelleme**: 2025-11-17
**Güncelleyen**: SCRDESK Development Team
**Sonraki Gözden Geçirme**: Her hafta Pazartesi
