#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
README="$ROOT/README.md"
CARGO="$ROOT/Cargo.toml"

if [ ! -f "$CARGO" ]; then
  echo "Cargo.toml not found"
  exit 1
fi

# Extraer versión de Cargo.toml
VERSION="$(sed -n 's/^version\s*=\s*"\(.*\)"/\1/p' "$CARGO" | head -n1 | tr -d '\r')"
if [ -z "$VERSION" ]; then
  echo "No version found in Cargo.toml"
  exit 1
fi

echo ">>> Sincronizando versión $VERSION en $README"

if [ ! -f "$README" ]; then
  echo "README.md not found, aborting"
  exit 1
fi

# 1) Actualizar badge shields.io (si existe)
sed -E -i "s@(https://img.shields.io/badge/Version-)[^-]+@\1${VERSION}@" "$README" || true

# # 2) Insertar/actualizar una línea 'Versión: X' justo después de la primera aparición del badge
# BADGE_LINE=$(grep -n -m1 'https://img.shields.io/badge/Version-' "$README" | cut -d: -f1 || true)
# if [ -n "$BADGE_LINE" ]; then
#   NEXT_LINE=$((BADGE_LINE + 1))
#   LINE_CONTENT=$(sed -n "${NEXT_LINE}p" "$README" || true)
#   if echo "$LINE_CONTENT" | grep -q '^Versión:'; then
#     # Reemplazar la línea existente
#     sed -i "${NEXT_LINE}s@^Versión:.*@Versión: ${VERSION}@" "$README" || true
#   else
#     # Insertar nueva línea después del badge
#     awk -v ln="$BADGE_LINE" -v ver="$VERSION" 'NR==ln{print; print "Versión: " ver; next} {print}' "$README" > "$README.tmp" && mv "$README.tmp" "$README"
#   fi
# else
#   # Si no hay badge, añadir la línea al inicio
#   printf "Versión: %s\n\n" "$VERSION" > "$README.tmp"
#   cat "$README" >> "$README.tmp"
#   mv "$README.tmp" "$README"
# fi

echo ">>> README.md actualizado con versión $VERSION"

exit 0
