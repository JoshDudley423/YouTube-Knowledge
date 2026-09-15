# YouTube Knowledge Base

This repo turns a set of YouTube channels into a compiled, cheap-to-query
knowledge base per subject. It has two kinds of work, and they must stay
separate:

- **Mechanical fetching** (scripts/): listing a channel's videos and
  downloading their transcripts. No LLM involved. Deterministic, re-runnable.
- **Compilation** (you, Claude): reading raw transcripts and distilling them
  into the `knowledge/` markdown files. This is the only place understanding
  happens, and it's what makes future questions cheap to answer.

Never let a future session answer a subject-matter question by re-reading
raw transcripts in `data/transcripts/` from scratch. That defeats the whole
point. Raw transcripts are the source data used *once* to build/update
`knowledge/`; after that, `knowledge/` is what gets read.

## Layout

```
channels.json                        registry of tracked channels
data/channels/<slug>/videos.json     video list + fetch status per channel (mechanical)
data/transcripts/<slug>/<id>.txt     cleaned raw transcript text (mechanical)
data/transcripts/<slug>/<id>.json    video metadata: title, url, upload_date, duration (mechanical)
knowledge/<topic>/SUMMARY.md         the compiled, cross-channel knowledge base for a subject -- READ THIS FIRST
knowledge/<topic>/videos/<id>.md     per-video distilled notes (intermediate layer, used to build/update SUMMARY.md)
scripts/                             fetch pipeline (Python + yt-dlp, no API key)
```

`topic` is a slug grouping one or more channels that cover the same subject
(e.g. `personal-finance`, `woodworking`, `home-networking`). Multiple
channels can share a topic; that's the point -- `SUMMARY.md` is where their
information gets merged into one reference.

## Workflow

### 1. Adding a channel

When the user points you at a channel:

```
python3 scripts/add_channel.py --url "<channel url>" --slug <short-id> \
    --topic <topic-slug> --name "<Channel Name>"
```

Pick `--topic` to match an existing topic if this channel covers the same
subject as one already tracked (check `channels.json`), otherwise start a
new topic slug. Ask the user only if it's genuinely ambiguous which topic a
channel belongs to.

### 2. Syncing (fetch new transcripts)

**Syncing must run from the user's own machine, not from a Claude Code cloud
session.** Cloud sessions share a datacenter egress IP that YouTube's bot
detection has flagged -- confirmed by a request getting redirected straight
to Google's `/sorry/` CAPTCHA page, independent of yt-dlp client/retry
tuning. There is no fix for this from inside a cloud session. If you're
running in one and the user asks to sync, don't attempt `scripts/sync.py`
yourself expecting it to work -- tell them to run it locally (commands
below) and push, or run it locally on their behalf if you're already
operating as their local CLI.

```
python3 scripts/sync.py --channel <slug>        # one channel
python3 scripts/sync.py --all                    # everything
python3 scripts/sync.py --channel <slug> --limit 15   # cap for a first backfill
```

This lists all videos on the channel, skips ones already fetched (tracked in
`data/channels/<slug>/videos.json`), and downloads captions (manual first,
falling back to auto-generated) for the rest, saving cleaned text +
metadata under `data/transcripts/<slug>/`.

For a channel with a very large back catalog, use `--limit` on the first
sync to backfill incrementally rather than pulling hundreds of transcripts
at once. Re-run without `--limit` (or with a higher one) to keep going.

If a run reports videos with no captions available, that's normal (some
videos have captions disabled) -- they're marked `has_transcript: false` and
skipped on future syncs.

After syncing locally: `git add data channels.json && git commit && git
push`. A cloud session picks up the new transcripts on its next `git pull`
and handles compiling them into `knowledge/` from there (step 3) -- that
part works fine in the cloud, only the fetch itself needs to run locally.

### 3. Compiling into the knowledge base

After a sync, check what's uncompiled:

```
python3 scripts/status.py
```

For each pending video, this is *your* job, not a script's:

1. Read the raw transcript (`data/transcripts/<slug>/<id>.txt`) and its
   metadata (`data/transcripts/<slug>/<id>.json`).
2. Write a distilled note to `knowledge/<topic>/videos/<id>.md`: the
   concrete claims, techniques, numbers, recommendations, and caveats the
   video makes -- not a summary of *that a video exists*, but the actual
   substance someone would want to recall later. Keep it dense, skip filler.
   Include the source channel, video title, and URL at the top.
3. Update `knowledge/<topic>/SUMMARY.md`: merge the new video's claims into
   the compiled subject reference. Organize by sub-topic/question, not by
   video. When channels agree, state it once. When they disagree or give
   conflicting numbers/advice, say so explicitly and name which
   channel/video said what. Every non-obvious claim should be traceable to
   a `[Channel Name: Video Title](video-note-or-url)` citation.

**Batch this work with a subagent** when there are more than a handful of
pending videos, so raw transcript text doesn't fill up your own context:
hand the subagent the list of pending `(slug, video_id, topic)` tuples, the
existing `knowledge/<topic>/SUMMARY.md` (if any) to build on, and the exact
output contract above (per-video note + updated SUMMARY.md). Do this per
topic so the subagent only needs that topic's existing summary as context.

If `knowledge/<topic>/SUMMARY.md` doesn't exist yet, create it with a short
header (topic name, channels covered, last-compiled date) followed by
organized sections.

### 4. Answering questions

When the user asks something about a subject:

1. Read `knowledge/<topic>/SUMMARY.md` for the relevant topic(s) first.
   This should answer most questions directly and cheaply.
2. Only if the summary lacks the needed detail, open the specific
   `knowledge/<topic>/videos/<id>.md` note(s) it cites.
3. Only as a last resort -- e.g. the user wants an exact quote or the
   summary flags something as needing more nuance -- read the raw
   transcript in `data/transcripts/`.
4. Cite which channel(s)/video(s) the answer is drawn from, especially when
   sources disagree.

If the knowledge base doesn't cover the question at all, say so, and offer
to add a relevant channel or note that no tracked channel covers it.

### Keeping things current

When the user says something like "update my knowledge base" or "check for
new videos," run `scripts/sync.py --all`, then compile any pending videos
per step 3, without needing to be asked for each sub-step.
