//! Markdown serializer for Vector-RAG ingestion.
//!
//! Plan Maestro §12.T7.3 — renders the [`Document`] as GitHub-Flavoured
//! Markdown:
//!
//! - Headings as `#` / `##` / … based on `BlockType::Heading::level`.
//! - Paragraphs separated by blank lines.
//! - List blocks emit each line as a `- ` item.
//! - Tables: the IA layer already returns GFM-flavoured Markdown in
//!   `Block.content` (see [`crate::IaPayload::Table`]), so we just paste
//!   it through.
//! - Figures: `![alt](href)` where `href` is either a `data:image/png;base64,…`
//!   URL or a path relative to `media_dir`, controlled by [`MarkdownOptions`].
//! - Equations: wrap the LaTeX in `$$ … $$` for display math, `$ … $` for
//!   inline when [`MarkdownOptions::inline_math_threshold`] is set.
//! - References: collected into a numbered list at the document end so
//!   inline `[ref-N]` markers remain stable across renders.
//!
//! The renderer is *pure*: same input ⇒ same output. Snapshot tests in
//! `tests/` will lock in golden outputs once `tests/fixtures` matures.

use std::path::PathBuf;

use strata_core::{Block, BlockType, Document};
use strata_fusion::{build_tree, Section, SectionChild};

/// Knobs for [`render`].
#[derive(Clone, Debug)]
pub struct MarkdownOptions {
    /// Where image figures should source their bytes from.
    pub image_strategy: ImageStrategy,
    /// Equations with token-count ≤ this go inline (`$…$`); larger ones
    /// use display math (`$$…$$`). Set to 0 to always use display.
    pub inline_math_threshold: usize,
    /// When true, append a numbered "References" section at the end.
    pub append_reference_list: bool,
}

#[derive(Clone, Debug)]
pub enum ImageStrategy {
    /// Inline base64 data URLs. Self-contained but heavy.
    DataUrl,
    /// Save images under `dir` and link with relative paths.
    MediaDir { dir: PathBuf },
    /// Skip image embedding entirely; emit just the figure caption.
    CaptionOnly,
}

impl Default for MarkdownOptions {
    fn default() -> Self {
        Self {
            image_strategy: ImageStrategy::CaptionOnly,
            inline_math_threshold: 0,
            append_reference_list: true,
        }
    }
}

/// Render `doc` to GitHub-Flavoured Markdown using `opts`.
pub fn render(doc: &Document, opts: &MarkdownOptions) -> String {
    let tree = build_tree(doc);
    let mut out = String::with_capacity(estimate_size(doc));
    write_section(&mut out, &tree, opts);

    if opts.append_reference_list {
        write_reference_list(&mut out, doc);
    }

    // Collapse runs of more than two blank lines.
    normalize_blank_lines(&out)
}

fn estimate_size(doc: &Document) -> usize {
    doc.pages
        .iter()
        .flat_map(|p| p.blocks.iter())
        .map(|b| b.content.len() + 16)
        .sum()
}

fn write_section(out: &mut String, section: &Section, opts: &MarkdownOptions) {
    if let Some(heading) = &section.heading {
        let level = section.level.clamp(1, 6) as usize;
        out.push_str(&"#".repeat(level));
        out.push(' ');
        out.push_str(heading.content.trim());
        out.push_str("\n\n");
    }
    for child in &section.children {
        match child {
            SectionChild::Block(b) => write_block(out, b, opts),
            SectionChild::Section(sub) => write_section(out, sub, opts),
        }
    }
}

fn write_block(out: &mut String, block: &Block, opts: &MarkdownOptions) {
    match &block.kind {
        BlockType::Heading { level } => {
            // Heading nested inside a Section happens for headings of a
            // level different from the surrounding section. Emit it as a
            // regular heading line.
            let lvl = (*level as usize).clamp(1, 6);
            out.push_str(&"#".repeat(lvl));
            out.push(' ');
            out.push_str(block.content.trim());
            out.push_str("\n\n");
        }
        BlockType::Paragraph | BlockType::Caption => {
            out.push_str(block.content.trim());
            out.push_str("\n\n");
        }
        BlockType::List => {
            for line in block.content.lines() {
                let trimmed = line.trim();
                if trimmed.is_empty() {
                    continue;
                }
                if trimmed.starts_with("- ") || trimmed.starts_with("* ") {
                    out.push_str(trimmed);
                } else {
                    out.push_str("- ");
                    out.push_str(trimmed);
                }
                out.push('\n');
            }
            out.push('\n');
        }
        BlockType::Table => {
            // IA layer returns already-formatted GFM; native tables also
            // carry pre-formatted content. Just paste verbatim.
            out.push_str(block.content.trim_end());
            out.push_str("\n\n");
        }
        BlockType::Equation => {
            let body = block.content.trim();
            let tokens = body.split_whitespace().count();
            if opts.inline_math_threshold > 0 && tokens <= opts.inline_math_threshold {
                out.push('$');
                out.push_str(body);
                out.push_str("$\n\n");
            } else {
                out.push_str("$$\n");
                out.push_str(body);
                out.push_str("\n$$\n\n");
            }
        }
        BlockType::CodeListing => {
            out.push_str("```\n");
            out.push_str(block.content.trim_end());
            out.push_str("\n```\n\n");
        }
        BlockType::Figure => {
            let alt = block.content.trim();
            let alt = if alt.is_empty() { "figure" } else { alt };
            match &opts.image_strategy {
                ImageStrategy::CaptionOnly => {
                    out.push_str("_figure: ");
                    out.push_str(alt);
                    out.push_str("_\n\n");
                }
                ImageStrategy::DataUrl => {
                    // The data URL bytes belong to the page object stream
                    // and are *not* attached to the Block here — the
                    // serializer up-stack will inject them. For now we
                    // emit a placeholder URL the caller can post-process.
                    out.push_str(&format!("![{alt}](data:image/png;base64,...)\n\n"));
                }
                ImageStrategy::MediaDir { dir } => {
                    out.push_str(&format!("![{alt}]({}/{}.png)\n\n", dir.display(), block.id));
                }
            }
        }
        BlockType::Footnote => {
            out.push_str("> ");
            out.push_str(block.content.trim());
            out.push_str("\n\n");
        }
        BlockType::Reference => {
            // References render as part of the trailing list (see
            // `write_reference_list`). When we encounter a reference inline
            // it's a citation marker — emit `[ref-…]` text.
            out.push_str(block.content.trim());
            out.push_str("\n\n");
        }
        BlockType::Header | BlockType::Footer | BlockType::PageNumber => {
            // Repeated headers / footers and page numbers shouldn't pollute
            // the body of a RAG-friendly Markdown. Skip entirely.
        }
    }
}

fn write_reference_list(out: &mut String, doc: &Document) {
    let refs: Vec<&Block> = doc
        .pages
        .iter()
        .flat_map(|p| p.blocks.iter().map(|b| b.as_ref()))
        .filter(|b| matches!(b.kind, BlockType::Reference))
        .collect();
    if refs.is_empty() {
        return;
    }
    out.push_str("## References\n\n");
    for (i, r) in refs.iter().enumerate() {
        out.push_str(&format!("{}. {}\n", i + 1, r.content.trim()));
    }
    out.push('\n');
}

fn normalize_blank_lines(text: &str) -> String {
    // Collapse runs of 3+ newlines into exactly two ("\n\n").
    let mut out = String::with_capacity(text.len());
    let mut blank_run = 0;
    for ch in text.chars() {
        if ch == '\n' {
            blank_run += 1;
            if blank_run <= 2 {
                out.push(ch);
            }
        } else {
            blank_run = 0;
            out.push(ch);
        }
    }
    // Single trailing newline.
    while out.ends_with("\n\n") {
        out.pop();
    }
    if out.is_empty() {
        return out;
    }
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;
    use strata_core::{BBox, BlockId, DocMeta, Page, PageOrientation, Provenance, Size};

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

    fn doc(blocks: Vec<Arc<Block>>) -> Document {
        let reading_order = blocks.iter().map(|b| b.id).collect();
        Document {
            meta: DocMeta {
                source_sha256: "0".repeat(64),
                source_filename: "t.pdf".into(),
                schema_version: "0.1.0".into(),
                profile: "balanced".into(),
                extra: Default::default(),
            },
            pages: vec![Arc::new(Page {
                number: 1,
                size: Size::new(595.0, 842.0).unwrap(),
                orientation: PageOrientation::Portrait,
                blocks,
                reading_order,
                media_box: BBox::new(0.0, 0.0, 595.0, 842.0).unwrap(),
            })],
        }
    }

    #[test]
    fn empty_document_renders_to_empty_string() {
        let md = render(&doc(vec![]), &MarkdownOptions::default());
        assert_eq!(md, "");
    }

    #[test]
    fn heading_renders_with_correct_pound_count() {
        let blocks = vec![
            block("Title", BlockType::Heading { level: 1 }),
            block("Subsection", BlockType::Heading { level: 2 }),
            block("Body", BlockType::Paragraph),
        ];
        let md = render(&doc(blocks), &MarkdownOptions::default());
        assert!(md.starts_with("# Title\n\n"), "got: {md}");
        assert!(md.contains("\n## Subsection\n\n"), "got: {md}");
        assert!(md.contains("\nBody\n"), "got: {md}");
    }

    #[test]
    fn list_block_emits_dash_items() {
        let blocks = vec![block(
            "first item\nsecond item\nthird item",
            BlockType::List,
        )];
        let md = render(&doc(blocks), &MarkdownOptions::default());
        assert!(md.contains("- first item"));
        assert!(md.contains("- second item"));
        assert!(md.contains("- third item"));
    }

    #[test]
    fn equation_uses_display_math_by_default() {
        let blocks = vec![block(r"\int_0^1 x\,dx = \frac{1}{2}", BlockType::Equation)];
        let md = render(&doc(blocks), &MarkdownOptions::default());
        assert!(md.contains("$$"), "expected display math: {md}");
        assert!(md.contains(r"\int_0^1"));
    }

    #[test]
    fn equation_uses_inline_math_when_short_and_threshold_set() {
        let blocks = vec![block("E = mc^2", BlockType::Equation)];
        let md = render(
            &doc(blocks),
            &MarkdownOptions {
                inline_math_threshold: 5,
                ..Default::default()
            },
        );
        assert!(md.contains("$E = mc^2$"), "got: {md}");
    }

    #[test]
    fn header_footer_pagenumber_are_skipped() {
        let blocks = vec![
            block("page 1", BlockType::PageNumber),
            block("My Paper", BlockType::Header),
            block("footer text", BlockType::Footer),
            block("body", BlockType::Paragraph),
        ];
        let md = render(&doc(blocks), &MarkdownOptions::default());
        assert!(!md.contains("page 1"));
        assert!(!md.contains("My Paper"));
        assert!(!md.contains("footer text"));
        assert!(md.contains("body"));
    }

    #[test]
    fn references_render_at_end_when_enabled() {
        let blocks = vec![
            block("body", BlockType::Paragraph),
            block("Smith et al., 2024. *Foo*. JoF.", BlockType::Reference),
            block("Doe & Roe, 2025. *Bar*. JoB.", BlockType::Reference),
        ];
        let md = render(&doc(blocks), &MarkdownOptions::default());
        assert!(md.contains("## References"), "got: {md}");
        assert!(md.contains("1. Smith"));
        assert!(md.contains("2. Doe"));
    }

    #[test]
    fn no_more_than_two_consecutive_blank_lines() {
        // Multiple paragraphs back-to-back should never produce 3+ \n in a row.
        let blocks = vec![
            block("p1", BlockType::Paragraph),
            block("p2", BlockType::Paragraph),
            block("p3", BlockType::Paragraph),
        ];
        let md = render(&doc(blocks), &MarkdownOptions::default());
        assert!(!md.contains("\n\n\n"), "got: {md:?}");
    }

    #[test]
    fn render_is_deterministic() {
        let blocks = vec![
            block("Title", BlockType::Heading { level: 1 }),
            block("body", BlockType::Paragraph),
        ];
        let doc = doc(blocks);
        let a = render(&doc, &MarkdownOptions::default());
        let b = render(&doc, &MarkdownOptions::default());
        assert_eq!(a, b);
    }
}
