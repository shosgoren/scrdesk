#!/bin/bash
# Cross-platform release build script for ScrDesk Desktop Client

set -e

VERSION="1.0.0"
BUILD_DIR="releases/v${VERSION}"

echo "🏗️  Building ScrDesk Desktop Client v${VERSION}"

# Create releases directory
mkdir -p "${BUILD_DIR}"

# Detect current platform
PLATFORM=$(uname -s)
ARCH=$(uname -m)

echo "📦 Building for ${PLATFORM} ${ARCH}..."

case "${PLATFORM}" in
    Darwin)
        echo "🍎 Building macOS version..."
        if [ "${ARCH}" = "arm64" ]; then
            # Apple Silicon (M1/M2/M3)
            cargo build --release --target aarch64-apple-darwin
            cp target/aarch64-apple-darwin/release/scrdesk "${BUILD_DIR}/scrdesk-macos-arm64"

            # Also build for Intel if possible
            if command -v rustup &> /dev/null; then
                rustup target add x86_64-apple-darwin 2>/dev/null || true
                cargo build --release --target x86_64-apple-darwin || echo "⚠️  Intel build skipped"
                [ -f target/x86_64-apple-darwin/release/scrdesk ] && \
                    cp target/x86_64-apple-darwin/release/scrdesk "${BUILD_DIR}/scrdesk-macos-intel"
            fi
        else
            # Intel Mac
            cargo build --release --target x86_64-apple-darwin
            cp target/x86_64-apple-darwin/release/scrdesk "${BUILD_DIR}/scrdesk-macos-intel"
        fi
        echo "✅ macOS build complete"
        ;;

    Linux)
        echo "🐧 Building Linux version..."
        cargo build --release --target x86_64-unknown-linux-gnu
        cp target/x86_64-unknown-linux-gnu/release/scrdesk "${BUILD_DIR}/scrdesk-linux-x86_64"
        strip "${BUILD_DIR}/scrdesk-linux-x86_64"
        echo "✅ Linux build complete"
        ;;

    MINGW*|MSYS*|CYGWIN*)
        echo "🪟 Building Windows version..."
        cargo build --release --target x86_64-pc-windows-msvc
        cp target/x86_64-pc-windows-msvc/release/scrdesk.exe "${BUILD_DIR}/scrdesk-windows-x86_64.exe"
        echo "✅ Windows build complete"
        ;;

    *)
        echo "❌ Unsupported platform: ${PLATFORM}"
        exit 1
        ;;
esac

# Create checksums
cd "${BUILD_DIR}"
echo "🔐 Creating checksums..."
sha256sum scrdesk-* > SHA256SUMS 2>/dev/null || shasum -a 256 scrdesk-* > SHA256SUMS

echo ""
echo "✨ Build complete! Binaries are in ${BUILD_DIR}/"
ls -lh
echo ""
echo "📋 SHA256 checksums:"
cat SHA256SUMS
