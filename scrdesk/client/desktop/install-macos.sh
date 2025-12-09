#!/bin/bash
# ScrDesk macOS Installation Script
# This script automates the installation process for macOS

set -e

echo "🚀 ScrDesk macOS Installer"
echo "=========================="
echo ""

# Detect architecture
ARCH=$(uname -m)
if [[ "$ARCH" == "arm64" ]]; then
    BINARY_NAME="scrdesk-macos-arm64"
    echo "✓ Detected Apple Silicon (M1/M2/M3)"
elif [[ "$ARCH" == "x86_64" ]]; then
    BINARY_NAME="scrdesk-macos-intel"
    echo "✓ Detected Intel processor"
else
    echo "❌ Unsupported architecture: $ARCH"
    exit 1
fi

# Download URL
DOWNLOAD_URL="https://scrdesk.com/downloads/$BINARY_NAME"
INSTALL_DIR="$HOME/.local/bin"
INSTALL_PATH="$INSTALL_DIR/scrdesk"

echo ""
echo "📦 Downloading ScrDesk..."
curl -L -o "/tmp/$BINARY_NAME" "$DOWNLOAD_URL"

echo ""
echo "🔧 Installing..."

# Create installation directory
mkdir -p "$INSTALL_DIR"

# Move binary
mv "/tmp/$BINARY_NAME" "$INSTALL_PATH"

# Make executable
chmod +x "$INSTALL_PATH"

# Remove quarantine attribute
xattr -d com.apple.quarantine "$INSTALL_PATH" 2>/dev/null || true

echo ""
echo "✅ Installation complete!"
echo ""
echo "ScrDesk has been installed to: $INSTALL_PATH"
echo ""
echo "To run ScrDesk, use one of the following methods:"
echo ""
echo "  1. Run directly:"
echo "     $INSTALL_PATH"
echo ""
echo "  2. Add to PATH (recommended):"
echo "     echo 'export PATH=\"\$HOME/.local/bin:\$PATH\"' >> ~/.zshrc"
echo "     source ~/.zshrc"
echo "     scrdesk"
echo ""
echo "  3. Create an alias:"
echo "     echo 'alias scrdesk=\"$INSTALL_PATH\"' >> ~/.zshrc"
echo "     source ~/.zshrc"
echo "     scrdesk"
echo ""
echo "📝 Note: If macOS blocks the app, go to:"
echo "   System Settings → Privacy & Security → Click 'Open Anyway'"
echo ""
echo "🎉 Happy remote desktop sessions!"
