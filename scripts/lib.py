"""Shared helpers for the YouTube Knowledge Base scripts.

All of this is mechanical (listing videos, downloading captions, cleaning
them up) -- no summarization happens here. Summarization/compilation into
the knowledge/ files is done by Claude reading the raw transcripts produced
here, per the workflow in CLAUDE.md.
"""
from __future__ import annotations

import html
import json
import re
import time
from pathlib import Path

import yt_dlp

REPO_ROOT = Path(__file__).resolve().parent.parent
CHANNELS_FILE = REPO_ROOT / "channels.json"
DATA_DIR = REPO_ROOT / "data"
CHANNELS_DATA_DIR = DATA_DIR / "channels"
TRANSCRIPTS_DIR = DATA_DIR / "transcripts"


def load_channels() -> list[dict]:
    if not CHANNELS_FILE.exists():
        return []
    return json.loads(CHANNELS_FILE.read_text())


def save_channels(channels: list[dict]) -> None:
    CHANNELS_FILE.write_text(json.dumps(channels, indent=2) + "\n")


def channel_videos_file(slug: str) -> Path:
    return CHANNELS_DATA_DIR / slug / "videos.json"


def load_known_videos(slug: str) -> dict[str, dict]:
    path = channel_videos_file(slug)
    if not path.exists():
        return {}
    videos = json.loads(path.read_text())
    return {v["id"]: v for v in videos}


def save_known_videos(slug: str, videos_by_id: dict[str, dict]) -> None:
    path = channel_videos_file(slug)
    path.parent.mkdir(parents=True, exist_ok=True)
    ordered = sorted(
        videos_by_id.values(),
        key=lambda v: v.get("upload_date") or "",
        reverse=True,
    )
    path.write_text(json.dumps(ordered, indent=2) + "\n")


def list_channel_video_ids(channel_url: str) -> list[dict]:
    """Return [{id, title, url}, ...] for every video on a channel's /videos tab.

    Uses flat extraction (no per-video network calls), so it's cheap even
    for channels with hundreds of videos. Order is whatever YouTube returns,
    which is newest-first for the standard /videos tab.
    """
    opts = {
        "quiet": True,
        "no_warnings": True,
        "extract_flat": True,
        "skip_download": True,
    }
    url = channel_url.rstrip("/")
    if not url.endswith(("/videos", "/streams")):
        url = url + "/videos"
    with yt_dlp.YoutubeDL(opts) as ydl:
        info = ydl.extract_info(url, download=False)
    entries = info.get("entries") or []
    results = []
    for e in entries:
        if not e or not e.get("id"):
            continue
        results.append({
            "id": e["id"],
            "title": e.get("title"),
            "url": f"https://www.youtube.com/watch?v={e['id']}",
        })
    return results


VTT_TAG_RE = re.compile(r"<[^>]+>")
VTT_TIMING_LINE_RE = re.compile(r"-->")


def vtt_to_text(vtt: str) -> str:
    """Collapse a WebVTT captions file into deduped, readable plain text.

    Auto-generated YouTube captions are "rolling" -- each cue repeats part of
    the previous line -- so naive concatenation produces heavy duplication.
    This keeps each line only once, in order.
    """
    lines = []
    seen_last = None
    for raw_line in vtt.splitlines():
        line = raw_line.strip()
        if not line:
            continue
        if line.startswith("WEBVTT") or line.startswith("Kind:") or line.startswith("Language:"):
            continue
        if VTT_TIMING_LINE_RE.search(line):
            continue
        if line.isdigit():
            continue
        line = VTT_TAG_RE.sub("", line)
        line = html.unescape(line).strip()
        if not line or line == seen_last:
            continue
        lines.append(line)
        seen_last = line
    # Second pass: drop lines that are pure duplicates of the immediately
    # preceding line after tag-stripping (common with rolling captions).
    deduped = []
    for line in lines:
        if deduped and deduped[-1] == line:
            continue
        deduped.append(line)
    return "\n".join(deduped)


def fetch_video_transcript(video_id: str, lang: str = "en") -> tuple[str | None, dict]:
    """Download captions + metadata for one video.

    Returns (transcript_text_or_None, metadata_dict). transcript is None if
    no captions (manual or auto) are available in the requested language.
    """
    import tempfile

    with tempfile.TemporaryDirectory() as tmp:
        outtmpl = str(Path(tmp) / "%(id)s.%(ext)s")
        opts = {
            "quiet": True,
            "no_warnings": True,
            "noprogress": True,
            "skip_download": True,
            "writesubtitles": True,
            "writeautomaticsub": True,
            "subtitleslangs": [lang, f"{lang}-orig"],
            "subtitlesformat": "vtt",
            "outtmpl": outtmpl,
            # The default 'web' client triggers YouTube's "sign in to
            # confirm you're not a bot" check from datacenter IPs. These
            # clients serve player responses (and thus caption URLs)
            # without needing cookies or a PO token.
            "extractor_args": {"youtube": {"player_client": ["android", "tv", "web_safari"]}},
        }
        with yt_dlp.YoutubeDL(opts) as ydl:
            info = ydl.extract_info(
                f"https://www.youtube.com/watch?v={video_id}", download=True
            )

        metadata = {
            "id": info.get("id"),
            "title": info.get("title"),
            "channel": info.get("channel") or info.get("uploader"),
            "upload_date": info.get("upload_date"),
            "duration": info.get("duration"),
            "url": info.get("webpage_url"),
            "description": (info.get("description") or "")[:2000],
        }

        vtt_files = sorted(Path(tmp).glob(f"{video_id}*.vtt"))
        if not vtt_files:
            return None, metadata
        vtt_text = vtt_files[0].read_text(errors="ignore")
        return vtt_to_text(vtt_text), metadata


def save_transcript(slug: str, video_id: str, text: str, metadata: dict) -> Path:
    out_dir = TRANSCRIPTS_DIR / slug
    out_dir.mkdir(parents=True, exist_ok=True)
    txt_path = out_dir / f"{video_id}.txt"
    meta_path = out_dir / f"{video_id}.json"
    txt_path.write_text(text)
    meta_path.write_text(json.dumps(metadata, indent=2) + "\n")
    return txt_path


def polite_sleep(seconds: float = 1.5) -> None:
    time.sleep(seconds)
