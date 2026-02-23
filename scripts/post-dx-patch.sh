#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT="$(cd "$SCRIPT_DIR/.." && pwd)"

ICONS_DIR="${ICONS_DIR:-}"
JKS_FILE="${JKS_FILE:-}"
KEY_PASSWORD="${KEY_PASSWORD:-}"
INSTALL="${INSTALL:-false}"
SINGLE_ARCH="${SINGLE_ARCH:-false}"

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

  # --- Remove deprecated extractNativeLibs from AndroidManifest (AGP 8+ warns about it).
  # Native lib extraction is handled via useLegacyPackaging=true in build.gradle.kts instead.
  if [ -f "app/src/main/AndroidManifest.xml" ]; then
    echo ">>> Eliminando android:extractNativeLibs del AndroidManifest (deprecado en AGP 8+)"
    sed -i -E 's/\s*android:extractNativeLibs\s*=\s*"(true|false)"//g' app/src/main/AndroidManifest.xml || true
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

    # --- Patch MainActivity to handle Android back with onKeyDown + view hierarchy search
    # onBackPressed is deprecated in API 33+ and unreliable with WryActivity.
    # onKeyDown + recursive WebView search in view hierarchy works on all versions.
    echo ">>> Parcheando MainActivity para manejar botón Back (onKeyDown + view hierarchy) si existe"
    while IFS= read -r activity; do
    [ -f "$activity" ] || continue
    echo ">>> Revisando $activity"
    # Skip if already patched
    if grep -F -q 'findWebViewEverywhere' "$activity"; then
      echo "    - Patch ya presente en $activity, omitiendo"
      continue
    fi

    if [[ "$activity" == *.kt ]]; then
      python3 - "$activity" <<'PYEOF'
import sys, re
path = sys.argv[1]
src = open(path, 'r', encoding='utf-8').read()

if 'findWebViewEverywhere' in src:
    print('already-patched')
    sys.exit(0)

# Extract package declaration and any imports/typealiases above the class
# The generated file looks like:
#   package dev.dioxus.main;
#   import ...;
#   typealias ...;
#   class MainActivity : WryActivity()
# We rewrite it keeping the header but replacing the class declaration with a full body.
lines = src.splitlines(keepends=True)
header_lines = []
class_line_idx = -1
for i, line in enumerate(lines):
    stripped = line.strip()
    if stripped.startswith('class ') or (stripped.startswith('open class ') or stripped.startswith('abstract class ')):
        class_line_idx = i
        break
    header_lines.append(line)

if class_line_idx == -1:
    print('no-class')
    sys.exit(1)

# Extract the parent class from the class declaration
class_decl = lines[class_line_idx].strip().rstrip('{').rstrip()
# e.g. "class MainActivity : WryActivity()"
# We'll keep the inheritance as-is and add a body

new_class = class_decl + ' {\n'
new_class += '''
    override fun onCreate(savedInstanceState: android.os.Bundle?) {
        super.onCreate(savedInstanceState)
        onBackPressedDispatcher.addCallback(this, object : androidx.activity.OnBackPressedCallback(true) {
            override fun handleOnBackPressed() {
                val wv = findWebViewEverywhere()
                if (wv != null) {
                    // Dioxus uses pushState/replaceState so canGoBack() is unreliable.
                    // Check pathname via JS: if we are at root ("/") exit, otherwise go back.
                    wv.evaluateJavascript(
                        "(function(){var p=window.location.pathname;if(p==='/'||p===''||p==='/index.html'){return 1;}window.history.back();return 0;})()"
                    ) { result ->
                        if (result != "0") {
                            isEnabled = false
                            onBackPressedDispatcher.onBackPressed()
                            isEnabled = true
                        }
                    }
                } else {
                    isEnabled = false
                    onBackPressedDispatcher.onBackPressed()
                    isEnabled = true
                }
            }
        })
    }

    private fun findWebViewEverywhere(): android.webkit.WebView? {
        // 1. Search view hierarchy from decor view
        findWebViewInHierarchy(window.decorView)?.let { return it }
        // 2. Reflection over fields in this class and all superclasses
        var clazz: Class<*>? = this.javaClass
        while (clazz != null && clazz != Any::class.java) {
            for (field in clazz.declaredFields) {
                try {
                    field.isAccessible = true
                    val v = field.get(this)
                    if (v is android.webkit.WebView) return v
                } catch (_: Throwable) {}
            }
            clazz = clazz.superclass
        }
        return null
    }

    private fun findWebViewInHierarchy(view: android.view.View): android.webkit.WebView? {
        if (view is android.webkit.WebView) return view
        if (view is android.view.ViewGroup) {
            for (i in 0 until view.childCount) {
                val found = findWebViewInHierarchy(view.getChildAt(i))
                if (found != null) return found
            }
        }
        return null
    }
}
'''

result = ''.join(header_lines) + new_class
open(path, 'w', encoding='utf-8').write(result)
print('patched')
PYEOF
      echo "    - Patch intentado en $activity"
    fi

    if [[ "$activity" == *.java ]]; then
      python3 - "$activity" <<'PYEOF'
import sys
path = sys.argv[1]
src = open(path, 'r', encoding='utf-8').read()
if 'findWebViewEverywhere' in src:
    print('already-patched')
    sys.exit(0)
lines = src.splitlines(keepends=True)
header_lines = []
class_line_idx = -1
for i, line in enumerate(lines):
    if line.strip().startswith('class ') or line.strip().startswith('public class '):
        class_line_idx = i
        break
    header_lines.append(line)
if class_line_idx == -1:
    print('no-class')
    sys.exit(1)
class_decl = lines[class_line_idx].strip().rstrip('{').rstrip()
new_class = class_decl + ' {\n'
new_class += '''
    @Override
    public void onCreate(android.os.Bundle savedInstanceState) {
        super.onCreate(savedInstanceState);
        final androidx.activity.OnBackPressedDispatcher dispatcher = getOnBackPressedDispatcher();
        dispatcher.addCallback(this, new androidx.activity.OnBackPressedCallback(true) {
            @Override
            public void handleOnBackPressed() {
                android.webkit.WebView wv = findWebViewEverywhere();
                if (wv != null) {
                    // Dioxus uses pushState/replaceState so canGoBack() is unreliable.
                    // Check pathname via JS: if we are at root ("/") exit, otherwise go back.
                    wv.evaluateJavascript(
                        "(function(){var p=window.location.pathname;if(p==='/'||p===''||p==='/index.html'){return 1;}window.history.back();return 0;})()",
                        result -> {
                            if (!"0".equals(result)) {
                                setEnabled(false);
                                dispatcher.onBackPressed();
                                setEnabled(true);
                            }
                        });
                } else {
                    setEnabled(false);
                    dispatcher.onBackPressed();
                    setEnabled(true);
                }
            }
        });
    }

    private android.webkit.WebView findWebViewEverywhere() {
        android.webkit.WebView wv = findWebViewInHierarchy(getWindow().getDecorView());
        if (wv != null) return wv;
        Class<?> clazz = this.getClass();
        while (clazz != null && !clazz.equals(Object.class)) {
            for (java.lang.reflect.Field field : clazz.getDeclaredFields()) {
                try {
                    field.setAccessible(true);
                    Object v = field.get(this);
                    if (v instanceof android.webkit.WebView) return (android.webkit.WebView) v;
                } catch (Throwable t) {}
            }
            clazz = clazz.getSuperclass();
        }
        return null;
    }

    private android.webkit.WebView findWebViewInHierarchy(android.view.View view) {
        if (view instanceof android.webkit.WebView) return (android.webkit.WebView) view;
        if (view instanceof android.view.ViewGroup) {
            android.view.ViewGroup vg = (android.view.ViewGroup) view;
            for (int i = 0; i < vg.getChildCount(); i++) {
                android.webkit.WebView found = findWebViewInHierarchy(vg.getChildAt(i));
                if (found != null) return found;
            }
        }
        return null;
    }
}
'''
result = ''.join(header_lines) + new_class
open(path, 'w', encoding='utf-8').write(result)
print('patched')
PYEOF
    fi

    done < <(find app/src -type f \( -name 'MainActivity.kt' -o -name 'MainActivity.java' \) -print 2>/dev/null || true)

  echo ">>> Ejecutando ./gradlew clean"
  ./gradlew clean || true

  # --- Patch build.gradle.kts: añadir useLegacyPackaging=true en release
  # Necesario para que Gradle extraiga el .so del APK antes de cargarlo.
  # Trabaja junto con extractNativeLibs=true en el manifest.
  MODULE_BUILD="app/build.gradle.kts"
  if [ -f "$MODULE_BUILD" ] && ! grep -q "useLegacyPackaging" "$MODULE_BUILD"; then
    echo ">>> Añadiendo useLegacyPackaging=true al bloque release en $MODULE_BUILD"
    python3 - "$MODULE_BUILD" <<'PYEOF'
import sys, re
path = sys.argv[1]
content = open(path).read()
# Insert packaging block inside getByName("release") { ... }
pattern = r'(getByName\("release"\)\s*\{)'
replacement = r'''\1
            packaging {
                jniLibs.useLegacyPackaging = true
            }'''
new = re.sub(pattern, replacement, content, count=1)
open(path, 'w').write(new)
print('useLegacyPackaging injected' if new != content else 'no change')
PYEOF
  else
    echo ">>> useLegacyPackaging ya presente en $MODULE_BUILD o archivo no encontrado"
  fi
  # Si SINGLE_ARCH=true (una sola arquitectura de prueba), no se inyectan splits
  # para evitar generar APKs innecesarios de otras ABIs.
  MODULE_BUILD="app/build.gradle.kts"
  if [ "$SINGLE_ARCH" = "true" ]; then
    echo ">>> SINGLE_ARCH=true — omitiendo inyección de splits ABI"
  elif [ -f "$MODULE_BUILD" ]; then
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
  fi  # end SINGLE_ARCH check

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

      # Borrar los XMLs de adaptive icon en mipmap-anydpi-v26 para que Android use
      # directamente los PNGs por densidad. Si estos XMLs existen (aunque apunten a un
      # drawable separado) Android 26+ puede ignorar los PNGs y mostrar el icono por
      # defecto. La solución: eliminarlos y no recrearlos.
      echo ">>> Eliminando XMLs de adaptive icon en mipmap-anydpi-v26 (usando solo PNGs por densidad)"
      rm -f app/src/main/res/mipmap-anydpi-v26/ic_launcher.xml || true
      rm -f app/src/main/res/mipmap-anydpi-v26/ic_launcher_round.xml || true

      echo ">>> Iconos generados en res/mipmap-* (sin adaptive icon XML)"
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
    # Deduplicate ALL_APKS into UNIQUE_APKS
    UNIQUE_APKS=()
    for apk in "${ALL_APKS[@]}"; do
      skip=0
      for u in "${UNIQUE_APKS[@]}"; do
        if [ "$u" = "$apk" ]; then skip=1; break; fi
      done
      [ $skip -eq 0 ] && UNIQUE_APKS+=("$apk")
    done

    # If single-architecture build or only one apk produced, pick one primary APK
    if [ "${SINGLE_ARCH:-false}" = "true" ] || [ "${#UNIQUE_APKS[@]}" -le 1 ]; then
      PRIMARY_APK=""
      # Prefer a universal artifact if present
      for apk in "${UNIQUE_APKS[@]}"; do
        lname=$(basename "$apk" | tr '[:upper:]' '[:lower:]')
        if echo "$lname" | grep -q "universal" || echo "$lname" | grep -q "app-release"; then
          PRIMARY_APK="$apk"
          break
        fi
      done
      # fallback to first
      if [ -z "$PRIMARY_APK" ] && [ "${#UNIQUE_APKS[@]}" -gt 0 ]; then
        PRIMARY_APK="${UNIQUE_APKS[0]}"
      fi

      # Remove other APKs in the output dir to keep a single artifact for pipelines
      if [ -n "$PRIMARY_APK" ]; then
        for f in "${UNIQUE_APKS[@]}"; do
          if [ "$f" != "$PRIMARY_APK" ]; then
            rm -f "$f" || true
          fi
        done
        # rebuild ALL_APKS to only include primary
        ALL_APKS=("$PRIMARY_APK")
      fi
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
    # Renombrar/copiar los APK firmados/alineados con nombres legibles por ABI
    RELEASE_ASSETS_FILE="$APK_OUTPUT_DIR/release-assets.txt"
    : > "$RELEASE_ASSETS_FILE"
    # Determinar nombre y versión de la app (desde Cargo.toml)
    APP_NAME_RAW="$(sed -n 's/^name\s*=\s*"\(.*\)"/\1/p' "$ROOT/Cargo.toml" | head -n1 || true)"
    # Preferir versión desde etiqueta Git (env GIT_TAG o GITHUB_REF_NAME si están presentes)
    if [ -n "${GIT_TAG:-}" ]; then
      APP_VERSION_RAW="$GIT_TAG"
    elif [ -n "${GITHUB_REF_NAME:-}" ]; then
      APP_VERSION_RAW="$GITHUB_REF_NAME"
    else
      APP_VERSION_RAW="$(git -C "$ROOT" describe --tags --exact-match 2>/dev/null || true)"
      if [ -z "$APP_VERSION_RAW" ]; then
        APP_VERSION_RAW="$(git -C "$ROOT" rev-parse --short HEAD 2>/dev/null || true)"
      fi
    fi
    # Si no se encontraba tag/git, fallback a Cargo.toml
    if [ -z "$APP_VERSION_RAW" ]; then
      APP_VERSION_RAW="$(sed -n 's/^version\s*=\s*"\(.*\)"/\1/p' "$ROOT/Cargo.toml" | head -n1 || true)"
    fi
    # Normalizar: quitar prefijo 'v' si existe
    APP_VERSION="$(echo "$APP_VERSION_RAW" | sed 's/^v//i')"
    if [ -z "$APP_NAME_RAW" ]; then
      APP_NAME_RAW="app"
    fi
    if [ -z "$APP_VERSION" ]; then
      APP_VERSION="0.0.0"
    fi
    # slug: kebab-case, ascii lowercase, replace non-alnum with '-'
    APP_NAME="$(echo "$APP_NAME_RAW" | iconv -f utf8 -t ascii//TRANSLIT 2>/dev/null | tr '[:upper:]' '[:lower:]' | sed 's/[^a-z0-9]+/-/g' | sed 's/^-\|-$//g')"

    for APK_IN in "${ALL_APKS[@]}"; do
      BASENAME="$(basename "$APK_IN" .apk)"
      SIGNED="${APK_OUTPUT_DIR}/${BASENAME}-signed.apk"
      ALIGNED="${APK_OUTPUT_DIR}/${BASENAME}-aligned.apk"

      # Determinar ABI a partir del nombre del archivo
      ABI_LABEL="unknown"
      lname="${BASENAME,,}"
      if echo "$lname" | grep -q "universal"; then
        ABI_LABEL="universal"
      elif echo "$lname" | grep -q "arm64" || echo "$lname" | grep -q "aarch64"; then
        ABI_LABEL="arm64-v8a"
      elif echo "$lname" | grep -q "arm" || echo "$lname" | grep -q "armeabi" || echo "$lname" | grep -q "armv7"; then
        ABI_LABEL="armeabi-v7a"
      elif echo "$lname" | grep -q "x86_64" || echo "$lname" | grep -q "x86-64"; then
        ABI_LABEL="x86_64"
      elif echo "$lname" | grep -q "x86" || echo "$lname" | grep -q "i686"; then
        ABI_LABEL="x86"
      fi

      # Si solo hay un APK (SINGLE_ARCH o lista de ALL_APKS de tamaño 1) o el nombre
      # parece ser un release genérico (app-release), tratarlo como "universal" para
      # evitar generar nombres "unknown-...".
      if [ "${SINGLE_ARCH:-false}" = "true" ] || [ "${#ALL_APKS[@]}" -eq 1 ] || echo "$lname" | grep -Eq "release|app-release|app-release-aligned"; then
        ABI_LABEL="universal"
      fi

      # Nombre bonito: <arch>-<app>-v<version>.apk (prefiere signed, luego aligned, luego original)
      OUT_NAME="${APK_OUTPUT_DIR}/${ABI_LABEL}-${APP_NAME}-v${APP_VERSION}.apk"
      if [ -f "$SIGNED" ]; then
        cp "$SIGNED" "$OUT_NAME" || true
        echo "Copied signed $SIGNED -> $OUT_NAME"
        echo "$OUT_NAME" >> "$RELEASE_ASSETS_FILE"
      elif [ -f "$ALIGNED" ]; then
        cp "$ALIGNED" "$OUT_NAME" || true
        echo "Copied aligned $ALIGNED -> $OUT_NAME"
        echo "$OUT_NAME" >> "$RELEASE_ASSETS_FILE"
      elif [ -f "$APK_IN" ]; then
        cp "$APK_IN" "$OUT_NAME" || true
        echo "Copied raw $APK_IN -> $OUT_NAME"
        echo "$OUT_NAME" >> "$RELEASE_ASSETS_FILE"
      else
        echo "Warning: no artifact found for $APK_IN"
      fi
    done

    # También exponer el APK universal con nombre estandar en la raíz para compatibilidad
    if [ -f "$APK_OUTPUT_DIR/app-universal-release-signed.apk" ]; then
      cp "$APK_OUTPUT_DIR/app-universal-release-signed.apk" "app-release-signed.apk" || true
    else
      # intentar copiar el que detectamos antes
      UNIV_BASENAME="$(basename "$UNIVERSAL_APK" .apk)"
      UNIV_SIGNED="${APK_OUTPUT_DIR}/${UNIV_BASENAME}-signed.apk"
      UNIV_ALIGNED="${APK_OUTPUT_DIR}/${UNIV_BASENAME}-aligned.apk"
      [ -f "$UNIV_SIGNED"  ] && cp "$UNIV_SIGNED"  "app-release-signed.apk"  || true
      [ -f "$UNIV_ALIGNED" ] && cp "$UNIV_ALIGNED" "app-release-aligned.apk" || true
    fi

    # También crear en la raíz un APK con nombre legible: <arch>-<slug>-v<version>.apk
    # Preferimos la variante 'universal' generada en $APK_OUTPUT_DIR
    NICE_BASENAME="universal-${APP_NAME}-v${APP_VERSION}.apk"
    if [ -f "${APK_OUTPUT_DIR}/${NICE_BASENAME}" ]; then
      cp "${APK_OUTPUT_DIR}/${NICE_BASENAME}" "${NICE_BASENAME}" || true
    else
      # Buscar cualquier artefacto ya renombrado en APK_OUTPUT_DIR que incluya el slug+version
      match=$(find "$APK_OUTPUT_DIR" -maxdepth 1 -type f -name "*-${APP_NAME}-v${APP_VERSION}.apk" -print -quit 2>/dev/null || true)
      if [ -n "$match" ]; then
        base=$(basename "$match")
        cp "$match" "$base" || true
      else
        # Fallback: copiar app-release-aligned/signed si existen y renombrarlas
        if [ -f "app-release-aligned.apk" ]; then
          cp "app-release-aligned.apk" "${NICE_BASENAME}" || true
        elif [ -f "app-release-signed.apk" ]; then
          cp "app-release-signed.apk" "${NICE_BASENAME}" || true
        fi
      fi
    fi

    # Eliminar APKs duplicados en el directorio de salida cuando solo usamos una arquitectura
    if [ "${SINGLE_ARCH:-false}" = "true" ] || [ "${#ALL_APKS[@]}" -le 1 ]; then
        echo ">>> SINGLE_ARCH o único APK — limpiando duplicados en $APK_OUTPUT_DIR"
      # Encontrar todos los APKs bajo el directorio de la app y eliminar los que no sean el NICE_BASENAME
        find "$APK_OUTPUT_DIR" -type f -name "*.apk" | while read -r p; do
        base=$(basename "$p")
        if [ "$base" != "${NICE_BASENAME}" ] && [ "$base" != "app-release-signed.apk" ] && [ "$base" != "app-release-aligned.apk" ]; then
          echo ">>> Removing duplicate APK: $p"
          rm -f "$p" || true
        fi
      done
      # Asegurar que solo queden el NICE_BASENAME y los app-release-*.apk en outputs
    fi

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
