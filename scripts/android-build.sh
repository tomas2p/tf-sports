#!/usr/bin/env bash
# ─────────────────────────────────────────────────────────────────────────────
# android-build.sh
# Firma y compila la release de Android leyendo credenciales de keystore.properties
# ─────────────────────────────────────────────────────────────────────────────
# Uso:
#   cp keystore.properties.example keystore.properties   # hazlo una vez
#   # edita keystore.properties con tus valores
#   bash scripts/android-build.sh
#
# En CI, keystore.properties se genera automáticamente desde los secrets.
# ─────────────────────────────────────────────────────────────────────────────
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"
PROPS_FILE="$ROOT/keystore.properties"

if [ ! -f "$PROPS_FILE" ]; then
  echo "ERROR: $PROPS_FILE no encontrado."
  echo "Copia keystore.properties.example a keystore.properties y rellena los valores."
  exit 1
fi

# Leer valores del archivo properties
get_prop() {
  grep -E "^$1\s*=" "$PROPS_FILE" | sed "s/^$1\s*=\s*//" | tr -d '\r'
}

JKS_FILE="$(get_prop storeFile)"
KEY_ALIAS="$(get_prop keyAlias)"
KEY_PASSWORD="$(get_prop password)"

if [ -z "$JKS_FILE" ] || [ -z "$KEY_ALIAS" ] || [ -z "$KEY_PASSWORD" ]; then
  echo "ERROR: keystore.properties debe contener storeFile, keyAlias y password."
  exit 1
fi

if [ ! -f "$JKS_FILE" ]; then
  echo "ERROR: Keystore no encontrado en '$JKS_FILE'"
  exit 1
fi

# Ruta relativa al workspace (Dioxus la resuelve relativa al proyecto)
JKS_RELATIVE="$(realpath --relative-to="$ROOT" "$JKS_FILE" 2>/dev/null || echo "$JKS_FILE")"

echo ">>> Parcheando Dioxus.toml con credenciales de keystore.properties..."
cp "$ROOT/Dioxus.toml" "$ROOT/Dioxus.toml.bak"

sed -i "s|jks_file = \".*\"|jks_file = \"$JKS_RELATIVE\"|"       "$ROOT/Dioxus.toml"
sed -i "s|jks_alias = \".*\"|jks_alias = \"$KEY_ALIAS\"|"         "$ROOT/Dioxus.toml"
sed -i "s|jks_password = \".*\"|jks_password = \"$KEY_PASSWORD\"|" "$ROOT/Dioxus.toml"
sed -i "s|key_alias = \".*\"|key_alias = \"$KEY_ALIAS\"|"         "$ROOT/Dioxus.toml"
sed -i "s|key_password = \".*\"|key_password = \"$KEY_PASSWORD\"|" "$ROOT/Dioxus.toml"

echo ">>> Dioxus.toml parcheado (restaurando al terminar):"
grep -A8 '\[bundle.android\]' "$ROOT/Dioxus.toml" | sed 's/password = .*/password = ***/'

restore() {
  echo ">>> Restaurando Dioxus.toml original..."
  mv "$ROOT/Dioxus.toml.bak" "$ROOT/Dioxus.toml"
}
trap restore EXIT

echo ""
echo ">>> dx build --platform android --release"
export PATH="$HOME/.cargo/bin:$PATH"
cd "$ROOT"
dx build --platform android --release

echo ""
echo ">>> APK/AAB generados:"
find . -type f \( -iname "*.apk" -o -iname "*.aab" \) -print
