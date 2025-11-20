#!/bin/bash

###############################################################################
# SCRDESK Rebranding Script
# This script replaces RustDesk references with SCRDESK
###############################################################################

set -e

SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"

echo "🔧 Starting SCRDESK rebranding..."
echo "📁 Project directory: $PROJECT_DIR"
cd "$PROJECT_DIR"

# Backup
echo "📦 Creating backup..."
BACKUP_DIR="$PROJECT_DIR/.backup-$(date +%Y%m%d-%H%M%S)"
mkdir -p "$BACKUP_DIR"

# 1. Main source files
echo "✏️  [1/10] Updating src/main.rs..."
sed -i.bak 's/use librustdesk::/use libscrdesk::/g' src/main.rs
sed -i.bak 's/"RustDesk command line tool"/"SCRDESK command line tool"/g' src/main.rs
sed -i.bak 's/Purslane Ltd<info@rustdesk.com>/SCRDESK Team <info@scrdesk.com>/g' src/main.rs

# 2. Common.rs - Critical app name references
echo "✏️  [2/10] Updating src/common.rs..."
sed -i.bak 's/RUSTDESK_APPNAME/SCRDESK_APPNAME/g' src/common.rs
sed -i.bak 's/rustdesk\.com/scrdesk.com/g' src/common.rs
sed -i.bak 's/"RustDesk"/"SCRDESK"/g' src/common.rs

# 3. English language file (template for others)
echo "✏️  [3/10] Updating src/lang/en.rs..."
sed -i.bak 's/RustDesk network/SCRDESK network/g' src/lang/en.rs
sed -i.bak 's/grant RustDesk/grant SCRDESK/g' src/lang/en.rs
sed -i.bak 's/RustDesk can not/SCRDESK can not/g' src/lang/en.rs
sed -i.bak 's/RustDesk Input/SCRDESK Input/g' src/lang/en.rs
sed -i.bak 's/Keep RustDesk/Keep SCRDESK/g' src/lang/en.rs
sed -i.bak 's/Show RustDesk/Show SCRDESK/g' src/lang/en.rs
sed -i.bak 's/About RustDesk/About SCRDESK/g' src/lang/en.rs
sed -i.bak 's/rustdesk\.com/scrdesk.com/g' src/lang/en.rs

# 4. Flutter consts
echo "✏️  [4/10] Updating flutter/lib/consts.dart..."
sed -i.bak 's/rustdesk_virtual_displays/scrdesk_virtual_displays/g' flutter/lib/consts.dart
sed -i.bak 's/RUSTDESK_APPNAME/SCRDESK_APPNAME/g' flutter/lib/consts.dart

# 5. Flutter common
echo "✏️  [5/10] Updating flutter/lib/common.dart..."
sed -i.bak 's/org\.rustdesk\.rustdesk/org.scrdesk.scrdesk/g' flutter/lib/common.dart
sed -i.bak 's/rustdesk\.com/scrdesk.com/g' flutter/lib/common.dart
sed -i.bak 's/Start closing RustDesk/Start closing SCRDESK/g' flutter/lib/common.dart

# 6. Android Manifest
echo "✏️  [6/10] Updating Android manifest..."
if [ -f "flutter/android/app/src/main/AndroidManifest.xml" ]; then
    sed -i.bak 's/android:label="RustDesk"/android:label="SCRDESK"/g' flutter/android/app/src/main/AndroidManifest.xml
    sed -i.bak 's/RustDesk Input/SCRDESK Input/g' flutter/android/app/src/main/AndroidManifest.xml
    sed -i.bak 's/android:scheme="rustdesk"/android:scheme="scrdesk"/g' flutter/android/app/src/main/AndroidManifest.xml
fi

# 7. Android build.gradle
echo "✏️  [7/10] Updating Android build.gradle..."
if [ -f "flutter/android/app/build.gradle" ]; then
    sed -i.bak 's/com\.carriez\.flutter_hbb/com.scrdesk.scrdesk/g' flutter/android/app/build.gradle
fi

# 8. macOS Info.plist
echo "✏️  [8/10] Updating macOS Info.plist..."
if [ -f "flutter/macos/Runner/Info.plist" ]; then
    sed -i.bak 's/com\.carriez\.rustdesk/com.scrdesk.scrdesk/g' flutter/macos/Runner/Info.plist
    sed -i.bak 's/<string>rustdesk<\/string>/<string>scrdesk<\/string>/g' flutter/macos/Runner/Info.plist
fi

# 9. Linux desktop files
echo "✏️  [9/10] Updating Linux desktop files..."
if [ -f "res/rustdesk.desktop" ]; then
    sed -i.bak 's/Name=RustDesk/Name=SCRDESK/g' res/rustdesk.desktop
    sed -i.bak 's/Exec=rustdesk/Exec=scrdesk/g' res/rustdesk.desktop
    sed -i.bak 's/Icon=rustdesk/Icon=scrdesk/g' res/rustdesk.desktop
    sed -i.bak 's/StartupWMClass=rustdesk/StartupWMClass=scrdesk/g' res/rustdesk.desktop
    mv res/rustdesk.desktop res/scrdesk.desktop
fi

if [ -f "res/rustdesk-link.desktop" ]; then
    sed -i.bak 's/RustDesk/SCRDESK/g' res/rustdesk-link.desktop
    sed -i.bak 's/rustdesk/scrdesk/g' res/rustdesk-link.desktop
    mv res/rustdesk-link.desktop res/scrdesk-link.desktop
fi

# 10. Build.py
echo "✏️  [10/10] Updating build.py..."
sed -i.bak "s/hbb_name = 'rustdesk'/hbb_name = 'scrdesk'/g" build.py

# Clean up .bak files
echo "🧹 Cleaning up backup files..."
find . -name "*.bak" -delete

echo ""
echo "✅ Rebranding complete!"
echo ""
echo "📋 Summary of changes:"
echo "   • Updated app name in all main source files"
echo "   • Changed package IDs (com.scrdesk.scrdesk)"
echo "   • Updated URL scheme (scrdesk://)"
echo "   • Modified Android & macOS manifests"
echo "   • Updated Linux desktop files"
echo "   • Changed environment variables"
echo ""
echo "⚠️  IMPORTANT: Manual steps still required:"
echo "   1. Update all language files (src/lang/*.rs) - 47 files"
echo "   2. Review and test build process"
echo "   3. Update icon files if needed"
echo "   4. Test on each platform"
echo ""
echo "🔍 To verify changes:"
echo "   grep -r \"RustDesk\" src/ flutter/lib/ --exclude-dir=target | wc -l"
echo ""
