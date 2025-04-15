#!/bin/bash

set -e

APP_NAME="RHA"
APP_BUNDLE="${APP_NAME}.app"
DMG_NAME="${APP_NAME}.dmg"
VOLUME_NAME="Instalar ${APP_NAME}"
DMG_TEMP_DIR="./dmg-temp"

# 1. Asegúrate de que existe el .app
if [ ! -d "$APP_BUNDLE" ]; then
    echo "❌ No se encontró $APP_BUNDLE. Ejecuta primero el build-mac-app.sh"
    exit 1
fi

# 2. Limpieza y preparación
rm -rf "$DMG_TEMP_DIR"
mkdir -p "$DMG_TEMP_DIR"
cp -R "$APP_BUNDLE" "$DMG_TEMP_DIR"

# 3. Crear el .dmg
hdiutil create -volname "$VOLUME_NAME" -srcfolder "$DMG_TEMP_DIR" -ov -format UDZO "$DMG_NAME"

# 4. Limpiar temporal
rm -rf "$DMG_TEMP_DIR"

echo "✅ .dmg creado: $DMG_NAME"
