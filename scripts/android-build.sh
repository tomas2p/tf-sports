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

# --- Extraer nombre y versión desde Cargo.toml (pueden ser sobrescritos por env vars)
APP_NAME_RAW="$(sed -n 's/^name\s*=\s*"\(.*\)"/\1/p' "$ROOT/Cargo.toml" | head -n1 || true)"
if [ -n "${APP_VERSION:-}" ]; then
  APP_VERSION="$APP_VERSION"
else
  APP_VERSION="$(sed -n 's/^version\s*=\s*"\(.*\)"/\1/p' "$ROOT/Cargo.toml" | head -n1 || true)"
fi
# Normalizar: quitar prefijo 'v' si existe
APP_VERSION="$(echo "$APP_VERSION" | sed 's/^v//i')"
export APP_VERSION APP_NAME_RAW

# Ejecutar sincronización del README si el script existe
SYNC_SCRIPT="$SCRIPT_DIR/sync-version-to-readme.sh"
if [ -x "$SYNC_SCRIPT" ]; then
  echo ">>> Ejecutando $SYNC_SCRIPT"
  "$SYNC_SCRIPT" || true
elif [ -f "$SYNC_SCRIPT" ]; then
  echo ">>> Ejecutando $SYNC_SCRIPT"
  bash "$SYNC_SCRIPT" || true
fi

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

echo ">>> Configurando Gradle para deshabilitar lintVital..."
mkdir -p "${GRADLE_USER_HOME:-$HOME/.gradle}/init.d"
cat > "${GRADLE_USER_HOME:-$HOME/.gradle}/init.d/disable-lint.gradle" << 'GRADLE_EOF'
gradle.taskGraph.whenReady { graph ->
    graph.allTasks.each { task ->
        if (task.name.toLowerCase().contains('lintvital')) {
            task.enabled = false
        }
    }
}
GRADLE_EOF

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
export PATH="$HOME/.cargo/bin:$PATH"
cd "$ROOT"

# Targets a compilar: triple Rust -> ABI Android
# aarch64  = dispositivos ARM64 modernos (la gran mayoría)
# armv7    = dispositivos ARM32 más antiguos
# x86_64   = emuladores x86_64 (AVD, etc.)
#
# Para pruebas locales puedes exportar `ANDROID_TARGET` con un único
# entry en formato "<triple>:<abi>" (por ejemplo
# "aarch64-linux-android:arm64-v8a") o `ANDROID_TARGETS` como lista
# separada por comas. Si no se especifica, se usa por defecto
# "aarch64-linux-android:arm64-v8a".
if [ -n "${ANDROID_TARGET:-}" ]; then
  TARGETS=("$ANDROID_TARGET")
elif [ -n "${ANDROID_TARGETS:-}" ]; then
  IFS=',' read -r -a TARGETS <<< "$ANDROID_TARGETS"
else
  # Default behavior: use a single-arch build for local testing (aarch64).
  # To build all three ABIs for a release, set ANDROID_BUILD_MODE=all
  if [ "${ANDROID_BUILD_MODE:-single}" = "all" ]; then
    TARGETS=(
      "aarch64-linux-android:arm64-v8a"
      "armv7-linux-androideabi:armeabi-v7a"
      "x86_64-linux-android:x86_64"
    )
  else
    # Single-arch default (chosen by you for faster local tests)
    TARGETS=(
      "aarch64-linux-android:arm64-v8a"
    )
  fi
  echo ">>> ANDROID_BUILD_MODE=${ANDROID_BUILD_MODE:-single} — targets: ${TARGETS[*]}"
fi

# Exportar SINGLE_ARCH si solo hay una arquitectura objetivo para evitar splits innecesarios
if [ "${#TARGETS[@]}" -eq 1 ]; then
  export SINGLE_ARCH=true
  echo ">>> Una sola arquitectura (${TARGETS[0]}) — SINGLE_ARCH=true (sin APK splits)"
else
  export SINGLE_ARCH=false
fi

echo ">>> Limpiando PNGs ic_launcher antiguos en target/dx (si existen)"
find "$ROOT/target/dx" -type f -path '*/release/android/app/app/src/main/res/**/ic_launcher.png' -delete 2>/dev/null || true

# 1. Compilar para cada ABI con dx.
#    Cada ejecución regenera el proyecto Gradle en target/dx/; la última lo deja listo.
#    Los .so se acumulan en target/<triple>/ para ser inyectados después.
for entry in "${TARGETS[@]}"; do
  triple="${entry%%:*}"
  abi="${entry##*:}"
  echo ""
  echo ">>> dx build --platform android --release --target $triple  ($abi)"
  dx build --platform android --release --target "$triple"
done

# 2. Localizar el proyecto Gradle generado por la última compilación
APP_DIR="$(find "$ROOT/target/dx" -type d -path '*/release/android/app' -print -quit 2>/dev/null || true)"
if [ -z "$APP_DIR" ]; then
  echo "ERROR: No se encontró el directorio de la app Gradle generada por dx."
  exit 1
fi
JNILIBS_DIR="$APP_DIR/app/src/main/jniLibs"

# 3. Inyectar los .so de todos los ABIs en el proyecto Gradle
echo ""
echo ">>> Inyectando librerías nativas para todos los ABIs en $JNILIBS_DIR"
for entry in "${TARGETS[@]}"; do
  triple="${entry%%:*}"
  abi="${entry##*:}"

  # dx pone el .so final en alguno de estos lugares
  SO_SRC=""
  for candidate in \
    "$ROOT/target/$triple/android-release/libdioxusmain.so" \
    "$ROOT/target/$triple/release/libdioxusmain.so"; do
    if [ -f "$candidate" ]; then
      SO_SRC="$candidate"
      break
    fi
  done
  # búsqueda de respaldo (excluye deps/ para evitar versiones intermedias)
  if [ -z "$SO_SRC" ]; then
    SO_SRC="$(find "$ROOT/target/$triple" -name "libdioxusmain.so" \
      -not -path '*/deps/*' -print -quit 2>/dev/null || true)"
  fi

  if [ -n "$SO_SRC" ] && [ -f "$SO_SRC" ]; then
    echo "  $abi  ←  $SO_SRC"
    mkdir -p "$JNILIBS_DIR/$abi"
    cp "$SO_SRC" "$JNILIBS_DIR/$abi/libdioxusmain.so"
  else
    echo "  ADVERTENCIA: no se encontró libdioxusmain.so para $triple ($abi) — se omite."
  fi
done

# 4. Ejecutar parche post-build reutilizable (ProGuard, minSdk, iconos, firma, zipalign…)
bash "$SCRIPT_DIR/post-dx-patch.sh"

echo ""
echo ">>> APK/AAB generados:"
if [ -n "${APP_DIR:-}" ]; then
  find "$APP_DIR" -type f \( -iname "*.apk" -o -iname "*.aab" \) -print
else
  find . -type f \( -iname "*.apk" -o -iname "*.aab" \) -print
fi
