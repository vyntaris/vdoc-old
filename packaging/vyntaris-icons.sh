#!/usr/bin/env bash
# Renders the app icon into assets/app-icon/hicolor (the icon theme layout the package installs):
# vdoc.svg from 48 px up and as the scalable icon, vdoc-small.svg (no badge) below. Needs rsvg-convert.
set -euo pipefail
cd "$(dirname "$0")/../assets/app-icon"
id=org.vyntaris.VDoc
for size in 16 22 24 32 48 64 128 256 512; do
    src=vdoc.svg
    [ "$size" -lt 48 ] && src=vdoc-small.svg
    mkdir -p "hicolor/${size}x${size}/apps"
    rsvg-convert -w "$size" -h "$size" "$src" -o "hicolor/${size}x${size}/apps/$id.png"
done
mkdir -p hicolor/scalable/apps
cp vdoc.svg "hicolor/scalable/apps/$id.svg"
echo "icons written to $(pwd)/hicolor"
