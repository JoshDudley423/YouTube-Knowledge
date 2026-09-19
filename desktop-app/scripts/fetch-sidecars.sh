#!/usr/bin/env bash
# Downloads yt-dlp's static release binaries and places them where Tauri's
# sidecar mechanism expects them (name suffixed with the Rust target
# triple). Run this once before `npm run tauri dev` / `npm run tauri build`.
set -euo pipefail
cd "$(dirname "$0")/../src-tauri"
mkdir -p binaries
cd binaries

base="https://github.com/yt-dlp/yt-dlp/releases/latest/download"
curl -fL -o yt-dlp-x86_64-unknown-linux-gnu "$base/yt-dlp_linux"
curl -fL -o yt-dlp-x86_64-pc-windows-msvc.exe "$base/yt-dlp.exe"
curl -fL -o yt-dlp-x86_64-apple-darwin "$base/yt-dlp_macos"
cp yt-dlp-x86_64-apple-darwin yt-dlp-aarch64-apple-darwin

chmod +x yt-dlp-x86_64-unknown-linux-gnu yt-dlp-x86_64-apple-darwin yt-dlp-aarch64-apple-darwin

echo "yt-dlp sidecars ready in src-tauri/binaries/"
