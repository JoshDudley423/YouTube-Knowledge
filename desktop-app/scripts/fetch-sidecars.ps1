# Downloads yt-dlp's static release binaries and places them where Tauri's
# sidecar mechanism expects them (name suffixed with the Rust target
# triple). Run this once before `npm run tauri dev` / `npm run tauri build`.
$ErrorActionPreference = "Stop"
$dest = Join-Path $PSScriptRoot "..\src-tauri\binaries"
New-Item -ItemType Directory -Force -Path $dest | Out-Null

$base = "https://github.com/yt-dlp/yt-dlp/releases/latest/download"
Invoke-WebRequest "$base/yt-dlp.exe" -OutFile (Join-Path $dest "yt-dlp-x86_64-pc-windows-msvc.exe")
Invoke-WebRequest "$base/yt-dlp_macos" -OutFile (Join-Path $dest "yt-dlp-x86_64-apple-darwin")
Copy-Item (Join-Path $dest "yt-dlp-x86_64-apple-darwin") (Join-Path $dest "yt-dlp-aarch64-apple-darwin")
Invoke-WebRequest "$base/yt-dlp_linux" -OutFile (Join-Path $dest "yt-dlp-x86_64-unknown-linux-gnu")

Write-Host "yt-dlp sidecars ready in src-tauri/binaries/"
