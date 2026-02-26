#!/usr/bin/env bash
set -euo pipefail
echo "Generando bundle en docs..."
dx bundle --out-dir docs

if [ -d docs/public ]; then
  echo "Moviendo assets de docs/public a docs..."
  # Use rsync to merge directories and preserve files; then remove the source dir
  if command -v rsync >/dev/null 2>&1; then
    rsync -a docs/public/ docs/ || true
    rm -rf docs/public || true
  else
    # Fallback: move entries individually, merging directories when needed
    shopt -s dotglob || true
    for f in docs/public/*; do
      if [ -e "$f" ]; then
        base=$(basename "$f")
        if [ -d "docs/$base" ] && [ -d "$f" ]; then
          mv "$f"/* "docs/$base/" 2>/dev/null || true
          rm -rf "$f" || true
        else
          mv -f "$f" docs/ || true
        fi
      fi
    done
    rmdir docs/public 2>/dev/null || true
  fi
fi

if [ -f docs/index.html ]; then
  echo "Creando docs/404.html..."
  cp docs/index.html docs/404.html
fi

echo "Build completado. Revisa 'docs' para los archivos estáticos."
