# SCRDESK - Self-Hosted Remote Desktop

SCRDESK is a fork of RustDesk, customized for self-hosted deployment with your own VPS server.

## About SCRDESK

SCRDESK is a remote desktop solution written in Rust. It works out of the box with your own configured server. You have full control of your data and privacy.

### Platform Support

- ✅ Windows
- ✅ macOS
- ✅ Linux
- ✅ Android
- ✅ iOS (limited functionality)

## Quick Start

### Server Setup (VPS)

#### Requirements
- VPS with minimum 1GB RAM (2GB+ recommended)
- Ubuntu 20.04 or later
- Open ports: 21115-21119 (TCP), 21116 (UDP)

#### Installation

```bash
# Download and install SCRDESK server
wget https://github.com/rustdesk/rustdesk-server/releases/latest/download/rustdesk-server-linux-amd64.zip
unzip rustdesk-server-linux-amd64.zip
cd rustdesk-server-linux-amd64

# Run the server
./hbbs -r <your-server-ip>:21117
./hbbr
```

#### Using Docker (Recommended)

```bash
# Clone this repository
git clone <your-scrdesk-repo>
cd scrdesk

# Run with docker-compose
docker-compose up -d
```

### Client Build Instructions

#### Prerequisites

- Rust toolchain (1.75+)
- C++ build tools
- vcpkg package manager

#### Build from Source

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clone SCRDESK
git clone --recurse-submodules <your-scrdesk-repo>
cd scrdesk

# Install dependencies (Ubuntu/Debian)
sudo apt install -y zip g++ gcc git curl wget nasm yasm libgtk-3-dev clang \
    libxcb-randr0-dev libxdo-dev libxfixes-dev libxcb-shape0-dev \
    libxcb-xfixes0-dev libasound2-dev libpulse-dev cmake make \
    libclang-dev ninja-build libgstreamer1.0-dev \
    libgstreamer-plugins-base1.0-dev libpam0g-dev

# Install vcpkg
git clone https://github.com/microsoft/vcpkg
cd vcpkg
./bootstrap-vcpkg.sh
export VCPKG_ROOT=$PWD
./vcpkg install libvpx libyuv opus aom
cd ..

# Build SCRDESK
cargo build --release
```

#### Windows Build

```bash
# Install vcpkg dependencies
vcpkg install libvpx:x64-windows-static libyuv:x64-windows-static opus:x64-windows-static aom:x64-windows-static

# Build
cargo build --release
```

### Configuration

After building, configure your clients to connect to your VPS server:

1. Run SCRDESK client
2. Go to Settings → Network
3. Set ID Server: `your-vps-ip:21116`
4. Set Relay Server: `your-vps-ip:21117`

## Project Structure

- **[libs/hbb_common](libs/hbb_common)**: Video codec, config, network wrapper, and utilities
- **[libs/scrap](libs/scrap)**: Screen capture functionality
- **[libs/enigo](libs/enigo)**: Keyboard/mouse control
- **[libs/clipboard](libs/clipboard)**: Clipboard management
- **[src/server](src/server)**: Audio/video/input services
- **[src/client.rs](src/client.rs)**: Peer connection handling
- **[flutter](flutter)**: Mobile and desktop UI

## License

SCRDESK is licensed under AGPL-3.0 (inherited from RustDesk).

This means:
- ✅ You can use it freely
- ✅ You can modify the source code
- ✅ You can distribute it
- ⚠️ You must disclose source code of any modifications
- ⚠️ Derivative works must also be AGPL-3.0

## Credits

SCRDESK is based on [RustDesk](https://github.com/rustdesk/rustdesk), an excellent open-source remote desktop solution.

## Disclaimer

**Misuse Disclaimer:** The developers of SCRDESK do not condone or support any unethical or illegal use of this software. Misuse, such as unauthorized access, control or invasion of privacy, is strictly prohibited. The authors are not responsible for any misuse of the application.

---

Built with ❤️ using Rust
