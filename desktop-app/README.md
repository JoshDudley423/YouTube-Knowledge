# YouTube Knowledge -- Desktop App

A standalone desktop app version of this repo's YouTube-scrounging idea:
paste a channel URL, it fetches transcripts, and you chat with a **fully
local** model over that channel's own transcripts. No cloud AI, no API
key, no account -- everything runs on your machine.

## Architecture

- **Shell**: [Tauri](https://tauri.app) (Rust backend + native OS webview).
  One codebase builds for Windows, macOS and Linux.
- **Fetching**: a bundled [yt-dlp](https://github.com/yt-dlp/yt-dlp) binary,
  run as a Tauri "sidecar" process -- no Python install required on the
  user's machine. Lists a channel's `/videos` tab (never `/shorts`, so
  Shorts are excluded by construction) and downloads captions per video.
- **Knowledge**: unlike the Claude-Code-cloud workflow in the main repo
  (which does a deep multi-pass LLM compilation into `SUMMARY.md`), this
  app does **retrieval-augmented generation (RAG) at query time**: transcripts
  are chunked and indexed with an in-memory BM25 search (`src-tauri/src/index.rs`),
  no cloud embeddings API involved. On each question, the top few matching
  chunks are pulled and fed to the local model as context. This was a
  deliberate simplification -- a small local model isn't strong enough to
  reproduce Claude's cross-video synthesis quality, and RAG-on-demand is far
  more robust to build and maintain than porting the multi-phase compile
  pipeline to run on-device.
- **Local model**: [llamafile](https://github.com/mozilla-ai/llamafile) --
  a single portable executable that bundles a quantized model with
  llama.cpp's OpenAI-compatible HTTP server. The app does **not** bundle or
  auto-download a specific model file -- a hardcoded download URL is a
  single point of failure (a model gets renamed, moved, or gated and the
  app breaks for everyone with no way to fix it except a code update). It
  picks up any `.llamafile` file you drop into its `models/` data directory
  instead, and manages it as a background process (`src-tauri/src/llm.rs`).
  Chat is real generation grounded in retrieved transcript passages, not a
  plain keyword search box.

Everything -- transcripts, the search index, and the model -- lives in the
OS's per-user app data directory, not in this repo.

## Project layout

```
src/                     frontend (vanilla HTML/CSS/JS, no framework)
src-tauri/src/
  ytdlp.rs               sidecar-based channel listing + caption fetch
  transcript.rs          WebVTT -> deduplicated plain text
  index.rs               chunking + in-memory BM25 search
  llm.rs                 llamafile process management + streaming chat
  commands.rs            Tauri commands the frontend calls
  state.rs               app state, channel registry persistence
scripts/fetch-sidecars.sh / .ps1   downloads yt-dlp binaries into src-tauri/binaries/
```

## Building it yourself

This was scaffolded and the Rust backend written/compiled inside a **Linux**
cloud sandbox (`cargo check` and `cargo test --lib` both pass here). Two
things can only be done on your own machine, for the same reason the main
repo's `scripts/sync.py` has to run locally:

1. **Windows and macOS builds / GUI testing.** The sandbox this was built
   in is Linux-only and has no way to produce or test a Windows/macOS
   binary or open an actual window.
2. **A real end-to-end fetch test.** This sandbox's cloud egress IP is
   flagged by YouTube's bot detection (confirmed while building this: even
   with the same `android,tv,web_safari` player-client workaround used
   elsewhere in this repo, caption downloads here get met with "Sign in to
   confirm you're not a bot"). Channel *listing* works fine from here
   (verified against a real channel); it's specifically the per-video
   caption fetch that needs a non-flagged residential IP. This isn't a bug
   to fix in the code -- it'll work normally from your own machine, exactly
   like the cloud-vs-local split documented in the repo root `CLAUDE.md`.

### Setup

```
cd desktop-app
npm install

# fetch yt-dlp binaries for all three platforms (safe to run from any OS --
# these are just downloads of prebuilt static binaries, not local builds)
npm run fetch-sidecars        # macOS/Linux
# or on Windows:
powershell -ExecutionPolicy Bypass -File scripts/fetch-sidecars.ps1

npm run dev                    # opens the app with hot reload
npm run build                  # produces an installer/bundle for your OS
```

`npm run build` only produces a bundle for the OS you run it on (Tauri
doesn't cross-compile GUI apps). To ship both, run `npm run build` once on
a Windows machine and once on a macOS machine.

### First run

On first launch the app has no channels and no local model.

1. Use the sidebar to paste a channel URL, a display name, and a topic,
   which kicks off a sync automatically.
2. Separately, get yourself a `.llamafile` model: click "Open models
   folder" in the sidebar's model panel (this opens the app's `models/`
   data directory), then in your own browser download any `.llamafile`
   you like -- for example search GitHub's `mozilla-ai/llamafile` releases,
   or Hugging Face, for a few-GB "Instruct" model -- and save it straight
   into that folder. **On Windows**, rename the downloaded file so it ends
   in `.llamafile.exe` (Windows won't run a file without a recognized
   executable extension). Click "Refresh" in the app afterward.
3. After that, chat works fully offline using whatever model you dropped
   in -- no bundled or auto-downloaded default model ships with the app, so
   there's no fixed filename to match and no single download link that can
   break for everyone at once.

### Known rough edges / next steps

- The shell-sidecar permission block in
  `src-tauri/capabilities/default.json` was written against the
  tauri-plugin-shell v2 permission format from memory -- if `npm run dev`
  errors on a permission check when invoking yt-dlp, check the
  [tauri-plugin-shell docs](https://v2.tauri.app/plugin/shell/) for the
  current scope syntax and adjust that file.
- There's no re-sync scheduling or background polling yet -- syncing is
  manual (click "Sync" on a channel).
- No channel/topic editing after creation; remove and re-add if you get a
  topic wrong.
- Model quality/speed is entirely up to which `.llamafile` you pick: a
  small (~3B parameter) model is fast but noticeably weaker than the cloud
  Claude Code workflow in the rest of this repo; a larger model answers
  better but is much slower on an ordinary laptop CPU with no GPU. That
  tradeoff -- "no cloud AI, runs on your laptop" -- is fundamental to this
  app's design, not something a different model choice fully escapes.
