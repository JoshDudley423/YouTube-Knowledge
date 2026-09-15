#!/usr/bin/env python3
"""Show which fetched transcripts haven't been compiled into the knowledge base yet.

A transcript at data/transcripts/<slug>/<id>.txt is considered "compiled"
once a matching note exists at knowledge/<topic>/videos/<id>.md, where
<topic> is that channel's topic per channels.json.

Usage:
  python3 scripts/status.py
"""
from pathlib import Path

from lib import REPO_ROOT, TRANSCRIPTS_DIR, load_channels

KNOWLEDGE_DIR = REPO_ROOT / "knowledge"


def main() -> int:
    channels = load_channels()
    if not channels:
        print("No channels registered yet.")
        return 0

    by_slug = {c["slug"]: c for c in channels}
    any_pending = False

    for slug, channel in by_slug.items():
        transcript_dir = TRANSCRIPTS_DIR / slug
        if not transcript_dir.exists():
            continue
        topic = channel["topic"]
        video_notes_dir = KNOWLEDGE_DIR / topic / "videos"
        pending = []
        for txt_file in sorted(transcript_dir.glob("*.txt")):
            video_id = txt_file.stem
            note_path = video_notes_dir / f"{video_id}.md"
            if not note_path.exists():
                pending.append((video_id, txt_file))

        if pending:
            any_pending = True
            print(f"\n{channel['name']} ({slug}) -> topic '{topic}': {len(pending)} uncompiled")
            for video_id, txt_file in pending:
                size_kb = txt_file.stat().st_size / 1024
                print(f"  {video_id}  ({size_kb:.0f} KB)  {txt_file}")

    if not any_pending:
        print("Everything fetched has been compiled. Nothing pending.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
