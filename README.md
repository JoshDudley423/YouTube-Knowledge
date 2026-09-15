# YouTube Knowledge Base

Point this at YouTube channels on a subject you care about, and it builds a
compiled, cross-channel knowledge base you can query cheaply -- without
Claude re-reading raw video transcripts every time you ask a question.

## How it works

1. **You point Claude at a channel.** Tell Claude (in a Claude Code session
   in this repo) about a channel and what subject it covers.
2. **Claude fetches transcripts.** It runs the scripts in `scripts/` (built
   on [yt-dlp](https://github.com/yt-dlp/yt-dlp), no YouTube API key needed)
   to list the channel's videos and download their captions.
3. **Claude compiles a knowledge base.** It reads the raw transcripts and
   distills them into organized markdown files under `knowledge/<topic>/`,
   merging information across every channel tracked for that subject and
   noting where channels agree or disagree.
4. **You ask questions.** Claude answers from the compiled `knowledge/`
   files -- fast and cheap -- and only digs into raw transcripts when it
   needs more detail than the summary has.

See `CLAUDE.md` for the exact workflow Claude follows.

## Quick start

```bash
pip install -r requirements.txt

# Track a channel
python3 scripts/add_channel.py --url "https://www.youtube.com/@SomeChannel" \
    --slug some-channel --topic personal-finance --name "Some Channel"

# Pull transcripts (first run: cap it while backfilling a large channel)
python3 scripts/sync.py --channel some-channel --limit 15

# See what's fetched but not yet folded into the knowledge base
python3 scripts/status.py
```

From there, just talk to Claude in this repo: "add this channel," "sync
everything and update the knowledge base," "what do these channels say
about X."

## Layout

- `channels.json` -- registry of tracked channels and their topic.
- `data/transcripts/<channel>/` -- raw transcript text + metadata per video.
- `knowledge/<topic>/SUMMARY.md` -- the compiled reference for a subject.
- `knowledge/<topic>/videos/<id>.md` -- distilled per-video notes.
- `scripts/` -- the fetch pipeline.

Transcripts are fetched via captions (manual or auto-generated), so a video
needs captions available to be included.
