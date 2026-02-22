#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

MIN_SDK="${MIN_SDK:-21}"
ICONS_DIR="${ICONS_DIR:-}"
JKS_FILE="${JKS_FILE:-}"
KEY_PASSWORD="${KEY_PASSWORD:-}"
INSTALL="${INSTALL:-false}"

DX_APP_DIR="$(find "$ROOT/target/dx" -type d -path '*/release/android/app' -print -quit 2>/dev/null || true)"
if [ -n "$DX_APP_DIR" ]; then
  echo ">>> Directorio de app de Android encontrado: $DX_APP_DIR"
  cd "$DX_APP_DIR"

  # --- Fix gradle properties: comment deprecated defaults and enable configuration cache
  if [ -f "gradle.properties" ]; then
    echo ">>> Ajustando gradle.properties: comentar defaults y habilitar configuration cache"
    cp gradle.properties gradle.properties.bak || true
    # comment deprecated default buildfeatures flag
    sed -i -E "s/^\s*(android.defaults.buildfeatures.buildconfig\s*=\s*true)/# \1/" gradle.properties || true
    # enable suppress of source/target deprecation warning
    if ! grep -q "android.javaCompile.suppressSourceTargetDeprecationWarning" gradle.properties; then
      echo "android.javaCompile.suppressSourceTargetDeprecationWarning=true" >> gradle.properties
    else
      sed -i -E "s/^android.javaCompile.suppressSourceTargetDeprecationWarning\s*=.*/android.javaCompile.suppressSourceTargetDeprecationWarning=true/" gradle.properties || true
    fi
    # enable configuration cache
    if ! grep -q "org.gradle.configuration-cache" gradle.properties; then
      echo "org.gradle.configuration-cache=true" >> gradle.properties
    else
      sed -i -E "s/^org.gradle.configuration-cache\s*=.*/org.gradle.configuration-cache=true/" gradle.properties || true
    fi
    # reduce warnings noise
    if ! grep -q "org.gradle.warning.mode" gradle.properties; then
      echo "org.gradle.warning.mode=none" >> gradle.properties
    else
      sed -i -E "s/^org.gradle.warning.mode\s*=.*/org.gradle.warning.mode=none/" gradle.properties || true
    fi
  fi

  # --- Ensure AndroidManifest does not set extractNativeLibs (manifest merger warnings)
  if [ -f "app/src/main/AndroidManifest.xml" ]; then
    echo ">>> Eliminando android:extractNativeLibs de manifests si existe"
    sed -i -E "s/\s*android:extractNativeLibs\s*=\s*\"(true|false)\"//g" app/src/main/AndroidManifest.xml || true
  fi

  # --- Deploy ProGuard/R8 rules: prefer repo file if present
  PROGUARD_FILE="app/proguard-rules.pro"
  echo ">>> Instalando reglas ProGuard/R8 en $PROGUARD_FILE (si existe en repo)"
  mkdir -p "$(dirname "$PROGUARD_FILE")"
  REPO_RULES="$ROOT/scripts/android-proguard-rules.pro"
  if [ -f "$REPO_RULES" ]; then
    cp "$REPO_RULES" "$PROGUARD_FILE" || true
  else
    echo ">>> Archivo $REPO_RULES no encontrado — usando reglas por defecto embebidas"
    cat > "$PROGUARD_FILE" <<'RPROB'
  # Default R8 rules added by post-dx-patch
  -keep public class com.google.vending.licensing.ILicensingService { void <init>(); }
  -keep public class com.android.vending.licensing.ILicensingService { void <init>(); }
  -keep class android.support.annotation.Keep { void <init>(); }
  -keep class androidx.annotation.Keep { void <init>(); }
  -keep class androidx.webkit.WebViewClientCompat { void <init>(); }
  -keep class androidx.versionedparcelable.ParcelImpl { void <init>(); }
  -keep class * extends androidx.startup.Initializer { void <init>(); }
RPROB
  fi

  echo ">>> Parcheando minSdk a $MIN_SDK en archivos Gradle generados (si existen)"
  GRADLE_FILES="$(find . -maxdepth 4 -type f \( -name 'build.gradle.kts' -o -name 'build.gradle' \) -print -quit)"
  if [ -n "$GRADLE_FILES" ]; then
    while IFS= read -r gf; do
      [ -f "$gf" ] || continue
      echo ">>> Parcheando $gf"
      cp "$gf" "$gf.bak" || true
      if [[ "$gf" == *.kts ]]; then
        sed -i -E "s/(minSdk)\s*=\s*[0-9]+/\1 = $MIN_SDK/g" "$gf" || true
      else
        sed -i -E "s/(minSdkVersion)\s*(=\s*)?[0-9]+/\1 $MIN_SDK/g" "$gf" || true
        sed -i -E "s/(minSdk)\s*=\s*[0-9]+/\1 = $MIN_SDK/g" "$gf" || true
      fi
    done <<EOF
$(find . -maxdepth 4 -type f \( -name 'build.gradle.kts' -o -name 'build.gradle' \) -print)
EOF
  else
    echo ">>> No se encontraron archivos build.gradle(.kts) para parchear."
  fi

  echo ">>> Ejecutando ./gradlew clean"
  ./gradlew clean || true

  # --- Ensure module build will produce a universal APK + per-ABI splits
  MODULE_BUILD="app/build.gradle.kts"
  if [ -f "$MODULE_BUILD" ]; then
    if ! grep -q "isUniversalApk" "$MODULE_BUILD"; then
      echo ">>> Añadiendo splits ABI para generar APK universal en $MODULE_BUILD"
      python3 - "$MODULE_BUILD" <<'PYEOF'
import sys
path = sys.argv[1]
content = open(path).read()
splits_block = '''
    splits {
        abi {
            isEnable = true
            isUniversalApk = true
            include("armeabi-v7a", "arm64-v8a", "x86_64")
        }
    }
'''
idx = content.find('android {')
if idx != -1:
    depth = 0
    i = idx
    while i < len(content):
        if content[i] == '{':
            depth += 1
        elif content[i] == '}':
            depth -= 1
            if depth == 0:
                content = content[:i] + splits_block + content[i:]
                break
        i += 1
open(path, 'w').write(content)
print('splits block injected OK')
PYEOF
    else
      echo ">>> splits ABI ya presente en $MODULE_BUILD"
    fi
  fi

  # Icon handling
  if [ -n "$ICONS_DIR" ] && [ -d "$ICONS_DIR/res" ]; then
    echo ">>> Reemplazando iconos desde: ${ICONS_DIR}"
    find app/src/main/res -name "*.webp" -type f -delete || true
    cp -r "${ICONS_DIR}/res" app/src/main/ || true
    rm -f app/src/main/res/mipmap-anydpi-v26/ic_launcher.xml || true
  else
    ASSET_ICON="$ROOT/assets/android-chrome-512x512.png"
    if [ -f "$ASSET_ICON" ]; then
      echo ">>> Generando iconos desde $ASSET_ICON"

      # Borrar TODOS los iconos que dx generó (webp + xml + png previos) para partir limpio
      find app/src/main/res -type f \
        \( -name "ic_launcher*.webp" -o -name "ic_launcher*.png" -o -name "ic_launcher*.xml" \) \
        -delete || true
      find app/src/main/res -name "*.webp" -type f -delete || true

      # Detectar herramienta de conversión (ImageMagick 7 o 6)
      if command -v magick >/dev/null 2>&1; then
        CONVERT_BIN="magick"
      elif command -v convert >/dev/null 2>&1; then
        CONVERT_BIN="convert"
      else
        CONVERT_BIN=""
      fi

      # Generar ic_launcher.png e ic_launcher_round.png en cada densidad
      # Formato: <carpeta>:<tamaño_px>
      DENSITY_MAP="mipmap-mdpi:48 mipmap-hdpi:72 mipmap-xhdpi:96 mipmap-xxhdpi:144 mipmap-xxxhdpi:192"
      for pair in $DENSITY_MAP; do
        dir="${pair%%:*}"
        size="${pair##*:}"
        target_dir="app/src/main/res/$dir"
        mkdir -p "$target_dir"
        for icon_name in ic_launcher ic_launcher_round; do
          target_file="$target_dir/${icon_name}.png"
          if [ -n "$CONVERT_BIN" ]; then
            if [ "$CONVERT_BIN" = "magick" ]; then
              magick "$ASSET_ICON" -resize "${size}x${size}" "$target_file" || cp "$ASSET_ICON" "$target_file" || true
            else
              "$CONVERT_BIN" "$ASSET_ICON" -resize "${size}x${size}" "$target_file" || cp "$ASSET_ICON" "$target_file" || true
            fi
          else
            cp "$ASSET_ICON" "$target_file" || true
          fi
        done
        echo "  $dir (${size}x${size})"
      done

      # Color de fondo para el adaptive icon (blanco por defecto)
      mkdir -p "app/src/main/res/values"
      cat > "app/src/main/res/values/ic_launcher_background.xml" <<'COLOREOF'
<?xml version="1.0" encoding="utf-8"?>
<resources>
    <color name="ic_launcher_background">#FFFFFF</color>
</resources>
COLOREOF

      # Adaptive icon v26+:
      #   <background>  = color sólido (no el mismo PNG, eso causaba el icono duplicado)
      #   <foreground>  = PNG del icono
      IC_DIR="app/src/main/res/mipmap-anydpi-v26"
      mkdir -p "$IC_DIR"
      for xml_name in ic_launcher ic_launcher_round; do
        cat > "$IC_DIR/${xml_name}.xml" <<'ICEOF'
<?xml version="1.0" encoding="utf-8"?>
<adaptive-icon xmlns:android="http://schemas.android.com/apk/res/android">
    <background android:drawable="@color/ic_launcher_background"/>
    <foreground android:drawable="@mipmap/ic_launcher"/>
</adaptive-icon>
ICEOF
      done

      echo ">>> Iconos generados en res/mipmap-*"
    else
      echo ">>> ICONS_DIR no establecido y no existe $ASSET_ICON — omitiendo reemplazo de iconos."
    fi
  fi

  echo ">>> Ensamblando release con Gradle"
  ./gradlew assembleRelease || true

  # --- APK handling: con splits activos Gradle genera varios APKs (uno por ABI + universal)
  APK_OUTPUT_DIR="app/build/outputs/apk/release"
  UNIVERSAL_APK=""
  ALL_APKS=()

  if [ -d "$APK_OUTPUT_DIR" ]; then
    # Buscar APK universal (distintos nombres posibles según versión de Gradle/dx)
    for pattern in \
      "$APK_OUTPUT_DIR/app-universal-release-unsigned.apk" \
      "$APK_OUTPUT_DIR/app-universal-release.apk" \
      "$APK_OUTPUT_DIR/app-release-unsigned.apk" \
      "$APK_OUTPUT_DIR/app-release.apk"; do
      if [ -f "$pattern" ]; then
        UNIVERSAL_APK="$pattern"
        break
      fi
    done
    # Si no hay uno explícito, coger el primero disponible
    if [ -z "$UNIVERSAL_APK" ]; then
      UNIVERSAL_APK="$(find "$APK_OUTPUT_DIR" -name "*.apk" -print -quit 2>/dev/null || true)"
    fi
    # Lista completa
    while IFS= read -r apk; do
      ALL_APKS+=("$apk")
    done < <(find "$APK_OUTPUT_DIR" -name "*.apk" -print 2>/dev/null || true)
  fi

  if [ -n "$UNIVERSAL_APK" ]; then
    echo ">>> APK universal: $UNIVERSAL_APK"
    printf '>>> APK encontrados:\n'
    printf '    %s\n' "${ALL_APKS[@]}"

    ZIPALIGN_BIN="$(command -v zipalign || true)"
    APKSIGNER_BIN="$(command -v apksigner || true)"

    for APK_IN in "${ALL_APKS[@]}"; do
      BASENAME="$(basename "$APK_IN" .apk)"
      ALIGNED="${APK_OUTPUT_DIR}/${BASENAME}-aligned.apk"
      SIGNED="${APK_OUTPUT_DIR}/${BASENAME}-signed.apk"

      if [ -n "$ZIPALIGN_BIN" ]; then
        "$ZIPALIGN_BIN" -v -p 4 "$APK_IN" "$ALIGNED" || true
      else
        cp "$APK_IN" "$ALIGNED" || true
      fi

      if [ -n "$APKSIGNER_BIN" ] && [ -n "$JKS_FILE" ] && [ -f "$JKS_FILE" ] && [ -n "$KEY_PASSWORD" ]; then
        "$APKSIGNER_BIN" sign --ks "$JKS_FILE" --ks-pass pass:"$KEY_PASSWORD" --out "$SIGNED" "$ALIGNED" || true
        echo "  firmado → $SIGNED"
      fi
    done

    # Copiar universal alineado/firmado a la raíz del app dir (compatibilidad con pasos anteriores)
    UNIV_BASENAME="$(basename "$UNIVERSAL_APK" .apk)"
    UNIV_ALIGNED="${APK_OUTPUT_DIR}/${UNIV_BASENAME}-aligned.apk"
    UNIV_SIGNED="${APK_OUTPUT_DIR}/${UNIV_BASENAME}-signed.apk"
    [ -f "$UNIV_SIGNED"  ] && cp "$UNIV_SIGNED"  "app-release-signed.apk"  || true
    [ -f "$UNIV_ALIGNED" ] && cp "$UNIV_ALIGNED" "app-release-aligned.apk" || true

    if [ "$INSTALL" = "true" ]; then
      INSTALL_APK=""
      [ -f "app-release-signed.apk"  ] && INSTALL_APK="app-release-signed.apk"
      [ -z "$INSTALL_APK" ] && [ -f "app-release-aligned.apk" ] && INSTALL_APK="app-release-aligned.apk"
      [ -z "$INSTALL_APK" ] && INSTALL_APK="$UNIVERSAL_APK"
      adb install -r "$INSTALL_APK" || true
    fi
  else
    echo ">>> No se encontró ningún APK en $APK_OUTPUT_DIR — omitiendo zipalign/apksigner."
  fi

  cd "$ROOT"
else
  echo ">>> No se encontró target/dx/*/release/android/app — omitiendo pasos de iconos/optimización." 
fi

echo ">>> Patch completo"
