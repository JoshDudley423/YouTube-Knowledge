use regex::Regex;
use std::sync::OnceLock;

fn timing_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^\d{2}:\d{2}:\d{2}[.,]\d{3}\s*-->").unwrap())
}

fn cue_num_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"^\d+$").unwrap())
}

fn tag_re() -> &'static Regex {
    static RE: OnceLock<Regex> = OnceLock::new();
    RE.get_or_init(|| Regex::new(r"<[^>]+>").unwrap())
}

/// Cleans WebVTT caption text into deduplicated plain text: strips the
/// header, cue numbers, timing lines and inline tags, unescapes HTML
/// entities, and drops consecutive repeated lines (auto-captions re-emit
/// the same rolling line multiple times).
pub fn vtt_to_text(vtt: &str) -> String {
    let mut lines_out: Vec<String> = Vec::new();
    let mut last_line: Option<String> = None;

    for raw_line in vtt.lines() {
        let line = raw_line.trim();
        if line.is_empty()
            || line == "WEBVTT"
            || line.starts_with("Kind:")
            || line.starts_with("Language:")
            || line.starts_with("NOTE")
            || line.starts_with("STYLE")
        {
            continue;
        }
        if timing_re().is_match(line) || cue_num_re().is_match(line) {
            continue;
        }

        let no_tags = tag_re().replace_all(line, "");
        let decoded = html_escape::decode_html_entities(&no_tags).to_string();
        let cleaned = decoded.trim().to_string();
        if cleaned.is_empty() {
            continue;
        }
        if last_line.as_deref() == Some(cleaned.as_str()) {
            continue;
        }
        lines_out.push(cleaned.clone());
        last_line = Some(cleaned);
    }

    lines_out.join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dedupes_and_strips() {
        let vtt = "WEBVTT\n\n1\n00:00:00.000 --> 00:00:02.000\n<c>Hello there</c>\n\n2\n00:00:02.000 --> 00:00:04.000\nHello there\nfriend &amp; you\n";
        let text = vtt_to_text(vtt);
        assert_eq!(text, "Hello there friend & you");
    }
}
