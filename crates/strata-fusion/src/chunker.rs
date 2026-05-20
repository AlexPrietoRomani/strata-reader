//! Semantic chunker for Vector-RAG ingestion.
//!
//! Plan Maestro §12.T7.5 — the chunker walks the document's reading
//! order, accumulating blocks into [`Chunk`]s of at most
//! `max_tokens` tokens. Two non-negotiable invariants:
//!
//! 1. **No mid-block splits.** A block is added in full or not at all.
//!    The Markdown serializer can rely on every chunk being a sequence
//!    of complete blocks.
//! 2. **Sliding-window overlap.** Each chunk keeps the last
//!    `overlap` tokens of the previous chunk as a prefix so semantic
//!    context spans chunk boundaries (standard RAG trick to avoid
//!    losing references that straddle a cut).
//!
//! Token counting is whitespace-based — a reasonable proxy for the
//! BPE counts most RAG indexes use. Callers that need exact GPT-tokens
//! can wrap a counter and pass a [`ChunkOptions::token_counter`].

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use strata_core::{BBox, Block, BlockId, BlockType, Document};

use crate::sections::{build_tree, Section, SectionChild};

/// Per-chunk metadata + payload.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chunk {
    /// Concatenated text content (one block per line, separated by `\n\n`).
    pub text: String,
    /// Approximate token count (whitespace-split). Useful for downstream
    /// stratification.
    pub token_count: usize,
    /// 1-indexed page numbers touched by this chunk.
    pub source_pages: Vec<u32>,
    /// Block bounding boxes — the Graph-RAG side uses these to draw the
    /// highlight rectangles in the source PDF preview.
    pub bboxes: Vec<BBox>,
    /// Path of section titles from root down to the deepest containing
    /// section (e.g. ["Introduction", "Related Work"]).
    pub section_path: Vec<String>,
    /// Block ids contributing to this chunk, in reading order.
    pub block_ids: Vec<BlockId>,
}

/// Knobs for [`chunk`].
#[derive(Clone, Debug)]
pub struct ChunkOptions {
    pub max_tokens: usize,
    pub overlap: usize,
    /// Optional custom counter — for callers using a BPE tokenizer.
    pub token_counter: Option<fn(&str) -> usize>,
}

impl Default for ChunkOptions {
    fn default() -> Self {
        Self { max_tokens: 512, overlap: 64, token_counter: None }
    }
}

fn count_tokens(opts: &ChunkOptions, text: &str) -> usize {
    if let Some(counter) = opts.token_counter {
        counter(text)
    } else {
        text.split_whitespace().count()
    }
}

/// Chunk a document. Returns a list of chunks in reading order; the
/// concatenation of their texts preserves the document body up to the
/// overlap duplication.
pub fn chunk(doc: &Document, opts: &ChunkOptions) -> Vec<Chunk> {
    let tree = build_tree(doc);
    let block_page = build_block_page_index(doc);

    let mut chunks: Vec<Chunk> = Vec::new();
    walk_section(&tree, &[], &block_page, opts, &mut chunks);
    chunks
}

fn build_block_page_index(doc: &Document) -> std::collections::HashMap<BlockId, u32> {
    let mut index = std::collections::HashMap::new();
    for page in &doc.pages {
        for b in &page.blocks {
            index.insert(b.id, page.number);
        }
    }
    index
}

fn walk_section(
    section: &Section,
    path: &[String],
    block_page: &std::collections::HashMap<BlockId, u32>,
    opts: &ChunkOptions,
    chunks: &mut Vec<Chunk>,
) {
    let mut current_path: Vec<String> = path.to_vec();
    if section.heading.is_some() {
        current_path.push(section.title().to_string());
    }

    let mut current = ChunkBuilder::new(&current_path);

    for child in &section.children {
        match child {
            SectionChild::Block(b) => {
                add_block_to_chunk(&mut current, b, block_page, opts, chunks);
            }
            SectionChild::Section(sub) => {
                // Flush the accumulated chunk before descending so each
                // section starts on a clean boundary.
                if current.has_content() {
                    chunks.push(current.finalize(opts));
                    current = ChunkBuilder::new(&current_path);
                }
                walk_section(sub, &current_path, block_page, opts, chunks);
            }
        }
    }
    if current.has_content() {
        chunks.push(current.finalize(opts));
    }
}

fn add_block_to_chunk(
    current: &mut ChunkBuilder,
    block: &Arc<Block>,
    block_page: &std::collections::HashMap<BlockId, u32>,
    opts: &ChunkOptions,
    chunks: &mut Vec<Chunk>,
) {
    // Skip strictly-empty blocks (page numbers / headers that the Triage
    // didn't drop but contributed no text).
    if block.content.trim().is_empty() && !matches!(block.kind, BlockType::Figure | BlockType::Table) {
        return;
    }
    let block_tokens = count_tokens(opts, &block.content);

    // If adding this block would push us over the limit AND we already
    // have content, finalize the current chunk first.
    if current.has_content() && current.token_count + block_tokens > opts.max_tokens {
        let overlap_text = current.overlap_tail(opts);
        let old_builder = std::mem::replace(current, ChunkBuilder::new(&current.section_path));
        chunks.push(old_builder.finalize(opts));
        if !overlap_text.is_empty() && opts.overlap > 0 {
            current.push_overlap(&overlap_text, count_tokens(opts, &overlap_text));
        }
    }

    let page = block_page.get(&block.id).copied().unwrap_or(0);
    current.push_block(Arc::clone(block), block_tokens, page);
}

#[derive(Clone)]
struct ChunkBuilder {
    section_path: Vec<String>,
    parts: Vec<String>,
    block_ids: Vec<BlockId>,
    bboxes: Vec<BBox>,
    pages: Vec<u32>,
    token_count: usize,
}

impl ChunkBuilder {
    fn new(section_path: &[String]) -> Self {
        Self {
            section_path: section_path.to_vec(),
            parts: Vec::new(),
            block_ids: Vec::new(),
            bboxes: Vec::new(),
            pages: Vec::new(),
            token_count: 0,
        }
    }

    fn has_content(&self) -> bool {
        !self.parts.is_empty()
    }

    fn push_block(&mut self, block: Arc<Block>, tokens: usize, page: u32) {
        self.parts.push(block.content.clone());
        self.block_ids.push(block.id);
        self.bboxes.push(block.bbox);
        if !self.pages.contains(&page) {
            self.pages.push(page);
        }
        self.token_count += tokens;
    }

    fn push_overlap(&mut self, text: &str, tokens: usize) {
        self.parts.push(text.to_string());
        self.token_count += tokens;
    }

    /// Return the last `overlap` tokens of the current chunk as plain text.
    fn overlap_tail(&self, opts: &ChunkOptions) -> String {
        if opts.overlap == 0 {
            return String::new();
        }
        let joined = self.parts.join("\n\n");
        let words: Vec<&str> = joined.split_whitespace().collect();
        if words.len() <= opts.overlap {
            return joined;
        }
        let start = words.len() - opts.overlap;
        words[start..].join(" ")
    }

    fn finalize(self, _opts: &ChunkOptions) -> Chunk {
        let text = self.parts.join("\n\n");
        Chunk {
            text,
            token_count: self.token_count,
            source_pages: self.pages,
            bboxes: self.bboxes,
            section_path: self.section_path,
            block_ids: self.block_ids,
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use strata_core::{DocMeta, Page, PageOrientation, Provenance, Size};

    fn block(content: &str, kind: BlockType) -> Arc<Block> {
        Arc::new(Block {
            id: BlockId::new(),
            kind,
            bbox: BBox::new(0.0, 0.0, 10.0, 10.0).unwrap(),
            content: content.into(),
            children: vec![],
            provenance: Provenance::rust_native(),
        })
    }

    fn doc_from(blocks: Vec<Arc<Block>>, page_no: u32) -> Document {
        let reading_order = blocks.iter().map(|b| b.id).collect();
        let page = Page {
            number: page_no,
            size: Size::new(595.0, 842.0).unwrap(),
            orientation: PageOrientation::Portrait,
            blocks,
            reading_order,
            media_box: BBox::new(0.0, 0.0, 595.0, 842.0).unwrap(),
        };
        Document {
            meta: DocMeta {
                source_sha256: "0".repeat(64),
                source_filename: "t.pdf".into(),
                schema_version: "0.1.0".into(),
                profile: "balanced".into(),
                extra: Default::default(),
            },
            pages: vec![Arc::new(page)],
        }
    }

    #[test]
    fn empty_document_yields_no_chunks() {
        let doc = doc_from(vec![], 1);
        let chunks = chunk(&doc, &ChunkOptions::default());
        assert!(chunks.is_empty());
    }

    #[test]
    fn single_paragraph_yields_one_chunk() {
        let doc = doc_from(vec![block("Hello world", BlockType::Paragraph)], 1);
        let chunks = chunk(&doc, &ChunkOptions::default());
        assert_eq!(chunks.len(), 1);
        assert_eq!(chunks[0].text, "Hello world");
        assert_eq!(chunks[0].token_count, 2);
        assert_eq!(chunks[0].source_pages, vec![1]);
    }

    #[test]
    fn never_splits_a_block_mid_way() {
        // Each paragraph has 30 tokens; max_tokens = 50; so two paragraphs
        // (60 tokens) should land in two chunks because adding the second
        // would overshoot.
        let words = "word ".repeat(30);
        let p1 = block(&words, BlockType::Paragraph);
        let p2 = block(&words, BlockType::Paragraph);
        let doc = doc_from(vec![p1, p2], 1);

        let chunks = chunk(&doc, &ChunkOptions { max_tokens: 50, overlap: 0, token_counter: None });
        assert_eq!(chunks.len(), 2);
        // No chunk exceeds the budget by more than ONE block's worth.
        for c in &chunks {
            assert!(c.token_count <= 50, "chunk token_count {} > 50", c.token_count);
        }
        // Each chunk carries exactly one block.
        for c in &chunks {
            assert_eq!(c.block_ids.len(), 1);
        }
    }

    #[test]
    fn section_path_propagates_to_chunks() {
        let h1 = block("Introduction", BlockType::Heading { level: 1 });
        let p = block("intro body", BlockType::Paragraph);
        let h2 = block("Related Work", BlockType::Heading { level: 2 });
        let p2 = block("rw body", BlockType::Paragraph);
        let doc = doc_from(vec![h1, p, h2, p2], 1);

        let chunks = chunk(&doc, &ChunkOptions { max_tokens: 100, overlap: 0, token_counter: None });
        assert!(chunks.len() >= 2);
        let last = chunks.last().unwrap();
        // Last chunk should be inside Introduction > Related Work.
        assert_eq!(last.section_path, vec!["Introduction".to_string(), "Related Work".to_string()]);
    }

    #[test]
    fn overlap_repeats_tail_words_in_next_chunk() {
        let p1 = block("one two three four five six seven eight nine ten", BlockType::Paragraph);
        let p2 = block("eleven twelve thirteen", BlockType::Paragraph);
        let p3 = block("fourteen fifteen sixteen", BlockType::Paragraph);
        // Budget = 10 tokens; first chunk = paragraph 1 (10 tokens).
        // Then paragraph 2 (3 tokens) starts a new chunk with the last
        // `overlap = 4` words of paragraph 1 prepended.
        let doc = doc_from(vec![p1, p2, p3], 1);
        let chunks =
            chunk(&doc, &ChunkOptions { max_tokens: 10, overlap: 4, token_counter: None });
        assert!(chunks.len() >= 2);
        let second = &chunks[1];
        // The overlap tail of "one two three four five six seven eight nine ten"
        // is "seven eight nine ten".
        assert!(
            second.text.contains("seven eight nine ten"),
            "overlap tail missing from second chunk: {}",
            second.text
        );
    }

    #[test]
    fn token_count_distribution_centers_on_budget() {
        // 50 paragraphs of 25 tokens each, budget 100: chunks should average
        // 4 blocks ≈ 100 tokens. Just check none overshoots and none is
        // egregiously empty.
        let words = "word ".repeat(25);
        let blocks: Vec<_> = (0..50).map(|_| block(&words, BlockType::Paragraph)).collect();
        let doc = doc_from(blocks, 1);
        let chunks =
            chunk(&doc, &ChunkOptions { max_tokens: 100, overlap: 0, token_counter: None });

        for c in &chunks {
            assert!(c.token_count > 0);
            assert!(c.token_count <= 100, "chunk overflow {} > 100", c.token_count);
        }
        // 50 * 25 = 1250 tokens. At 100 each that's 12-13 chunks ± 1.
        assert!(chunks.len() >= 12 && chunks.len() <= 14, "got {} chunks", chunks.len());
    }
}
