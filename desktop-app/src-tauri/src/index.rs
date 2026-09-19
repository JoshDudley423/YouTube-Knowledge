//! A small in-memory BM25 search index over transcript chunks.
//!
//! There's no cloud embeddings API available offline, and the corpus size
//! here (a few hundred videos per channel, a few thousand chunks) is tiny
//! enough that a hand-rolled BM25 over the whole thing in memory is both
//! simpler and fast enough -- no vector DB, no external service.

use std::collections::HashMap;

const CHUNK_WORDS: usize = 180;
const CHUNK_OVERLAP: usize = 40;
const K1: f64 = 1.5;
const B: f64 = 0.75;

#[derive(Debug, Clone)]
pub struct Chunk {
    pub slug: String,
    pub video_id: String,
    pub title: String,
    pub url: String,
    pub text: String,
}

pub struct SearchIndex {
    chunks: Vec<Chunk>,
    chunk_tokens: Vec<Vec<String>>,
    doc_freq: HashMap<String, usize>,
    doc_len: Vec<usize>,
    avg_len: f64,
}

fn tokenize(text: &str) -> Vec<String> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|s| s.len() > 1)
        .map(|s| s.to_lowercase())
        .collect()
}

/// Splits a video's transcript into overlapping word-window chunks so a
/// query can retrieve the specific passage that answers it, not the whole
/// (possibly hour-long) transcript.
pub fn chunk_transcript(slug: &str, video_id: &str, title: &str, url: &str, text: &str) -> Vec<Chunk> {
    let words: Vec<&str> = text.split_whitespace().collect();
    if words.is_empty() {
        return Vec::new();
    }
    let mut chunks = Vec::new();
    let step = CHUNK_WORDS.saturating_sub(CHUNK_OVERLAP).max(1);
    let mut start = 0;
    while start < words.len() {
        let end = (start + CHUNK_WORDS).min(words.len());
        let chunk_text = words[start..end].join(" ");
        chunks.push(Chunk {
            slug: slug.to_string(),
            video_id: video_id.to_string(),
            title: title.to_string(),
            url: url.to_string(),
            text: chunk_text,
        });
        if end == words.len() {
            break;
        }
        start += step;
    }
    chunks
}

impl SearchIndex {
    pub fn build(chunks: Vec<Chunk>) -> Self {
        let chunk_tokens: Vec<Vec<String>> = chunks.iter().map(|c| tokenize(&c.text)).collect();
        let doc_len: Vec<usize> = chunk_tokens.iter().map(|t| t.len()).collect();
        let avg_len = if doc_len.is_empty() {
            0.0
        } else {
            doc_len.iter().sum::<usize>() as f64 / doc_len.len() as f64
        };

        let mut doc_freq: HashMap<String, usize> = HashMap::new();
        for tokens in &chunk_tokens {
            let mut seen = std::collections::HashSet::new();
            for t in tokens {
                if seen.insert(t.clone()) {
                    *doc_freq.entry(t.clone()).or_insert(0) += 1;
                }
            }
        }

        SearchIndex {
            chunks,
            chunk_tokens,
            doc_freq,
            doc_len,
            avg_len,
        }
    }

    pub fn is_empty(&self) -> bool {
        self.chunks.is_empty()
    }

    fn idf(&self, term: &str) -> f64 {
        let n = self.chunks.len() as f64;
        let n_t = *self.doc_freq.get(term).unwrap_or(&0) as f64;
        ((n - n_t + 0.5) / (n_t + 0.5) + 1.0).ln()
    }

    /// Returns the top `top_k` chunks for `query`, optionally restricted to
    /// a single channel slug.
    pub fn search(&self, query: &str, top_k: usize, slug_filter: Option<&str>) -> Vec<(&Chunk, f64)> {
        let query_terms = tokenize(query);
        if query_terms.is_empty() || self.is_empty() {
            return Vec::new();
        }

        let mut scores: Vec<(usize, f64)> = Vec::with_capacity(self.chunks.len());
        for (i, chunk) in self.chunks.iter().enumerate() {
            if let Some(slug) = slug_filter {
                if chunk.slug != slug {
                    continue;
                }
            }
            let tokens = &self.chunk_tokens[i];
            let len = self.doc_len[i] as f64;
            let mut score = 0.0;
            for term in &query_terms {
                let f = tokens.iter().filter(|t| *t == term).count() as f64;
                if f == 0.0 {
                    continue;
                }
                let idf = self.idf(term);
                let denom = f + K1 * (1.0 - B + B * len / self.avg_len.max(1.0));
                score += idf * (f * (K1 + 1.0)) / denom;
            }
            if score > 0.0 {
                scores.push((i, score));
            }
        }

        scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        scores
            .into_iter()
            .take(top_k)
            .map(|(i, s)| (&self.chunks[i], s))
            .collect()
    }
}
