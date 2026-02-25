#!/usr/bin/env bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

echo ">>> Ejecutando build para web (GitHub Pages)..."
bash "$SCRIPT_DIR/build-pages.sh"

echo ""
echo ">>> Ejecutando build Android (si está configurado)..."
bash "$SCRIPT_DIR/android-build.sh" || {
  echo "ADVERTENCIA: el script android-build.sh falló o no está configurado; continuar..."
}

echo ""
echo "All build steps completed. Revisa la salida anterior para detalles."
