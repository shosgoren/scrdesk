# Changelog

SCRDESK projesindeki tüm önemli değişiklikler bu dosyada belgelenecektir.

Format [Keep a Changelog](https://keepachangelog.com/en/1.0.0/) standardına uygundur,
ve bu proje [Semantic Versioning](https://semver.org/spec/v2.0.0.html) kullanır.

## [Unreleased]

### Planlanıyor
- Logo ve branding değişiklikleri
- Varsayılan sunucu yapılandırması
- GitHub Actions CI/CD pipeline
- Otomatik build ve release sistemi

---

## [1.0.0-alpha] - 2025-11-17

### Added
- 🎉 **İlk SCRDESK release** - RustDesk'ten fork edildi
- 📚 **Kapsamlı Türkçe Dokümantasyon**:
  - `README.md` - Proje ana sayfası (İngilizce)
  - `QUICKSTART.md` - 5 dakikada kurulum rehberi (Türkçe)
  - `SERVER_SETUP.md` - Detaylı VPS sunucu kurulum kılavuzu (Türkçe)
  - `BUILD_GUIDE.md` - Tüm platformlar için derleme rehberi (Türkçe)
  - `PROJECT_SUMMARY.md` - Proje yapısı ve özeti (Türkçe)
  - `DEVELOPMENT_ROADMAP.md` - Geliştirme yol haritası ve ilerleme takibi
  - `CHANGELOG.md` - Bu dosya
- 🐳 **Docker Desteği**:
  - `docker-compose.yml` - Hazır sunucu kurulumu için
  - Data persistence yapılandırması
- 📦 **GitHub Repository Kurulumu**:
  - Repository: `https://github.com/shosgoren/scrdesk`
  - Git yapılandırması
  - `.gitignore` güncellemeleri

### Changed
- 🏷️ **Proje Rebranding**:
  - Package name: `rustdesk` → `scrdesk`
  - Library name: `librustdesk` → `libscrdesk`
  - Binary name: `rustdesk` → `scrdesk`
  - Default run target: `scrdesk`
- 📝 **Cargo.toml Güncellemeleri**:
  - Version: `1.4.4` → `1.0.0`
  - Authors: `SCRDESK Team <info@scrdesk.com>`
  - Description: `SCRDESK Remote Desktop`
- 🪟 **Windows Metadata**:
  - ProductName: `SCRDESK`
  - FileDescription: `SCRDESK Remote Desktop`
  - OriginalFilename: `scrdesk.exe`
  - LegalCopyright: `Copyright © 2025 SCRDESK. All rights reserved.`
- 🍎 **macOS Bundle**:
  - Bundle name: `SCRDESK`
  - Bundle identifier: `com.carriez.rustdesk` → `com.scrdesk.scrdesk`
- 📂 **Git Konfigürasyonu**:
  - Remote origin: RustDesk → SCRDESK repository
  - `.gitignore`: Server data ve log dosyaları eklendi

### Removed
- ⛓️ Upstream RustDesk git remote bağlantısı kaldırıldı

---

## RustDesk Geçmişi

SCRDESK, [RustDesk v1.4.4](https://github.com/rustdesk/rustdesk) versiyonundan fork edilmiştir.

### RustDesk v1.4.4'ten Miras Kalan Özellikler

#### Platform Desteği
- ✅ Windows 7/8/10/11
- ✅ macOS 10.14+
- ✅ Linux (Ubuntu, Debian, Arch, Fedora, CentOS, openSUSE)
- ✅ Android 5.0+
- ✅ iOS 13.0+ (kısıtlı özellikler)

#### Core Özellikler
- ✅ Uzaktan masaüstü kontrolü
- ✅ Dosya transferi
- ✅ TCP tünelleme
- ✅ Ses aktarımı
- ✅ Pano senkronizasyonu
- ✅ Hardware codec desteği (H264, H265)
- ✅ VP8/VP9 video codec
- ✅ Opus audio codec
- ✅ Self-hosted sunucu desteği
- ✅ End-to-end şifreleme
- ✅ 2FA (Two-Factor Authentication)
- ✅ Çoklu monitör desteği
- ✅ Dosya sürükle-bırak
- ✅ Session kayıt
- ✅ Duvar kağıdı değiştirme
- ✅ Privacy mode

#### Teknik Altyapı
- ✅ Rust programlama dili
- ✅ Flutter UI framework
- ✅ Sciter UI (deprecated, eski sürümler için)
- ✅ QUIC protocol desteği
- ✅ KCP protocol
- ✅ WebRTC STUN/TURN
- ✅ NAT traversal (hole punching)

---

## Versiyon Açıklamaları

### Semantic Versioning Formatı
```
MAJOR.MINOR.PATCH[-PRERELEASE][+BUILD]

Örnek: 1.2.3-alpha.1+20250117
```

- **MAJOR**: Breaking changes (geriye dönük uyumlu olmayan değişiklikler)
- **MINOR**: Yeni özellikler (geriye dönük uyumlu)
- **PATCH**: Bug fixes (geriye dönük uyumlu)
- **PRERELEASE**: alpha, beta, rc (release candidate)
- **BUILD**: Build metadata

### Release Types

- **alpha**: İlk geliştirme aşaması, eksik özellikler olabilir
- **beta**: Özellik tamamlandı, test aşamasında
- **rc**: Release candidate, son testler
- **(stable)**: Kararlı sürüm, üretim için hazır

---

## Değişiklik Kategorileri

### Added
Yeni özellikler, dosyalar, fonksiyonlar

### Changed
Mevcut fonksiyonalitede değişiklikler

### Deprecated
Yakında kaldırılacak özellikler (uyarı)

### Removed
Kaldırılan özellikler

### Fixed
Bug düzeltmeleri

### Security
Güvenlik açıkları ve düzeltmeleri

---

## Git Commit Mesaj Formatı

```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types
- `feat`: Yeni özellik
- `fix`: Bug düzeltmesi
- `docs`: Dokümantasyon değişikliği
- `style`: Kod formatı (semantik değişiklik yok)
- `refactor`: Kod refactoring
- `perf`: Performans iyileştirmesi
- `test`: Test ekleme/düzeltme
- `chore`: Build, dependency güncellemeleri
- `ci`: CI/CD değişiklikleri

### Scope
- `ui`: Kullanıcı arayüzü
- `server`: Sunucu tarafı
- `client`: Client tarafı
- `build`: Build sistemi
- `docs`: Dokümantasyon
- `core`: Core fonksiyonalite

### Örnekler
```bash
feat(ui): add SCRDESK custom logo
fix(server): resolve connection timeout issue
docs(readme): update installation instructions
chore(deps): update Rust to 1.75
ci(github): add automated release workflow
```

---

## Release Checklist

Yeni version çıkarmadan önce:

- [ ] Tüm testler başarılı (`cargo test`)
- [ ] Linting temiz (`cargo clippy`)
- [ ] Format düzgün (`cargo fmt`)
- [ ] CHANGELOG.md güncellendi
- [ ] DEVELOPMENT_ROADMAP.md güncellendi
- [ ] VERSION.txt güncellendi (eğer varsa)
- [ ] Cargo.toml version numarası güncellendi
- [ ] Git tag oluşturuldu
  ```bash
  git tag -a v1.0.0 -m "Release v1.0.0"
  git push origin v1.0.0
  ```
- [ ] GitHub Release oluşturuldu
- [ ] Binary dosyalar yüklendi
- [ ] Release notes yazıldı

---

## Bağlantılar

- **GitHub Releases**: https://github.com/shosgoren/scrdesk/releases
- **GitHub Issues**: https://github.com/shosgoren/scrdesk/issues
- **Development Roadmap**: [DEVELOPMENT_ROADMAP.md](DEVELOPMENT_ROADMAP.md)
- **Contributing Guide**: (Yakında)

---

**Changelog Formatı**: [Keep a Changelog](https://keepachangelog.com/)
**Versioning**: [Semantic Versioning](https://semver.org/)
