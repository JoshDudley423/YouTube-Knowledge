#!/usr/bin/env python3
"""Sync tracked channels: discover new videos and download their transcripts.

Usage:
  python3 scripts/sync.py --all
  python3 scripts/sync.py --channel some-slug
  python3 scripts/sync.py --channel some-slug --limit 15   # cap new videos this run

This only does mechanical fetching (list videos, download captions, save
raw transcript text + metadata under data/transcripts/<slug>/). It does not
summarize anything. After running this, tell Claude to compile the new
transcripts into the knowledge base -- see CLAUDE.md.
"""
import argparse
import datetime
import sys

from lib import (
    fetch_video_transcript,
    list_channel_video_ids,
    load_channels,
    load_known_videos,
    polite_sleep,
    save_channels,
    save_known_videos,
    save_transcript,
)


def sync_channel(channel: dict, limit: int | None) -> dict:
    slug = channel["slug"]
    print(f"== {channel['name']} ({slug}) ==")

    known = load_known_videos(slug)
    print(f"  known videos on record: {len(known)}")

    try:
        remote = list_channel_video_ids(channel["url"])
    except Exception as e:
        print(f"  ERROR listing videos: {e}", file=sys.stderr)
        return {"slug": slug, "new": 0, "failed": 0, "error": str(e)}

    print(f"  videos found on channel: {len(remote)}")
    new_videos = [v for v in remote if v["id"] not in known]
    print(f"  new videos to fetch: {len(new_videos)}")

    if limit is not None:
        new_videos = new_videos[:limit]
        print(f"  capped to --limit {limit}: fetching {len(new_videos)}")

    fetched, failed, no_captions = 0, 0, 0
    for i, v in enumerate(new_videos, 1):
        print(f"  [{i}/{len(new_videos)}] {v['id']} - {v.get('title', '')[:70]}")
        try:
            text, metadata = fetch_video_transcript(v["id"])
        except Exception as e:
            print(f"      ERROR: {e}", file=sys.stderr)
            failed += 1
            continue

        if text is None:
            print("      no captions available, skipping")
            no_captions += 1
            known[v["id"]] = {
                "id": v["id"],
                "title": v.get("title"),
                "url": v["url"],
                "upload_date": None,
                "has_transcript": False,
            }
            continue

        save_transcript(slug, v["id"], text, metadata)
        known[v["id"]] = {
            "id": v["id"],
            "title": metadata.get("title") or v.get("title"),
            "url": v["url"],
            "upload_date": metadata.get("upload_date"),
            "duration": metadata.get("duration"),
            "has_transcript": True,
        }
        fetched += 1
        polite_sleep()

    save_known_videos(slug, known)
    print(f"  done: {fetched} transcripts fetched, {no_captions} without captions, {failed} failed")
    return {"slug": slug, "new": fetched, "no_captions": no_captions, "failed": failed}


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    group = ap.add_mutually_exclusive_group(required=True)
    group.add_argument("--all", action="store_true", help="Sync every registered channel")
    group.add_argument("--channel", help="Sync a single channel by slug")
    ap.add_argument("--limit", type=int, default=None, help="Max new videos to fetch this run")
    args = ap.parse_args()

    channels = load_channels()
    if not channels:
        print("No channels registered yet. Run scripts/add_channel.py first.", file=sys.stderr)
        return 1

    if args.channel:
        targets = [c for c in channels if c["slug"] == args.channel]
        if not targets:
            print(f"No channel with slug '{args.channel}'.", file=sys.stderr)
            return 1
    else:
        targets = channels

    results = []
    for channel in targets:
        result = sync_channel(channel, args.limit)
        results.append(result)
        channel["last_synced"] = datetime.datetime.now(datetime.timezone.utc).isoformat()

    save_channels(channels)

    print("\n=== summary ===")
    total_new = sum(r.get("new", 0) for r in results)
    total_failed = sum(r.get("failed", 0) for r in results)
    for r in results:
        print(f"  {r['slug']}: +{r.get('new', 0)} transcripts, {r.get('failed', 0)} failed")
    print(f"total: {total_new} new transcripts, {total_failed} failed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
