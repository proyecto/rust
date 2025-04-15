#!/bin/bash

set -e

APP_NAME="RHA"
BIN_NAME="RHA"
DB_PATH="data/test.db"
APP_DIR="$APP_NAME.app"
BUILD_DIR="target/release"
APP_EXECUTABLE="$BUILD_DIR/$BIN_NAME"
APP_MACOS="$APP_DIR/Contents/MacOS"
APP_RESOURCES="$APP_DIR/Contents/Resources"
PLIST="$APP_DIR/Contents/Info.plist"

echo "🔨 Compilando en modo release..."
cargo build --release

echo "📦 Preparando estructura .app..."
rm -rf "$APP_DIR"
mkdir -p "$APP_MACOS"
mkdir -p "$APP_RESOURCES"

echo "📋 Copiando ejecutable..."
cp "$APP_EXECUTABLE" "$APP_MACOS/"
chmod +x "$APP_MACOS/$BIN_NAME"

echo "📋 Copiando base de datos..."
cp "$DB_PATH" "$APP_RESOURCES/"

echo "📝 Creando Info.plist..."
cat > "$PLIST" <<EOF
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
 "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
  <key>CFBundleExecutable</key>
  <string>$BIN_NAME</string>
  <key>CFBundleIdentifier</key>
  <string>com.rust.rha</string>
  <key>CFBundleName</key>
  <string>$APP_NAME</string>
  <key>CFBundleVersion</key>
  <string>0.1</string>
  <key>CFBundlePackageType</key>
  <string>APPL</string>
  <key>NSPrincipalClass</key>
  <string>NSApplication</string>
</dict>
</plist>
EOF

echo "🔐 Firmando la aplicación..."
codesign --force --deep --sign - "$APP_DIR"

echo "🧼 Quitando cuarentena (si aplica)..."
xattr -rd com.apple.quarantine "$APP_DIR"

echo "🚀 Ejecutando aplicación..."
open "$APP_DIR"
