# SCRDESK Proje Özeti

## 📋 Proje Hakkında

**SCRDESK**, RustDesk açık kaynak uzaktan masaüstü yazılımının fork'udur. Kendi VPS sunucunuzda barındırabileceğiniz, tamamen özelleştirilebilir bir uzaktan erişim çözümüdür.

### Temel Özellikler

✅ **Çoklu Platform Desteği**
- Windows (7/8/10/11)
- macOS (10.14+)
- Linux (Ubuntu, Debian, Arch, Fedora, vb.)
- Android (5.0+)
- iOS (13.0+, kısıtlı)

✅ **Kendi Sunucunuz**
- VPS'nizde tam kontrol
- Veri gizliliği garantisi
- AGPL-3.0 lisanslı

✅ **Güçlü Özellikler**
- Ekran paylaşımı
- Dosya transferi
- Uzaktan kontrol
- TCP tünelleme
- Sesli destek

---

## 📁 Proje Yapısı

```
SCRDESK/
├── README.md                 # Ana proje açıklaması
├── QUICKSTART.md            # Hızlı başlangıç kılavuzu
├── SERVER_SETUP.md          # VPS sunucu kurulum rehberi (Türkçe)
├── BUILD_GUIDE.md           # Platform bazlı build rehberi (Türkçe)
├── PROJECT_SUMMARY.md       # Bu dosya
├── Cargo.toml               # Rust proje konfigürasyonu (SCRDESK olarak güncellendi)
├── docker-compose.yml       # Sunucu Docker yapılandırması
├── build.py                 # Build scripti
├── build.rs                 # Rust build scripti
│
├── src/                     # Ana kaynak kod
│   ├── main.rs             # Giriş noktası
│   ├── server/             # Sunucu servisleri
│   ├── client.rs           # Client bağlantı yönetimi
│   ├── platform/           # Platform-specific kod
│   └── ...
│
├── libs/                    # Kütüphaneler
│   ├── hbb_common/         # Ortak fonksiyonlar (codec, network, vb.)
│   ├── scrap/              # Ekran yakalama
│   ├── enigo/              # Klavye/fare kontrolü
│   ├── clipboard/          # Pano yönetimi
│   └── ...
│
├── flutter/                 # Flutter UI (mobil ve masaüstü)
│   ├── lib/                # Dart kaynak kod
│   ├── android/            # Android-specific
│   ├── ios/                # iOS-specific
│   └── ...
│
├── res/                     # Kaynaklar (icon, logo, vb.)
├── docs/                    # Ek dokümantasyon
├── .github/                 # GitHub Actions CI/CD
└── examples/                # Örnek kodlar

```

---

## 🚀 Hızlı Başlangıç

### 1. Sunucu Kurulumu (5 dakika)

```bash
# VPS'ye bağlan
ssh root@your-vps-ip

# Docker kur
curl -fsSL https://get.docker.com | sh

# SCRDESK sunucusu başlat
mkdir scrdesk-server && cd scrdesk-server
wget https://raw.githubusercontent.com/your-repo/scrdesk/main/docker-compose.yml
# docker-compose.yml'de YOUR_SERVER_IP'yi değiştir
docker-compose up -d

# Public key'i al
cat ./data/id_ed25519.pub
```

### 2. Client Kurulumu

Mevcut RustDesk client'larını kullanabilirsiniz:
- [Windows](https://github.com/rustdesk/rustdesk/releases)
- [macOS](https://github.com/rustdesk/rustdesk/releases)
- [Linux](https://github.com/rustdesk/rustdesk/releases)
- [Android APK](https://github.com/rustdesk/rustdesk/releases)

Ayarlar → Network → Sunucu bilgilerinizi girin.

### 3. Kendi Build'inizi Yapın (Opsiyonel)

```bash
# Projeyi klonla
git clone https://github.com/your-repo/scrdesk
cd scrdesk

# Bağımlılıkları kur (BUILD_GUIDE.md'ye bakın)
# ...

# Build et
cargo build --release
```

---

## 🔧 Yapılandırma

### Sunucu Ayarları

`docker-compose.yml`:
```yaml
command: hbbs -r YOUR_IP:21117 -k _  # -k _ : key zorunlu
```

### Client Ayarları

```
ID Server: your-server-ip:21116
Relay Server: your-server-ip:21117
Key: [Public key from server]
```

---

## 📦 Build İşlemleri

### Gereksinimler

- **Rust**: 1.75+
- **vcpkg**: Dependency manager
- **Platform tools**: gcc, cmake, nasm, vb.

### Platform Bazlı Build

**Windows:**
```powershell
vcpkg install libvpx:x64-windows-static libyuv:x64-windows-static
cargo build --release
```

**macOS:**
```bash
brew install cmake nasm
vcpkg install libvpx libyuv opus aom
cargo build --release
```

**Linux:**
```bash
sudo apt install libgtk-3-dev clang cmake
vcpkg install libvpx libyuv opus aom
cargo build --release
```

**Android:**
```bash
cd flutter
flutter build apk --release
```

**iOS:**
```bash
cd flutter
flutter build ios --release
```

Detaylar için: [BUILD_GUIDE.md](BUILD_GUIDE.md)

---

## 🔒 Güvenlik

### Zorunlu Ayarlar

1. **Public Key Encryption**: Sunucu `-k _` parametresi ile başlatın
2. **Firewall**: Sadece gerekli portları açın (21115-21119)
3. **SSH**: Root login'i devre dışı bırakın
4. **Updates**: Düzenli güncelleme yapın

### Opsiyonel ama Önerilen

- **SSL/TLS**: Nginx reverse proxy kullanın
- **Fail2Ban**: Brute-force koruması
- **VPN**: Ek güvenlik katmanı
- **2FA**: Client'larda iki faktörlü kimlik doğrulama

Detaylar: [SERVER_SETUP.md](SERVER_SETUP.md)

---

## 🛠️ Geliştirme

### Kod Yapısı

1. **Backend (Rust)**:
   - `src/`: Ana uygulama mantığı
   - `libs/`: Alt seviye kütüphaneler

2. **Frontend (Flutter)**:
   - `flutter/lib/`: UI kodu
   - Platform-specific bridges

### Değişiklik Yapma

#### Uygulama İsmini Değiştirme

```toml
# Cargo.toml
[package]
name = "scrdesk"  # ✅ Değiştirildi
version = "1.0.0"
```

#### Logo/Branding Değiştirme

```bash
# Logo dosyalarını değiştir
res/32x32.png
res/128x128.png
res/icon.svg
```

#### Sunucu URL'sini Sabitlemek

```rust
// src/config.rs
pub const DEFAULT_SERVER: &str = "your-server-ip:21116";
```

---

## 📊 İzleme ve Loglama

### Sunucu Logları

```bash
# Docker
docker logs -f scrdesk-hbbs
docker logs -f scrdesk-hbbr

# Systemd
journalctl -u scrdesk-hbbs -f
```

### Performans İzleme

```bash
# Kaynak kullanımı
docker stats

# Network trafiği
iftop -i eth0
```

---

## 🐛 Sorun Giderme

### Yaygın Problemler

| Problem | Çözüm |
|---------|-------|
| "ID server offline" | Firewall kontrolü, servis durumu |
| "Relay server error" | hbbr servisini restart edin |
| Build hatası | vcpkg dependencies kurun |
| Bağlantı yavaş | VPS bandwidth kontrolü |

Detaylı troubleshooting: [QUICKSTART.md](QUICKSTART.md)

---

## 📚 Dokümantasyon

- **[README.md](README.md)**: Genel bakış
- **[QUICKSTART.md](QUICKSTART.md)**: Hızlı başlangıç (Türkçe)
- **[SERVER_SETUP.md](SERVER_SETUP.md)**: Detaylı sunucu kurulumu (Türkçe)
- **[BUILD_GUIDE.md](BUILD_GUIDE.md)**: Build rehberi (Türkçe)
- **[RustDesk Docs](https://rustdesk.com/docs/)**: Upstream dokümantasyon

---

## 📄 Lisans

SCRDESK, **AGPL-3.0** lisansı altındadır (RustDesk'ten miras).

### Lisans Gereksinimleri:

✅ **İzin Verilen:**
- Ticari kullanım
- Değiştirme
- Dağıtım
- Patent kullanımı

⚠️ **Koşullar:**
- Kaynak kodunu açıklamalısınız
- Lisans ve telif hakkı bildirimini korumalısınız
- Değişiklikleri belgelemelisiniz
- Aynı lisansı kullanmalısınız (copyleft)

❌ **Sorumluluk:**
- Garanti yoktur
- Sorumluluk sınırlaması

---

## 🤝 Katkıda Bulunma

1. Fork yapın
2. Feature branch oluşturun (`git checkout -b feature/amazing`)
3. Değişikliklerinizi commit edin
4. Branch'inizi push edin
5. Pull Request açın

### Geliştirme Ortamı

```bash
# Dev dependencies
cargo install cargo-watch
cargo install cargo-audit

# Test çalıştırma
cargo test

# Canlı reload
cargo watch -x run
```

---

## 🗺️ Yol Haritası

### v1.0 (Mevcut)
- ✅ RustDesk fork'u
- ✅ SCRDESK rebranding
- ✅ Türkçe dokümantasyon
- ✅ VPS kurulum scriptleri

### v1.1 (Planlanan)
- ⬜ Custom logo ve tema
- ⬜ Otomatik build scriptleri
- ⬜ Web admin paneli
- ⬜ Kullanıcı yönetimi

### v2.0 (Gelecek)
- ⬜ Mobil app store yayını
- ⬜ Gelişmiş şifreleme
- ⬜ Multi-tenant destek
- ⬜ Analytics dashboard

---

## 🙏 Teşekkürler

SCRDESK, [RustDesk](https://github.com/rustdesk/rustdesk) projesinin fork'udur.

RustDesk ekibine ve tüm katkıda bulunanlara teşekkürler! 🎉

---

## 📞 İletişim

- **GitHub Issues**: Hata bildirimi ve özellik istekleri
- **Email**: info@scrdesk.com (örnek)
- **Website**: https://scrdesk.com (örnek)

---

**SCRDESK ile uzaktan erişim artık sizin kontrolünüzde! 🚀**
