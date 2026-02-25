#!/usr/bin/env bash
set -euo pipefail
echo "Generando bundle en docs..."
dx bundle --out-dir docs

if [ -d docs/public ]; then
  echo "Moviendo assets de docs/public a docs..."
  mv docs/public/* docs || true
  rmdir docs/public || true
fi

if [ -f docs/index.html ]; then
  echo "Creando docs/404.html..."
  cp docs/index.html docs/404.html
fi

echo "Build completado. Revisa 'docs' para los archivos estáticos."
