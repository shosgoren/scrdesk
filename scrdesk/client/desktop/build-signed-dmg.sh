#!/bin/bash
# Build properly signed macOS app for M4 (Apple Silicon)

set -e

VERSION="1.0.0"
APP_NAME="ScrDesk"
BUNDLE_ID="com.scrdesk.client"
BUILD_DIR="releases/v${VERSION}"

echo "🍎 Building signed macOS app for Apple Silicon (M4)"

# Detect architecture
ARCH=$(uname -m)
if [ "${ARCH}" != "arm64" ]; then
    echo "⚠️  Warning: Not running on Apple Silicon, but building for ARM64 anyway"
fi

BINARY="scrdesk-macos-arm64"

# Create .app bundle
APP_DIR="${BUILD_DIR}/${APP_NAME}.app"
CONTENTS_DIR="${APP_DIR}/Contents"
MACOS_DIR="${CONTENTS_DIR}/MacOS"
RESOURCES_DIR="${CONTENTS_DIR}/Resources"

echo "📁 Creating .app bundle..."
rm -rf "${APP_DIR}"
mkdir -p "${MACOS_DIR}"
mkdir -p "${RESOURCES_DIR}"

# Copy binary
if [ ! -f "${BUILD_DIR}/${BINARY}" ]; then
    echo "❌ Binary not found: ${BUILD_DIR}/${BINARY}"
    exit 1
fi

cp "${BUILD_DIR}/${BINARY}" "${MACOS_DIR}/${APP_NAME}"
chmod +x "${MACOS_DIR}/${APP_NAME}"

# Create Info.plist with proper settings
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
    <key>CFBundleDisplayName</key>
    <string>${APP_NAME}</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleShortVersionString</key>
    <string>${VERSION}</string>
    <key>CFBundleVersion</key>
    <string>${VERSION}</string>
    <key>LSMinimumSystemVersion</key>
    <string>11.0</string>
    <key>NSHighResolutionCapable</key>
    <true/>
    <key>NSSupportsAutomaticGraphicsSwitching</key>
    <true/>
    <key>LSApplicationCategoryType</key>
    <string>public.app-category.utilities</string>
</dict>
</plist>
EOF

echo "✅ App bundle created"

# Ad-hoc code signing (works without developer certificate)
echo "🔐 Applying ad-hoc code signature..."
codesign --force --deep --sign - "${APP_DIR}"

echo "✓ App signed with ad-hoc signature"

# Remove quarantine from the app we just built
xattr -cr "${APP_DIR}"

# Create DMG
DMG_NAME="ScrDesk-M4-v${VERSION}.dmg"
DMG_PATH="${BUILD_DIR}/${DMG_NAME}"
TEMP_DMG="${BUILD_DIR}/temp.dmg"

echo "📦 Creating DMG..."
rm -f "${DMG_PATH}" "${TEMP_DMG}"

# Create a temporary folder for DMG contents
DMG_STAGING="${BUILD_DIR}/dmg_staging"
rm -rf "${DMG_STAGING}"
mkdir -p "${DMG_STAGING}"

# Copy app to staging
cp -R "${APP_DIR}" "${DMG_STAGING}/"

# Create symlink to Applications
ln -s /Applications "${DMG_STAGING}/Applications"

# Create DMG
hdiutil create -volname "${APP_NAME}" -srcfolder "${DMG_STAGING}" -ov -format UDZO "${DMG_PATH}"

# Clean up staging
rm -rf "${DMG_STAGING}"

# Sign the DMG
echo "🔐 Signing DMG..."
codesign --force --sign - "${DMG_PATH}"

echo "✅ DMG created and signed: ${DMG_PATH}"

# Verify
echo "🔍 Verifying signatures..."
codesign -dvv "${APP_DIR}" 2>&1 | head -5
codesign -dvv "${DMG_PATH}" 2>&1 | head -5

# Update checksums
cd "${BUILD_DIR}"
shasum -a 256 "${DMG_NAME}" > "${DMG_NAME}.sha256"

echo ""
echo "✨ Build complete!"
echo ""
ls -lh "${DMG_NAME}"
echo ""
echo "SHA256:"
cat "${DMG_NAME}.sha256"
