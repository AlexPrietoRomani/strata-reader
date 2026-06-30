//! Módulo: pipeline
//!
//! Descripción:
//! Función principal del pipeline: PDF → ParseArtifacts.
//! Orquesta todas las etapas: apertura del PDF, extracción geométrica,
//! triage, bridge IA, fusión y serialización.

use std::collections::HashMap;
use std::path::Path;
use std::sync::Arc;

use sha2::Digest;
use tracing::{info, warn};

use strata_core::{
    BBox, Block, BlockId, BlockType, DocMeta, Document, Page, PageOrientation, Provenance,
    ProvenanceSource, Size,
};
use strata_geometry::{
    classify_headings, cluster_lines, detect_table_borders, detect_table_candidates,
    filter_noise_lines, merge_lines_into_paragraphs, normalize_text, words_from_line, GlyphInput,
    LineSegment, ParagraphKind, XyCutConfig,
};
use strata_geometry::xy_cut_plus_plus;
use strata_quality::evaluate_cid_health;
use strata_serialize::{render_graph, render_markdown, ImageStrategy, MarkdownOptions};
use strata_triage::{
    triage_block, BlockContext, PageContext, Reason, TriageDecision, TriageProfile, TriageRoute,
    DEFAULT_CROP_DPI,
};
use strata_triage::render_crop;

use crate::artifacts::ParseArtifacts;
use crate::error::PipelineError;
use crate::ia_bridge::{run_ia_tasks, IaTask};
use crate::options::ParsePipelineOptions;

/// Calcula el SHA-256 de un fichero y lo retorna como hex string.
///
/// # Errores
///
/// Retorna `PipelineError::Io` si el archivo no se puede leer.
fn sha256_file(path: &Path) -> Result<String, PipelineError> {
    let bytes = std::fs::read(path)?;
    let hash = sha2::Sha256::digest(&bytes);
    Ok(format!("{hash:x}"))
}

/// Convierte `VectorPath`s a `LineSegment`s para `detect_table_borders`.
///
/// Solo extrae segmentos `MoveTo → LineTo` consecutivos — las curvas y los
/// arcos se ignoran porque no definen bordes de tabla rectangulares.
fn paths_to_line_segments(paths: &[strata_pdf::VectorPath]) -> Vec<LineSegment> {
    use strata_core::Point;
    use strata_pdf::Segment;

    let mut out = Vec::new();
    for path in paths {
        let mut cursor: Option<Point> = None;
        for seg in &path.segments {
            match seg {
                Segment::MoveTo(p) => {
                    cursor = Some(*p);
                }
                Segment::LineTo(p) => {
                    if let Some(start) = cursor {
                        out.push(LineSegment { start, end: *p });
                    }
                    cursor = Some(*p);
                }
                Segment::CurveTo { to, .. } => {
                    // Avanzamos el cursor pero no emitimos un segmento recto.
                    cursor = Some(*to);
                }
                Segment::Close => {}
            }
        }
    }
    out
}

/// Resuelve el `ProfileName` desde la cadena de texto de la opción.
fn resolve_profile(name: &str) -> strata_triage::ProfileName {
    match name {
        "fast" => strata_triage::ProfileName::Fast,
        "scientific" => strata_triage::ProfileName::Scientific,
        _ => strata_triage::ProfileName::Balanced,
    }
}

/// Heurística simple para detectar símbolos matemáticos en un carácter.
fn is_math_symbol(c: char) -> bool {
    matches!(c,
        '\u{00B2}'..='\u{00B3}' |  // superíndices ² ³
        '\u{00B9}'               |  // superíndice ¹
        '\u{0391}'..='\u{03C9}' |  // griego Α-ω
        '\u{2200}'..='\u{22FF}' |  // operadores matemáticos ∀…
        '\u{2A00}'..='\u{2AFF}' |  // operadores matemáticos suplementarios
        '+' | '-' | '=' | '<' | '>'
    )
}

/// Función principal del pipeline: abre el PDF, extrae contenido, lo triaga,
/// invoca IA si es necesario, fusiona y serializa.
///
/// # Errores
///
/// Retorna `PipelineError::PdfOpen` si el PDF no se puede abrir,
/// `PipelineError::PageExtraction` si una página falla, o
/// `PipelineError::Serialization` si la serialización JSON falla.
pub async fn parse_document(opts: ParsePipelineOptions) -> Result<ParseArtifacts, PipelineError> {
    info!(
        input = %opts.input.display(),
        profile = %opts.profile,
        use_ia = opts.use_ia,
        "pipeline iniciado"
    );

    // ── 1. Abrir PDF ──────────────────────────────────────────────────────────
    let decoder = strata_pdf::Decoder::open(&opts.input)
        .map_err(|e| PipelineError::PdfOpen(e.to_string()))?;
    let page_count = decoder.page_count();
    let sha = sha256_file(&opts.input)?;

    let stem = opts
        .input
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "output".into());

    let profile = TriageProfile::by_name(resolve_profile(&opts.profile));

    // ── 2. Procesamiento por página ───────────────────────────────────────────
    let mut doc_pages: Vec<Arc<Page>> = Vec::with_capacity(page_count);
    // IaTasks acumuladas para enviar al bridge en batch.
    let mut ia_tasks: Vec<IaTask> = Vec::new();

    for page_idx in 0..page_count {
        let page = decoder
            .page(page_idx)
            .map_err(|e| PipelineError::PageExtraction {
                page: page_idx,
                reason: e.to_string(),
            })?;
        let (page_w, page_h) = page.size();
        let page_bbox = BBox::new(0.0, 0.0, page_w, page_h)
            .unwrap_or_else(|_| BBox::new(0.0, 0.0, 595.0, 842.0).unwrap());
        let page_area = page_w * page_h;

        // Extraer contenido nativo.
        let glyphs = page.glyphs().unwrap_or_default();
        let paths = page.paths().unwrap_or_default();
        let images = page.images().unwrap_or_default();

        // Detectar escaneo y calidad CID.
        // `is_likely_scan` acepta &dyn PdfPage; Box<dyn PdfPage> deref automáticamente.
        let is_scan = strata_pdf::is_likely_scan(page.as_ref()).unwrap_or(false);
        let char_vec: Vec<char> = glyphs.iter().map(|g| g.unicode).collect();
        let cid_eval = evaluate_cid_health(&char_vec);
        let cid_severity = cid_eval.severity;

        let page_ctx = PageContext {
            is_scanned: is_scan,
            cid_severity,
            page_area,
        };

        // ── Geometría ─────────────────────────────────────────────────────────
        // GlyphInput.unicode es char (no Option<char>), coincide con Glyph.unicode.
        let glyph_inputs: Vec<GlyphInput> = glyphs
            .iter()
            .map(|g| GlyphInput {
                bbox: g.bbox,
                font_size: g.font_size,
                unicode: g.unicode,
            })
            .collect();
        let lines = cluster_lines(&glyph_inputs);
        let filtered_lines = filter_noise_lines(&lines, &glyph_inputs, page_bbox);

        // Detección de tablas con bordes vectoriales.
        let line_segs = paths_to_line_segments(&paths);
        let bordered_tables = detect_table_borders(&line_segs);

        // Detección de tablas sin bordes (alineación de palabras).
        let words_all: Vec<_> = filtered_lines
            .iter()
            .flat_map(|l| words_from_line(l, &glyph_inputs))
            .collect();
        let borderless_candidates = detect_table_candidates(&words_all);

        // Clasificación de headings y agrupación en párrafos.
        let line_font_sizes: Vec<f32> = filtered_lines
            .iter()
            .map(|l| {
                if l.glyph_indices.is_empty() {
                    12.0
                } else {
                    l.glyph_indices
                        .iter()
                        .map(|&i| glyph_inputs[i].font_size)
                        .sum::<f32>()
                        / l.glyph_indices.len() as f32
                }
            })
            .collect();
        let line_bboxes: Vec<BBox> = filtered_lines.iter().map(|l| l.bbox).collect();
        let line_texts: Vec<String> = filtered_lines
            .iter()
            .map(|l| {
                words_from_line(l, &glyph_inputs)
                    .iter()
                    .map(|w| w.text.as_str())
                    .collect::<Vec<_>>()
                    .join(" ")
            })
            .collect();
        let headings = classify_headings(&line_font_sizes, &line_bboxes, &line_texts, page_bbox);
        let paragraph_groups =
            merge_lines_into_paragraphs(&filtered_lines, &glyph_inputs, &headings);

        let mut blocks: Vec<Block> = Vec::new();

        // Bloques de texto (párrafos y encabezados).
        for group in &paragraph_groups {
            let raw = group
                .lines
                .iter()
                .flat_map(|l| words_from_line(l, &glyph_inputs))
                .map(|w| w.text.clone())
                .collect::<Vec<_>>()
                .join(" ");
            let content = normalize_text(&raw);
            if content.is_empty() {
                continue;
            }
            let kind = match group.kind {
                ParagraphKind::Heading { level } => BlockType::Heading { level },
                ParagraphKind::Body => BlockType::Paragraph,
            };
            blocks.push(Block {
                id: BlockId::new(),
                kind,
                bbox: group.bbox,
                content,
                children: vec![],
                metadata: None,
                provenance: Provenance::try_new(ProvenanceSource::Rust, None, 1.0, 0, 0).unwrap(),
            });
        }

        // Bloques de tablas con bordes (nativas — confianza alta).
        for tc in &bordered_tables {
            blocks.push(Block {
                id: BlockId::new(),
                kind: BlockType::Table,
                bbox: tc.bbox,
                content: String::new(),
                children: vec![],
                metadata: None,
                provenance: Provenance::try_new(ProvenanceSource::Rust, None, 0.8, 0, 0).unwrap(),
            });
        }

        // Bloques de tablas sin bordes como candidatos VLM (confianza baja).
        for bc in &borderless_candidates {
            blocks.push(Block {
                id: BlockId::new(),
                kind: BlockType::Table,
                bbox: bc.bbox,
                content: String::new(),
                children: vec![],
                metadata: None,
                provenance: Provenance::try_new(ProvenanceSource::Rust, None, 0.5, 0, 0).unwrap(),
            });
        }

        // Bloques de imágenes rasterizadas embebidas.
        for img in &images {
            blocks.push(Block {
                id: BlockId::new(),
                kind: BlockType::Figure,
                bbox: img.bbox,
                content: String::new(),
                children: vec![],
                metadata: None,
                provenance: Provenance::try_new(ProvenanceSource::Rust, None, 1.0, 0, 0).unwrap(),
            });
        }

        // ── 3. Triage: decidir qué bloques escalan a IA ──────────────────────
        if opts.use_ia {
            for block in &blocks {
                let blk_ctx = BlockContext {
                    bbox: block.bbox,
                    is_table_candidate: matches!(block.kind, BlockType::Table),
                    // `has_borders` es true cuando el bloque proviene de detect_table_borders.
                    has_borders: bordered_tables.iter().any(|t| t.bbox == block.bbox),
                    is_image: matches!(block.kind, BlockType::Figure),
                    contains_math_symbols: block.content.chars().any(is_math_symbol),
                    confidence: block.provenance.confidence,
                };

                let decision: TriageDecision = if opts.force_ocr {
                    TriageDecision::new(TriageRoute::OcrFullPage, Reason::PageIsScanned)
                } else {
                    triage_block(&blk_ctx, &page_ctx, &profile)
                };

                if decision.requires_ia() {
                    // render_crop acepta &dyn PdfPage; Box<dyn PdfPage> deref automáticamente.
                    match render_crop(page.as_ref(), block.bbox, DEFAULT_CROP_DPI) {
                        Ok(png_bytes) if !png_bytes.is_empty() => {
                            let hint = match decision.route {
                                TriageRoute::VlmTable => "table-borderless",
                                TriageRoute::VlmImage => "figure",
                                TriageRoute::VlmFormula => "formula",
                                TriageRoute::OcrFullPage => "ocr-page",
                                TriageRoute::Native => "native",
                            };
                            ia_tasks.push(IaTask {
                                block_id: block.id,
                                block_type: block.kind.clone(),
                                route: decision.route,
                                png_bytes,
                                hint: hint.into(),
                                page_no: (page_idx + 1) as u32,
                                dpi: DEFAULT_CROP_DPI,
                            });
                        }
                        Ok(_) => {
                            warn!(block_id = %block.id, "crop retornó bytes vacíos, saltando IA");
                        }
                        Err(e) => {
                            warn!(block_id = %block.id, error = %e, "render_crop falló, saltando IA");
                        }
                    }
                }
            }
        }

        // ── 4. Guardar imágenes en disco si aplica ────────────────────────────
        if opts.save_images || opts.media_dir.is_some() {
            let figure_blocks: Vec<&Block> = blocks
                .iter()
                .filter(|b| matches!(b.kind, BlockType::Figure))
                .collect();
            for (img, block) in images.iter().zip(figure_blocks.iter()) {
                let target_dir = opts.media_dir.clone().unwrap_or_else(|| {
                    opts.input
                        .parent()
                        .unwrap_or_else(|| Path::new("."))
                        .join(format!("{stem}_images"))
                });
                if let Err(e) = std::fs::create_dir_all(&target_dir) {
                    warn!(error = %e, "no se pudo crear el directorio de medios");
                } else {
                    let file_path = target_dir.join(format!("{}.png", block.id));
                    if let Err(e) = std::fs::write(&file_path, &img.raw_bytes) {
                        warn!(error = %e, "no se pudo guardar el crop de imagen");
                    }
                }
            }
        }

        // ── 5. Reading order (XY-Cut++) ───────────────────────────────────────
        let bboxes: Vec<BBox> = blocks.iter().map(|b| b.bbox).collect();
        let order = xy_cut_plus_plus(&bboxes, XyCutConfig::default());
        let reading_order: Vec<BlockId> = order.iter().map(|&i| blocks[i].id).collect();
        let block_arcs: Vec<Arc<Block>> = blocks.into_iter().map(Arc::new).collect();

        doc_pages.push(Arc::new(Page {
            number: (page_idx + 1) as u32,
            size: Size::new(page_w, page_h)
                .unwrap_or_else(|_| Size::new(595.0, 842.0).unwrap()),
            orientation: if page_w > page_h {
                PageOrientation::Landscape
            } else {
                PageOrientation::Portrait
            },
            blocks: block_arcs,
            reading_order,
            media_box: page_bbox,
        }));
    }

    // ── 6. Construir documento nativo ─────────────────────────────────────────
    let mut doc = Document::new(DocMeta {
        source_sha256: sha,
        source_filename: opts
            .input
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default(),
        schema_version: strata_core::version().to_string(),
        profile: opts.profile.clone(),
        extra: std::collections::BTreeMap::new(),
    });
    doc.pages = doc_pages;

    // ── 7. Invocar IA bridge y fusionar ───────────────────────────────────────
    let ia_results = if opts.use_ia && !ia_tasks.is_empty() {
        info!(tasks = ia_tasks.len(), "enviando tareas IA al bridge");
        match run_ia_tasks(ia_tasks, &opts).await {
            Ok(results) => {
                info!(payloads = results.len(), "payloads IA recibidos");
                results
            }
            Err(e) => {
                warn!(error = %e, "bridge IA falló, continuando con extracción nativa");
                HashMap::new()
            }
        }
    } else {
        HashMap::new()
    };

    // Fusionar si hay resultados IA.
    if !ia_results.is_empty() {
        doc = strata_fusion::merge(&doc, &ia_results);
    }

    // ── 8. Serializar ─────────────────────────────────────────────────────────
    let mut md_opts = MarkdownOptions::default();
    if let Some(ref media_dir) = opts.media_dir {
        md_opts.image_strategy = ImageStrategy::MediaDir {
            dir: media_dir.clone(),
        };
    } else if opts.save_images {
        let dir = opts
            .input
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(format!("{stem}_images"));
        md_opts.image_strategy = ImageStrategy::MediaDir { dir };
    }

    let markdown = render_markdown(&doc, &md_opts);
    let graph_json = serde_json::to_value(render_graph(&doc))
        .map_err(|e| PipelineError::Serialization(e.to_string()))?;

    info!(pages = page_count, "pipeline completado");

    Ok(ParseArtifacts {
        document: doc,
        markdown,
        graph_json,
        media_paths: vec![],
    })
}
