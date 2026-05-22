//! Módulo: paragraph
//! Modificación: 2026-05-22
//! Autor: Antigravity
//!
//! Descripción:
//! Este módulo contiene la lógica para agrupar líneas físicas consecutivas en
//! bloques semánticos de párrafos y encabezados (ParagraphGroups) basándose en
//! el espacio vertical (gap) y la clasificación de encabezados.
//!
//! Estructura Interna:
//! - `ParagraphKind`: Clasificación del grupo (Body o Heading con su nivel).
//! - `ParagraphGroup`: Contenedor para un conjunto de líneas agrupadas semánticamente.
//! - `merge_lines_into_paragraphs`: Agrupa líneas basándose en el gap vertical.

use strata_core::BBox;
use crate::word_line::{GlyphInput, Line};
use crate::headings::HeadingClass;

/// Representa el tipo de un grupo de párrafos.
#[derive(Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "kebab-case", tag = "kind")]
pub enum ParagraphKind {
    /// Cuerpo de texto general.
    Body,
    /// Encabezado con su nivel de jerarquía.
    Heading { level: u8 },
}

impl Default for ParagraphKind {
    fn default() -> Self {
        Self::Body
    }
}

/// Un grupo semántico de líneas que forman un párrafo continuo o un encabezado.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ParagraphGroup {
    /// Las líneas físicas que componen este grupo.
    pub lines: Vec<Line>,
    /// La caja delimitadora combinada de todas las líneas en este grupo.
    pub bbox: BBox,
    /// El tipo semántico del grupo.
    pub kind: ParagraphKind,
}

/// Agrupa líneas físicas consecutivas de texto en párrafos semánticos continuos.
///
/// Dos líneas de cuerpo (`HeadingClass::Body`) consecutivas se agrupan en el mismo
/// párrafo si el espacio vertical (gap) entre ellas es menor o igual al 70 % de la
/// altura mediana de todas las líneas de la página. Cualquier línea clasificada como
/// encabezado (`HeadingClass::Heading`) siempre inicia y termina los grupos, quedando
/// en su propio grupo independiente.
///
/// # Argumentos
///
/// * `lines` - Las líneas de texto físicas de la página (ordenadas de arriba a abajo).
/// * `glyphs` - El slice completo de glifos de la página.
/// * `headings` - Clasificación de cabeceras correspondiente a cada línea.
pub fn merge_lines_into_paragraphs(
    lines: &[Line],
    _glyphs: &[GlyphInput],
    headings: &[HeadingClass],
) -> Vec<ParagraphGroup> {
    if lines.is_empty() {
        return Vec::new();
    }

    // 1. Calcular la altura mediana de todas las líneas de la página
    let mut heights: Vec<f32> = lines
        .iter()
        .map(|l| l.bbox.height())
        .filter(|h| h.is_finite() && *h > 0.0)
        .collect();

    let median_line_height = if heights.is_empty() {
        10.0 // Valor por defecto si no hay líneas con altura válida
    } else {
        heights.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        heights[heights.len() / 2]
    };

    let threshold = 0.7 * median_line_height;
    let mut groups: Vec<ParagraphGroup> = Vec::new();
    let mut active_body_group: Option<ParagraphGroup> = None;

    for (i, line) in lines.iter().enumerate() {
        let heading_class = headings.get(i).copied().unwrap_or(HeadingClass::Body);

        match heading_class {
            HeadingClass::Heading { level } => {
                // Si hay un grupo de cuerpo activo, se cierra y se guarda
                if let Some(group) = active_body_group.take() {
                    groups.push(group);
                }
                // Los encabezados siempre forman su propio grupo independiente de inmediato
                groups.push(ParagraphGroup {
                    lines: vec![line.clone()],
                    bbox: line.bbox,
                    kind: ParagraphKind::Heading { level },
                });
            }
            HeadingClass::Body => {
                if let Some(ref mut group) = active_body_group {
                    // Calcular el gap vertical entre la última línea agregada y la actual.
                    // En coordenadas PDF, el origen está abajo, por lo que las líneas arriba
                    // tienen Y más alto. Así, la distancia es L_prev.y0 - L_curr.y1.
                    let prev_line = group.lines.last().unwrap();
                    let gap = prev_line.bbox.y0 - line.bbox.y1;

                    if gap <= threshold {
                        // El gap está bajo el límite: se une al párrafo activo
                        group.lines.push(line.clone());
                        group.bbox = group.bbox.union(line.bbox);
                    } else {
                        // El gap es grande: cerramos el párrafo actual e iniciamos uno nuevo
                        if let Some(finished) = active_body_group.replace(ParagraphGroup {
                            lines: vec![line.clone()],
                            bbox: line.bbox,
                            kind: ParagraphKind::Body,
                        }) {
                            groups.push(finished);
                        }
                    }
                } else {
                    // No hay grupo activo, iniciamos uno nuevo para el cuerpo
                    active_body_group = Some(ParagraphGroup {
                        lines: vec![line.clone()],
                        bbox: line.bbox,
                        kind: ParagraphKind::Body,
                    });
                }
            }
        }
    }

    // Guardar el último grupo activo si quedó alguno
    if let Some(group) = active_body_group {
        groups.push(group);
    }

    groups
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn create_line_with_bbox(y0: f32, height: f32) -> Line {
        Line {
            bbox: BBox::new(0.0, y0, 100.0, y0 + height).unwrap(),
            baseline_y: y0,
            glyph_indices: vec![],
        }
    }

    #[test]
    fn test_empty_lines_returns_empty() {
        let groups = merge_lines_into_paragraphs(&[], &[], &[]);
        assert!(groups.is_empty());
    }

    #[test]
    fn test_only_headings_are_independent() {
        // Dos títulos consecutivos deben quedar en dos grupos diferentes
        let lines = vec![
            create_line_with_bbox(200.0, 15.0),
            create_line_with_bbox(170.0, 12.0),
        ];
        let headings = vec![
            HeadingClass::Heading { level: 1 },
            HeadingClass::Heading { level: 2 },
        ];

        let groups = merge_lines_into_paragraphs(&lines, &[], &headings);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].kind, ParagraphKind::Heading { level: 1 });
        assert_eq!(groups[1].kind, ParagraphKind::Heading { level: 2 });
    }

    #[test]
    fn test_merge_consecutive_body_lines_small_gap() {
        // Altura mediana = 10.0 pt. Threshold = 7.0 pt.
        // y0 de L1 es 100.0 (top=110.0)
        // y1 de L2 es 95.0  (L2 y0=85.0) -> gap = 100.0 - 95.0 = 5.0 pt (<= 7.0, merge!)
        let lines = vec![
            create_line_with_bbox(100.0, 10.0),
            create_line_with_bbox(85.0, 10.0),
        ];
        let headings = vec![HeadingClass::Body, HeadingClass::Body];

        let groups = merge_lines_into_paragraphs(&lines, &[], &headings);
        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].kind, ParagraphKind::Body);
        assert_eq!(groups[0].lines.len(), 2);
        // La caja combinada de [0, 85, 100, 110]
        assert_eq!(groups[0].bbox, BBox::new(0.0, 85.0, 100.0, 110.0).unwrap());
    }

    #[test]
    fn test_split_body_lines_large_gap() {
        // Altura mediana = 10.0 pt. Threshold = 7.0 pt.
        // y0 de L1 es 100.0 (top=110.0)
        // y1 de L2 es 90.0  (L2 y0=80.0) -> gap = 100.0 - 90.0 = 10.0 pt (> 7.0, split!)
        let lines = vec![
            create_line_with_bbox(100.0, 10.0),
            create_line_with_bbox(80.0, 10.0),
        ];
        let headings = vec![HeadingClass::Body, HeadingClass::Body];

        let groups = merge_lines_into_paragraphs(&lines, &[], &headings);
        assert_eq!(groups.len(), 2);
        assert_eq!(groups[0].kind, ParagraphKind::Body);
        assert_eq!(groups[1].kind, ParagraphKind::Body);
        assert_eq!(groups[0].lines.len(), 1);
        assert_eq!(groups[1].lines.len(), 1);
    }

    #[test]
    fn test_heading_splits_body_paragraphs() {
        // L1 (Body), L2 (Heading), L3 (Body) -> deben resultar 3 grupos
        let lines = vec![
            create_line_with_bbox(120.0, 10.0),
            create_line_with_bbox(100.0, 12.0),
            create_line_with_bbox(80.0, 10.0),
        ];
        let headings = vec![
            HeadingClass::Body,
            HeadingClass::Heading { level: 1 },
            HeadingClass::Body,
        ];

        let groups = merge_lines_into_paragraphs(&lines, &[], &headings);
        assert_eq!(groups.len(), 3);
        assert_eq!(groups[0].kind, ParagraphKind::Body);
        assert_eq!(groups[1].kind, ParagraphKind::Heading { level: 1 });
        assert_eq!(groups[2].kind, ParagraphKind::Body);
    }
}
