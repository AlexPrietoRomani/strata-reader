//! JSON Graph-RAG serializer.
//!
//! Plan Maestro §12.T7.4 — emits a flat `{ meta, nodes, edges }` shape
//! that Graph-RAG / Agrisearch consumers can ingest without further
//! transformation:
//!
//! - One [`GraphNode`] per block, carrying its id, kebab-case type,
//!   content, BBox, page, provenance, and a small list of semantic tags
//!   (e.g. ``"section"``, ``"figure"``, ``"citation"``).
//! - Edges encode the three relationships the Plan calls out:
//!   - `contains`   → section ↔ child (block or sub-section).
//!   - `follows`    → reading-order successor inside the same section.
//!   - `caption-of` → figure ↔ adjacent caption block.
//!
//! Output schema is intentionally JSON-friendly (camelCase, no oneof
//! magic) so downstream JS / Python loaders don't need protobuf
//! runtime.

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use strata_core::{BBox, Block, BlockId, BlockType, DocMeta, Document, Provenance};
use strata_fusion::{build_tree, Section, SectionChild};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphNode {
    pub id: BlockId,
    pub node_type: String,
    pub content: String,
    pub bbox: BBox,
    pub page: u32,
    pub provenance: Provenance,
    pub tags: Vec<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphEdge {
    pub from: BlockId,
    pub to: BlockId,
    pub relation: EdgeRelation,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EdgeRelation {
    Contains,
    Follows,
    CaptionOf,
    References,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GraphDocument {
    pub meta: DocMeta,
    pub nodes: Vec<GraphNode>,
    pub edges: Vec<GraphEdge>,
}

/// Render `doc` into the Graph-RAG shape.
pub fn render(doc: &Document) -> GraphDocument {
    let tree = build_tree(doc);

    let mut nodes: Vec<GraphNode> = Vec::new();
    let mut edges: Vec<GraphEdge> = Vec::new();
    let block_page = index_block_page(doc);

    walk(&tree, None, &mut nodes, &mut edges, &block_page);
    attach_caption_edges(doc, &mut edges);

    GraphDocument { meta: doc.meta.clone(), nodes, edges }
}

fn index_block_page(doc: &Document) -> std::collections::HashMap<BlockId, u32> {
    let mut m = std::collections::HashMap::new();
    for p in &doc.pages {
        for b in &p.blocks {
            m.insert(b.id, p.number);
        }
    }
    m
}

fn walk(
    section: &Section,
    parent_id: Option<BlockId>,
    nodes: &mut Vec<GraphNode>,
    edges: &mut Vec<GraphEdge>,
    block_page: &std::collections::HashMap<BlockId, u32>,
) {
    let section_anchor_id = section.heading.as_deref().map(|b| b.id);

    // Emit the heading as a node (if any).
    if let Some(heading) = &section.heading {
        nodes.push(make_node(heading, block_page, &["section".to_string()]));
        if let Some(pid) = parent_id {
            edges.push(GraphEdge { from: pid, to: heading.id, relation: EdgeRelation::Contains });
        }
    }

    let mut prev_block_id: Option<BlockId> = None;

    for child in &section.children {
        match child {
            SectionChild::Block(b) => {
                let tags = tags_for(b);
                nodes.push(make_node(b, block_page, &tags));
                if let Some(anchor) = section_anchor_id {
                    edges.push(GraphEdge {
                        from: anchor,
                        to: b.id,
                        relation: EdgeRelation::Contains,
                    });
                } else if let Some(pid) = parent_id {
                    edges.push(GraphEdge { from: pid, to: b.id, relation: EdgeRelation::Contains });
                }
                if let Some(prev) = prev_block_id {
                    edges.push(GraphEdge { from: prev, to: b.id, relation: EdgeRelation::Follows });
                }
                prev_block_id = Some(b.id);
            }
            SectionChild::Section(sub) => {
                walk(sub, section_anchor_id.or(parent_id), nodes, edges, block_page);
                if let Some(sub_heading) = &sub.heading {
                    if let Some(prev) = prev_block_id {
                        edges.push(GraphEdge {
                            from: prev,
                            to: sub_heading.id,
                            relation: EdgeRelation::Follows,
                        });
                    }
                    prev_block_id = Some(sub_heading.id);
                }
            }
        }
    }
}

fn make_node(
    block: &Arc<Block>,
    block_page: &std::collections::HashMap<BlockId, u32>,
    tags: &[String],
) -> GraphNode {
    let mut all_tags: Vec<String> = tags_for(block);
    for extra in tags {
        if !all_tags.contains(extra) {
            all_tags.push(extra.clone());
        }
    }
    GraphNode {
        id: block.id,
        node_type: block_type_kebab(&block.kind),
        content: block.content.clone(),
        bbox: block.bbox,
        page: block_page.get(&block.id).copied().unwrap_or(0),
        provenance: block.provenance.clone(),
        tags: all_tags,
    }
}

fn block_type_kebab(kind: &BlockType) -> String {
    match kind {
        BlockType::Heading { level } => format!("heading-{level}"),
        BlockType::Paragraph => "paragraph".into(),
        BlockType::List => "list".into(),
        BlockType::Table => "table".into(),
        BlockType::Figure => "figure".into(),
        BlockType::Caption => "caption".into(),
        BlockType::Equation => "equation".into(),
        BlockType::CodeListing => "code-listing".into(),
        BlockType::Footnote => "footnote".into(),
        BlockType::Reference => "reference".into(),
        BlockType::Header => "header".into(),
        BlockType::Footer => "footer".into(),
        BlockType::PageNumber => "page-number".into(),
    }
}

fn tags_for(block: &Block) -> Vec<String> {
    match &block.kind {
        BlockType::Heading { .. } => vec!["section".into()],
        BlockType::Figure => vec!["figure".into()],
        BlockType::Equation => vec!["equation".into()],
        BlockType::Reference => vec!["citation".into()],
        BlockType::Table => vec!["table".into()],
        _ => Vec::new(),
    }
}

/// Detect Figure → Caption adjacency in reading order and emit a
/// `caption-of` edge for each pair. Captions usually appear immediately
/// below their figure in PDFs.
fn attach_caption_edges(doc: &Document, edges: &mut Vec<GraphEdge>) {
    for page in &doc.pages {
        let mut prev: Option<(BlockId, &BlockType)> = None;
        for id in &page.reading_order {
            let Some(block) = page.blocks.iter().find(|b| b.id == *id) else { continue };
            if let Some((prev_id, prev_kind)) = prev {
                let pair_figure_then_caption =
                    matches!(prev_kind, BlockType::Figure) && matches!(block.kind, BlockType::Caption);
                let pair_caption_then_figure =
                    matches!(prev_kind, BlockType::Caption) && matches!(block.kind, BlockType::Figure);
                if pair_figure_then_caption || pair_caption_then_figure {
                    // Edge from figure to caption (caption-of points to the figure).
                    let (from, to) = if matches!(block.kind, BlockType::Caption) {
                        (prev_id, block.id) // figure → caption
                    } else {
                        (block.id, prev_id) // figure → caption (figure is now)
                    };
                    edges.push(GraphEdge { from, to, relation: EdgeRelation::CaptionOf });
                }
            }
            prev = Some((block.id, &block.kind));
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use strata_core::{Page, PageOrientation, Size};

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
    fn empty_document_yields_empty_graph() {
        let g = render(&doc(vec![]));
        assert!(g.nodes.is_empty());
        assert!(g.edges.is_empty());
    }

    #[test]
    fn each_block_becomes_one_node() {
        let blocks = vec![
            block("H1", BlockType::Heading { level: 1 }),
            block("body", BlockType::Paragraph),
        ];
        let g = render(&doc(blocks));
        assert_eq!(g.nodes.len(), 2);
        let types: Vec<&str> = g.nodes.iter().map(|n| n.node_type.as_str()).collect();
        assert!(types.contains(&"heading-1"));
        assert!(types.contains(&"paragraph"));
    }

    #[test]
    fn section_emits_contains_edges() {
        let h = block("H1", BlockType::Heading { level: 1 });
        let h_id = h.id;
        let p = block("body", BlockType::Paragraph);
        let p_id = p.id;
        let g = render(&doc(vec![h, p]));
        assert!(g
            .edges
            .iter()
            .any(|e| e.from == h_id && e.to == p_id && e.relation == EdgeRelation::Contains));
    }

    #[test]
    fn follows_edges_chain_in_reading_order() {
        let h = block("H1", BlockType::Heading { level: 1 });
        let p1 = block("p1", BlockType::Paragraph);
        let p2 = block("p2", BlockType::Paragraph);
        let p3 = block("p3", BlockType::Paragraph);
        let (p1_id, p2_id, p3_id) = (p1.id, p2.id, p3.id);
        let g = render(&doc(vec![h, p1, p2, p3]));
        assert!(g
            .edges
            .iter()
            .any(|e| e.from == p1_id && e.to == p2_id && e.relation == EdgeRelation::Follows));
        assert!(g
            .edges
            .iter()
            .any(|e| e.from == p2_id && e.to == p3_id && e.relation == EdgeRelation::Follows));
    }

    #[test]
    fn figure_caption_pair_emits_caption_of_edge() {
        let fig = block("", BlockType::Figure);
        let cap = block("Fig 1: arch overview", BlockType::Caption);
        let (fig_id, cap_id) = (fig.id, cap.id);
        let g = render(&doc(vec![fig, cap]));
        assert!(g
            .edges
            .iter()
            .any(|e| e.from == fig_id && e.to == cap_id && e.relation == EdgeRelation::CaptionOf));
    }

    #[test]
    fn heading_node_carries_section_tag() {
        let g = render(&doc(vec![block("H1", BlockType::Heading { level: 1 })]));
        assert_eq!(g.nodes.len(), 1);
        assert!(g.nodes[0].tags.contains(&"section".to_string()));
    }

    #[test]
    fn figure_node_carries_figure_tag() {
        let g = render(&doc(vec![block("", BlockType::Figure)]));
        assert_eq!(g.nodes.len(), 1);
        assert!(g.nodes[0].tags.contains(&"figure".to_string()));
    }

    #[test]
    fn graph_round_trips_through_json() {
        let g = render(&doc(vec![
            block("H1", BlockType::Heading { level: 1 }),
            block("body", BlockType::Paragraph),
        ]));
        let json = serde_json::to_string(&g).unwrap();
        let back: GraphDocument = serde_json::from_str(&json).unwrap();
        assert_eq!(g, back);
    }

    #[test]
    fn edge_relation_kebab_case_serialization() {
        assert_eq!(serde_json::to_string(&EdgeRelation::CaptionOf).unwrap(), "\"caption-of\"");
        assert_eq!(serde_json::to_string(&EdgeRelation::Contains).unwrap(), "\"contains\"");
    }
}
