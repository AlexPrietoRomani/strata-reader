//! Build a hierarchical [`Section`] tree from the linear reading order.
//!
//! Plan Maestro §12.T7.2 — every Section can contain either nested
//! [`Section`]s or leaf [`Block`]s. Sections nest based on heading levels
//! (1 = top, 6 = deepest). Blocks that appear before the first heading
//! attach to the synthetic root.
//!
//! The output is the input to the Markdown serializer (T7.3) and the
//! Graph-RAG JSON renderer (T7.4).

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use strata_core::{Block, BlockType, Document};

/// A child of a [`Section`] — either another section (recursive) or a
/// leaf block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case")]
pub enum SectionChild {
    Section(Section),
    Block(Arc<Block>),
}

/// One hierarchical region of the document.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Section {
    /// `None` only for the synthetic root. Inner sections always carry
    /// the block whose heading opens them.
    pub heading: Option<Arc<Block>>,
    /// Heading depth (1-6). `0` for the synthetic root.
    pub level: u8,
    pub children: Vec<SectionChild>,
}

impl Section {
    /// Convenience: textual title of the section. Empty string for the
    /// root and headings without text.
    pub fn title(&self) -> &str {
        self.heading
            .as_deref()
            .map(|b| b.content.as_str())
            .unwrap_or("")
    }

    /// Depth-first iterator over every leaf block in the subtree, in
    /// reading order.
    pub fn iter_blocks(&self) -> impl Iterator<Item = Arc<Block>> + '_ {
        let mut stack: Vec<SectionChild> = self.children.iter().rev().cloned().collect();
        std::iter::from_fn(move || {
            while let Some(child) = stack.pop() {
                match child {
                    SectionChild::Block(b) => return Some(b),
                    SectionChild::Section(sec) => {
                        for c in sec.children.iter().rev().cloned() {
                            stack.push(c);
                        }
                    }
                }
            }
            None
        })
    }

    /// Compute the maximum depth of nested headings rooted at this section.
    /// A root with a single H1 child returns 1; nested H1 → H2 → H3 → 3.
    pub fn depth(&self) -> u8 {
        let mut max_child = 0;
        for c in &self.children {
            if let SectionChild::Section(s) = c {
                let d = s.depth();
                if d > max_child {
                    max_child = d;
                }
            }
        }
        // If the section itself has a heading, count it as depth 1.
        let self_depth: u8 = if self.heading.is_some() { 1 } else { 0 };
        self_depth.saturating_add(max_child)
    }
}

/// Build a tree from `doc`'s reading order. Walks every page in order
/// and threads block ids through the reading_order lookup.
pub fn build_tree(doc: &Document) -> Section {
    let mut root = Section {
        heading: None,
        level: 0,
        children: Vec::new(),
    };
    let mut stack: Vec<Section> = Vec::new();

    for page in &doc.pages {
        // Resolve ids → arcs once per page.
        let id_to_arc: std::collections::HashMap<_, _> =
            page.blocks.iter().map(|b| (b.id, Arc::clone(b))).collect();

        for id in &page.reading_order {
            let Some(block) = id_to_arc.get(id) else {
                continue;
            };
            match block.kind {
                BlockType::Heading { level } => {
                    push_heading(&mut root, &mut stack, Arc::clone(block), level);
                }
                _ => push_block(&mut root, &mut stack, Arc::clone(block)),
            }
        }
    }

    // Close any open sections back into the root.
    while let Some(top) = stack.pop() {
        attach_section(&mut root, &mut stack, top);
    }
    root
}

fn push_heading(root: &mut Section, stack: &mut Vec<Section>, block: Arc<Block>, level: u8) {
    // Close every open section whose depth is >= the new heading level.
    while stack.last().is_some_and(|s| s.level >= level) {
        if let Some(closed) = stack.pop() {
            attach_section(root, stack, closed);
        }
    }
    // Open a new section anchored on this heading.
    stack.push(Section {
        heading: Some(block),
        level,
        children: Vec::new(),
    });
}

fn push_block(root: &mut Section, stack: &mut [Section], block: Arc<Block>) {
    if let Some(top) = stack.last_mut() {
        top.children.push(SectionChild::Block(block));
    } else {
        root.children.push(SectionChild::Block(block));
    }
}

fn attach_section(root: &mut Section, stack: &mut [Section], closed: Section) {
    if let Some(parent) = stack.last_mut() {
        parent.children.push(SectionChild::Section(closed));
    } else {
        root.children.push(SectionChild::Section(closed));
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use strata_core::{BBox, BlockId, DocMeta, Page, PageOrientation, Provenance, Size};

    fn block(content: &str, kind: BlockType) -> Arc<Block> {
        Arc::new(Block {
            id: BlockId::new(),
            kind,
            bbox: BBox::new(0.0, 0.0, 10.0, 10.0).unwrap(),
            content: content.into(),
            children: vec![],
            metadata: None,
            provenance: Provenance::rust_native(),
        })
    }

    fn doc_from(blocks: Vec<Arc<Block>>) -> Document {
        let reading_order = blocks.iter().map(|b| b.id).collect();
        let page = Page {
            number: 1,
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
    fn empty_document_yields_empty_root() {
        let tree = build_tree(&doc_from(vec![]));
        assert!(tree.heading.is_none());
        assert_eq!(tree.level, 0);
        assert!(tree.children.is_empty());
    }

    #[test]
    fn blocks_before_first_heading_attach_to_root() {
        let p = block("preamble", BlockType::Paragraph);
        let h = block("Section 1", BlockType::Heading { level: 1 });
        let body = block("body", BlockType::Paragraph);
        let tree = build_tree(&doc_from(vec![p, h, body]));

        assert_eq!(tree.children.len(), 2);
        assert!(matches!(tree.children[0], SectionChild::Block(_)));
        let sec = match &tree.children[1] {
            SectionChild::Section(s) => s,
            _ => panic!("expected section"),
        };
        assert_eq!(sec.title(), "Section 1");
        assert_eq!(sec.children.len(), 1);
    }

    #[test]
    fn nested_headings_produce_nested_sections() {
        let h1 = block("H1", BlockType::Heading { level: 1 });
        let p1 = block("p under h1", BlockType::Paragraph);
        let h2 = block("H2", BlockType::Heading { level: 2 });
        let p2 = block("p under h2", BlockType::Paragraph);
        let h3 = block("H3", BlockType::Heading { level: 3 });
        let p3 = block("p under h3", BlockType::Paragraph);
        let tree = build_tree(&doc_from(vec![h1, p1, h2, p2, h3, p3]));

        // Root → H1 → [P, H2 → [P, H3 → [P]]]
        assert_eq!(tree.children.len(), 1);
        let s1 = match &tree.children[0] {
            SectionChild::Section(s) => s,
            _ => panic!(),
        };
        assert_eq!(s1.title(), "H1");
        assert_eq!(s1.children.len(), 2);
        let s2 = match &s1.children[1] {
            SectionChild::Section(s) => s,
            _ => panic!(),
        };
        assert_eq!(s2.title(), "H2");
        let s3 = match &s2.children[1] {
            SectionChild::Section(s) => s,
            _ => panic!(),
        };
        assert_eq!(s3.title(), "H3");
        assert_eq!(tree.depth(), 3);
    }

    #[test]
    fn sibling_h2_under_same_h1_closes_previous() {
        // H1 → H2_a → H2_b — the second H2 should be a *sibling* of H2_a, not
        // a child of it.
        let h1 = block("H1", BlockType::Heading { level: 1 });
        let h2a = block("H2_a", BlockType::Heading { level: 2 });
        let h2b = block("H2_b", BlockType::Heading { level: 2 });
        let tree = build_tree(&doc_from(vec![h1, h2a, h2b]));
        let s1 = match &tree.children[0] {
            SectionChild::Section(s) => s,
            _ => panic!(),
        };
        assert_eq!(s1.children.len(), 2);
        assert!(matches!(s1.children[0], SectionChild::Section(_)));
        assert!(matches!(s1.children[1], SectionChild::Section(_)));
    }

    #[test]
    fn iter_blocks_visits_in_reading_order() {
        let h1 = block("H1", BlockType::Heading { level: 1 });
        let p1 = block("p1", BlockType::Paragraph);
        let h2 = block("H2", BlockType::Heading { level: 2 });
        let p2 = block("p2", BlockType::Paragraph);
        let p3 = block("p3", BlockType::Paragraph);
        let tree = build_tree(&doc_from(vec![h1, p1, h2, p2, p3]));

        let contents: Vec<String> = tree.iter_blocks().map(|b| b.content.clone()).collect();
        assert_eq!(contents, vec!["p1", "p2", "p3"]);
    }

    #[test]
    fn round_trip_through_json() {
        let h1 = block("H1", BlockType::Heading { level: 1 });
        let p = block("body", BlockType::Paragraph);
        let tree = build_tree(&doc_from(vec![h1, p]));
        let json = serde_json::to_string(&tree).unwrap();
        let back: Section = serde_json::from_str(&json).unwrap();
        assert_eq!(tree, back);
    }
}
