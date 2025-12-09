#!/bin/bash
# Build macOS .app bundle and package as .zip
# This avoids DMG signing issues

set -e

VERSION="1.0.0"
APP_NAME="ScrDesk"
BUNDLE_ID="com.scrdesk.client"
BUILD_DIR="releases/v${VERSION}"

echo "🍎 Building macOS .app bundle for ${APP_NAME} v${VERSION}"

# Detect architecture
ARCH=$(uname -m)
if [ "${ARCH}" = "arm64" ]; then
    BINARY="scrdesk-macos-arm64"
    TARGET="aarch64-apple-darwin"
    ZIP_NAME="ScrDesk-macOS-AppleSilicon-v${VERSION}.zip"
else
    BINARY="scrdesk-macos-intel"
    TARGET="x86_64-apple-darwin"
    ZIP_NAME="ScrDesk-macOS-Intel-v${VERSION}.zip"
fi

# Create .app bundle structure
APP_DIR="${BUILD_DIR}/${APP_NAME}.app"
CONTENTS_DIR="${APP_DIR}/Contents"
MACOS_DIR="${CONTENTS_DIR}/MacOS"
RESOURCES_DIR="${CONTENTS_DIR}/Resources"

echo "📁 Creating .app bundle structure..."
rm -rf "${APP_DIR}"
mkdir -p "${MACOS_DIR}"
mkdir -p "${RESOURCES_DIR}"

# Copy binary
if [ -f "${BUILD_DIR}/${BINARY}" ]; then
    cp "${BUILD_DIR}/${BINARY}" "${MACOS_DIR}/${APP_NAME}"
    chmod +x "${MACOS_DIR}/${APP_NAME}"
else
    echo "❌ Binary not found: ${BUILD_DIR}/${BINARY}"
    echo "Please run ./build-release.sh first"
    exit 1
fi

# Create Info.plist
cat > "${CONTENTS_DIR}/Info.plist" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key>
    <string>${APP_NAME}</string>
    <key>CFBundleIdentifier</key>
    <string>${BUNDLE_ID}</string>
    <key>CFBundleName</key>
    <string>${APP_NAME}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>${VERSION}</string>
    <key>CFBundleVersion</key>
    <string>${VERSION}</string>
    <key>LSMinimumSystemVersion</key>
    <string>10.15</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSSupportsAutomaticGraphicsSwitching</key>
    <true/>
</dict>
</plist>
EOF

echo "✅ .app bundle created: ${APP_DIR}"

# Create .zip package
ZIP_PATH="${BUILD_DIR}/${ZIP_NAME}"
echo "📦 Creating .zip package..."

cd "${BUILD_DIR}"
rm -f "${ZIP_NAME}"
zip -r -y "${ZIP_NAME}" "${APP_NAME}.app"

echo "✅ .zip created: ${ZIP_PATH}"

# Update checksums
shasum -a 256 "${ZIP_NAME}" >> SHA256SUMS 2>/dev/null || true

echo ""
echo "✨ macOS build complete!"
echo ""
ls -lh "${ZIP_NAME}"
echo ""
echo "📋 To install:"
echo "   1. Download and unzip ${ZIP_NAME}"
echo "   2. Drag ${APP_NAME}.app to Applications folder"
echo "   3. Right-click ${APP_NAME}.app and select 'Open'"
echo "   4. Click 'Open' in the security dialog"
