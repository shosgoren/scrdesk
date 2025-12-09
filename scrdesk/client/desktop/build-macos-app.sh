#!/bin/bash
# Build macOS .app bundle and .dmg for ScrDesk

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
else
    BINARY="scrdesk-macos-intel"
    TARGET="x86_64-apple-darwin"
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

# Create .dmg if hdiutil is available
if command -v hdiutil &> /dev/null; then
    DMG_NAME="${APP_NAME}-${ARCH}-v${VERSION}.dmg"
    DMG_PATH="${BUILD_DIR}/${DMG_NAME}"
    TEMP_DMG="${BUILD_DIR}/temp.dmg"

    echo "📦 Creating .dmg installer..."

    # Remove old DMG if exists
    rm -f "${DMG_PATH}" "${TEMP_DMG}"

    # Create temporary DMG
    hdiutil create -size 50m -fs HFS+ -volname "${APP_NAME}" "${TEMP_DMG}" -ov

    # Mount it
    MOUNT_DIR=$(hdiutil attach -readwrite -noverify "${TEMP_DMG}" | grep Volumes | awk '{print $3}')

    # Copy app to DMG
    cp -R "${APP_DIR}" "${MOUNT_DIR}/"

    # Create a symlink to Applications
    ln -s /Applications "${MOUNT_DIR}/Applications"

    # Unmount
    hdiutil detach "${MOUNT_DIR}"

    # Convert to compressed DMG
    hdiutil convert "${TEMP_DMG}" -format UDZO -o "${DMG_PATH}"
    rm -f "${TEMP_DMG}"

    echo "✅ .dmg created: ${DMG_PATH}"

    # Update checksums
    cd "${BUILD_DIR}"
    shasum -a 256 *.dmg >> SHA256SUMS 2>/dev/null || true
else
    echo "⚠️  hdiutil not found, skipping .dmg creation"
fi

echo ""
echo "✨ macOS build complete!"
echo ""
ls -lh "${BUILD_DIR}"
