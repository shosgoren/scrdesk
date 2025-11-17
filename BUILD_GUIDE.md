# SCRDESK Build Rehberi

Bu belge, SCRDESK'i farklı platformlar için nasıl derleyeceğinizi açıklar.

## Genel Gereksinimler

- Rust 1.75 veya üzeri
- Git
- vcpkg paket yöneticisi
- Platform-specific build tools

## Platform Bazlı Kurulum

### 🪟 Windows

#### Gereksinimler
```powershell
# Visual Studio 2019/2022 (C++ build tools ile)
# Chocolatey ile bağımlılıkları yükle
choco install rust
choco install git
choco install nasm
choco install cmake
```

#### vcpkg Kurulumu
```powershell
git clone https://github.com/microsoft/vcpkg
cd vcpkg
.\bootstrap-vcpkg.bat
.\vcpkg integrate install

# Bağımlılıkları yükle
.\vcpkg install libvpx:x64-windows-static
.\vcpkg install libyuv:x64-windows-static
.\vcpkg install opus:x64-windows-static
.\vcpkg install aom:x64-windows-static

# Environment variable ayarla
setx VCPKG_ROOT "C:\path\to\vcpkg"
```

#### Build
```powershell
git clone --recurse-submodules https://github.com/your-repo/scrdesk
cd scrdesk
cargo build --release
```

#### Windows Installer Oluşturma
```powershell
# WiX Toolset kur (installer için)
choco install wixtoolset

# Python gerekli
python build.py --portable

# Installer oluştur
python build.py --installer
```

---

### 🍎 macOS

#### Gereksinimler
```bash
# Homebrew kur
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Bağımlılıkları yükle
brew install cmake
brew install nasm
brew install pkg-config
```

#### Rust Kurulumu
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### vcpkg Kurulumu
```bash
git clone https://github.com/microsoft/vcpkg ~/vcpkg
cd ~/vcpkg
./bootstrap-vcpkg.sh
export VCPKG_ROOT=$HOME/vcpkg

# Bağımlılıkları yükle
./vcpkg install libvpx
./vcpkg install libyuv
./vcpkg install opus
./vcpkg install aom
```

#### Build
```bash
git clone --recurse-submodules https://github.com/your-repo/scrdesk
cd scrdesk
cargo build --release
```

#### macOS .app Bundle Oluşturma
```bash
# Flutter kullanarak
cd flutter
flutter build macos --release

# App bundle: build/macos/Build/Products/Release/scrdesk.app
```

---

### 🐧 Linux (Ubuntu/Debian)

#### Sistem Bağımlılıkları
```bash
sudo apt update
sudo apt install -y \
    zip g++ gcc git curl wget nasm yasm \
    libgtk-3-dev clang libxcb-randr0-dev \
    libxdo-dev libxfixes-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libasound2-dev \
    libpulse-dev cmake make libclang-dev \
    ninja-build libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev \
    libpam0g-dev libappindicator3-dev
```

#### Rust Kurulumu
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

#### vcpkg Kurulumu
```bash
git clone https://github.com/microsoft/vcpkg ~/vcpkg
cd ~/vcpkg
git checkout 2023.04.15
./bootstrap-vcpkg.sh
export VCPKG_ROOT=$HOME/vcpkg

# Bağımlılıkları yükle
./vcpkg install libvpx libyuv opus aom
```

#### Build
```bash
git clone --recurse-submodules https://github.com/your-repo/scrdesk
cd scrdesk
cargo build --release
```

#### .deb Paketi Oluşturma
```bash
# cargo-deb kur
cargo install cargo-deb

# .deb paketi oluştur
cargo deb
```

#### AppImage Oluşturma
```bash
# AppImage tool'ları kur
wget https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage
chmod +x linuxdeploy-x86_64.AppImage

# AppImage oluştur
./linuxdeploy-x86_64.AppImage --appdir AppDir --output appimage
```

---

### 📱 Android

#### Gereksinimler
- Android Studio
- Android SDK (API 21+)
- NDK r23+
- Flutter SDK

#### Kurulum
```bash
# Flutter kur
git clone https://github.com/flutter/flutter.git -b stable
export PATH="$PATH:`pwd`/flutter/bin"

# Android SDK path ayarla
export ANDROID_SDK_ROOT=$HOME/Android/Sdk
export ANDROID_NDK_HOME=$ANDROID_SDK_ROOT/ndk/23.1.7779620
```

#### Build
```bash
cd scrdesk/flutter

# Debug APK
flutter build apk --debug

# Release APK
flutter build apk --release --split-per-abi

# AAB (Play Store için)
flutter build appbundle --release
```

#### APK Konumu
```
build/app/outputs/flutter-apk/app-release.apk
build/app/outputs/flutter-apk/app-arm64-v8a-release.apk
build/app/outputs/flutter-apk/app-armeabi-v7a-release.apk
build/app/outputs/flutter-apk/app-x86_64-release.apk
```

---

### 🍏 iOS

#### Gereksinimler
- macOS (zorunlu)
- Xcode 14+
- CocoaPods
- Apple Developer hesabı (distribution için)

#### Kurulum
```bash
# CocoaPods kur
sudo gem install cocoapods

# Flutter kur (macOS bölümündeki gibi)
```

#### Build
```bash
cd scrdesk/flutter

# Development build
flutter build ios --debug --no-codesign

# Release build (signing gerekli)
flutter build ios --release
```

#### Xcode ile Dağıtım
1. `ios/Runner.xcworkspace` dosyasını Xcode'da aç
2. Signing & Capabilities'de Apple Developer hesabını seç
3. Archive oluştur (Product → Archive)
4. App Store Connect'e yükle veya Ad-Hoc dağıtım yap

**Not**: iOS'ta uzaktan masaüstü uygulamaları App Store politikaları nedeniyle kısıtlanmıştır.

---

## Docker ile Build (Tüm Platformlar)

### Linux için Docker Build

```bash
# Build container'ı oluştur
docker build -t scrdesk-builder .

# Build et
docker run --rm -it \
  -v $PWD:/home/user/scrdesk \
  -v scrdesk-git-cache:/home/user/.cargo/git \
  -v scrdesk-registry-cache:/home/user/.cargo/registry \
  -e PUID="$(id -u)" -e PGID="$(id -g)" \
  scrdesk-builder
```

---

## Build Sorunları ve Çözümler

### Hata: "vcpkg not found"
```bash
# VCPKG_ROOT environment variable'ını ayarladığınızdan emin olun
echo $VCPKG_ROOT  # Linux/macOS
echo %VCPKG_ROOT% # Windows
```

### Hata: "linking with cc failed"
```bash
# Ubuntu/Debian:
sudo apt install build-essential

# macOS:
xcode-select --install
```

### Hata: "GTK not found" (Linux)
```bash
sudo apt install libgtk-3-dev
```

### Hata: Flutter dependency sorunu
```bash
cd flutter
flutter clean
flutter pub get
```

---

## Optimizasyon ve Boyut Küçültme

### Release Build ile Boyut Azaltma
```bash
# Profile ile optimize et
cargo build --profile release-with-debug

# Strip ile boyut azalt
strip target/release/scrdesk
```

### Cargo.toml optimizasyonları
```toml
[profile.release]
opt-level = "z"      # Size optimization
lto = true           # Link-time optimization
codegen-units = 1    # Better optimization
panic = 'abort'      # Smaller binary
strip = true         # Strip symbols
```

---

## Test ve Doğrulama

### Unit Tests
```bash
cargo test
```

### Integration Tests
```bash
cargo test --test '*'
```

### Belirli Platform için Test
```bash
# Android emulator test
cd flutter
flutter test
flutter drive --target=test_driver/app.dart
```

---

## Continuous Integration (CI)

### GitHub Actions Örneği

`.github/workflows/build.yml`:

```yaml
name: Build SCRDESK

on: [push, pull_request]

jobs:
  build-linux:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
        with:
          submodules: recursive

      - name: Install dependencies
        run: |
          sudo apt install -y libgtk-3-dev libxcb-randr0-dev

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Build
        run: cargo build --release

      - name: Upload artifact
        uses: actions/upload-artifact@v3
        with:
          name: scrdesk-linux
          path: target/release/scrdesk

  build-windows:
    runs-on: windows-latest
    steps:
      - uses: actions/checkout@v3
        with:
          submodules: recursive

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Build
        run: cargo build --release

      - name: Upload artifact
        uses: actions/upload-artifact@v3
        with:
          name: scrdesk-windows.exe
          path: target/release/scrdesk.exe

  build-macos:
    runs-on: macos-latest
    steps:
      - uses: actions/checkout@v3
        with:
          submodules: recursive

      - name: Setup Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable

      - name: Build
        run: cargo build --release

      - name: Upload artifact
        uses: actions/upload-artifact@v3
        with:
          name: scrdesk-macos
          path: target/release/scrdesk
```

---

## Versiyonlama

Yeni versiyon çıkarmak için:

1. `Cargo.toml` içindeki versiyonu güncelle
2. Git tag oluştur:
```bash
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

---

## Dağıtım

### GitHub Releases
```bash
# GitHub CLI ile
gh release create v1.0.0 \
  target/release/scrdesk-linux \
  target/release/scrdesk.exe \
  --title "SCRDESK v1.0.0" \
  --notes "Release notes..."
```

---

Daha fazla bilgi için:
- [RustDesk Build Documentation](https://rustdesk.com/docs/en/dev/build/)
- [Flutter Build Documentation](https://flutter.dev/docs/deployment)
