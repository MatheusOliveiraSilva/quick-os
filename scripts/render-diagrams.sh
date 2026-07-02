#!/usr/bin/env bash
# Render docs/mermaid/*.mmd → docs/images/*.png (for GitHub mobile viewing)
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

mkdir -p docs/images

for f in docs/mermaid/*.mmd; do
  base="$(basename "$f" .mmd)"
  echo "→ docs/images/${base}.png"
  npx --yes @mermaid-js/mermaid-cli \
    -i "$f" \
    -o "docs/images/${base}.png" \
    -b transparent \
    -w 1536 \
    -H 1024
done

echo "Done. Link format:"
echo "  https://raw.githubusercontent.com/MatheusOliveiraSilva/quick-os/main/docs/images/<name>.png"
