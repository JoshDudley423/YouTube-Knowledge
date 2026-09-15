#!/usr/bin/env python3
"""Register a new YouTube channel to track.

Usage:
  python3 scripts/add_channel.py --url "https://www.youtube.com/@SomeChannel" \\
      --slug some-channel --topic "topic-slug" --name "Some Channel"

`slug` and `topic` are filesystem/markdown-friendly identifiers (lowercase,
hyphens). `topic` groups this channel with others into the same compiled
knowledge base file at knowledge/<topic>/SUMMARY.md.
"""
import argparse
import sys

from lib import load_channels, save_channels


def main() -> int:
    ap = argparse.ArgumentParser(description=__doc__)
    ap.add_argument("--url", required=True, help="Channel URL, e.g. https://www.youtube.com/@handle")
    ap.add_argument("--slug", required=True, help="Short unique id, e.g. 'some-channel'")
    ap.add_argument("--topic", required=True, help="Topic grouping, e.g. 'personal-finance'")
    ap.add_argument("--name", required=True, help="Human-readable channel name")
    args = ap.parse_args()

    channels = load_channels()
    if any(c["slug"] == args.slug for c in channels):
        print(f"Channel slug '{args.slug}' already registered.", file=sys.stderr)
        return 1

    channels.append({
        "slug": args.slug,
        "name": args.name,
        "url": args.url,
        "topic": args.topic,
        "added": __import__("datetime").date.today().isoformat(),
        "last_synced": None,
    })
    save_channels(channels)
    print(f"Added channel '{args.name}' ({args.slug}) under topic '{args.topic}'.")
    print(f"Next: python3 scripts/sync.py --channel {args.slug}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
