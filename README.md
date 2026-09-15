# YouTube Knowledge Base

Point this at YouTube channels on a subject you care about, and it builds a
compiled, cross-channel knowledge base you can query cheaply -- without
Claude re-reading raw video transcripts every time you ask a question.

## How it works

1. **You point Claude at a channel.** Tell Claude (in a Claude Code session
   in this repo) about a channel and what subject it covers.
2. **Transcripts get fetched.** Scripts in `scripts/` (built on
   [yt-dlp](https://github.com/yt-dlp/yt-dlp), no YouTube API key needed)
   list the channel's videos and download their captions. **This step has
   to run from your own machine** -- see [Why fetching runs
   locally](#why-fetching-runs-locally) below.
3. **Claude compiles a knowledge base.** It reads the raw transcripts and
   distills them into organized markdown files under `knowledge/<topic>/`,
   merging information across every channel tracked for that subject and
   noting where channels agree or disagree. This part works fine in a cloud
   Claude Code session.
4. **You ask questions.** Claude answers from the compiled `knowledge/`
   files -- fast and cheap -- and only digs into raw transcripts when it
   needs more detail than the summary has.

See `CLAUDE.md` for the exact workflow Claude follows.

## Quick start (run locally)

```bash
git clone <this repo>
cd YouTube-Knowledge
pip install -r requirements.txt

# Track a channel
python3 scripts/add_channel.py --url "https://www.youtube.com/@SomeChannel" \
    --slug some-channel --topic personal-finance --name "Some Channel"

# Pull transcripts (first run: cap it while backfilling a large channel)
python3 scripts/sync.py --channel some-channel --limit 15

# See what's fetched but not yet folded into the knowledge base
python3 scripts/status.py

# Push what you fetched
git add data channels.json
git commit -m "Sync some-channel"
git push
```

Then, in a Claude Code session (local or cloud) on this repo: "compile the
new transcripts into the knowledge base," or later, "what do these channels
say about X."

## Why fetching runs locally

Claude Code cloud sessions share a datacenter IP pool that YouTube's bot
detection has flagged -- fetch attempts from the cloud get redirected to
Google's CAPTCHA page regardless of how the request is made. Your own
machine's IP doesn't have that problem. So the split is: **fetch locally,
compile and query in Claude Code (cloud or local, your choice)**. You only
need to run the `scripts/sync.py` step yourself, each time you want new
videos pulled in; everything else -- adding channels to the registry,
compiling transcripts into the knowledge base, and answering questions --
Claude handles for you, including from a cloud session, once the transcript
files are pushed.

## Layout

- `channels.json` -- registry of tracked channels and their topic.
- `data/transcripts/<channel>/` -- raw transcript text + metadata per video.
- `knowledge/<topic>/SUMMARY.md` -- the compiled reference for a subject.
- `knowledge/<topic>/videos/<id>.md` -- distilled per-video notes.
- `scripts/` -- the fetch pipeline.

Transcripts are fetched via captions (manual or auto-generated), so a video
needs captions available to be included.
