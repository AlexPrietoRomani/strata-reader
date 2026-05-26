//! Módulo: noise
//! Modificación: 2026-05-22
//! Autor: Antigravity
//!
//! Descripción:
//! Este módulo contiene los algoritmos para detectar y filtrar ruido en las líneas
//! extraídas de un PDF, incluyendo marcas de agua de arXiv, números de página al pie,
//! y caracteres sueltos o huérfanos (stray characters).
//!
//! Estructura Interna:
//! - `is_arxiv_watermark`: Detecta números de versión arXiv.
//! - `is_stray_char`: Detecta caracteres no alfanuméricos aislados.
//! - `is_page_number`: Detecta números de página en el margen inferior.
//! - `filter_noise_lines`: Filtra una colección de líneas aplicando los criterios anteriores.

use crate::word_line::{GlyphInput, Line};
use strata_core::BBox;

/// Determina si una línea es una marca de agua de arXiv.
///
/// Detecta números de versión arXiv (por ejemplo, "1706.03762v5" o "1706.03762"),
/// caracterizados por tener solo dígitos, puntos o la letra 'v'/'V', con un tamaño
/// de fuente relativamente grande, y ubicados en el tercio superior de la página.
///
/// # Argumentos
///
/// * `line` - La línea a evaluar.
/// * `glyphs` - El slice completo de glifos de la página.
/// * `page_bbox` - La caja delimitadora de la página completa.
pub fn is_arxiv_watermark(line: &Line, glyphs: &[GlyphInput], page_bbox: BBox) -> bool {
    let page_height = page_bbox.height();
    if page_height <= 0.0 {
        return false;
    }

    // 1. Verificar si la línea está en el tercio superior de la página (Y >= 2/3 de la altura)
    let relative_y = (line.baseline_y - page_bbox.y0) / page_height;
    if relative_y < 2.0 / 3.0 {
        return false;
    }

    // 2. Extraer el texto de la línea ignorando espacios en blanco
    let text: String = line
        .glyph_indices
        .iter()
        .map(|&i| glyphs[i].unicode)
        .filter(|c| !c.is_whitespace())
        .collect();

    if text.is_empty() {
        return false;
    }

    // El patrón típico de un número de versión arXiv contiene solo dígitos, puntos, y la letra 'v' o 'V'.
    let all_chars_valid = text
        .chars()
        .all(|c| c.is_ascii_digit() || c == '.' || c == 'v' || c == 'V');
    if !all_chars_valid {
        return false;
    }

    // 3. Verificar si el tamaño de fuente es grande (mediana >= 12.0 pt)
    let median_size = median_font_size(line, glyphs);
    if median_size < 12.0 {
        return false;
    }

    true
}

/// Determina si una línea consiste únicamente en un carácter huérfano o "stray character".
///
/// Evalúa si el texto de la línea (ignorando espacios) contiene exactamente un
/// único carácter y este carácter es no alfanumérico (por ejemplo, `#`, `*`, `_`, etc.).
///
/// # Argumentos
///
/// * `line` - La línea a evaluar.
/// * `glyphs` - El slice completo de glifos de la página.
pub fn is_stray_char(line: &Line, glyphs: &[GlyphInput]) -> bool {
    let text: String = line
        .glyph_indices
        .iter()
        .map(|&i| glyphs[i].unicode)
        .filter(|c| !c.is_whitespace())
        .collect();

    if text.chars().count() != 1 {
        return false;
    }

    let first_char = text.chars().next().unwrap();
    !first_char.is_alphanumeric()
}

/// Determina si una línea es un número de página en el footer.
///
/// Identifica números sueltos (solo dígitos) ubicados en el 5 % inferior
/// de la altura de la página.
///
/// # Argumentos
///
/// * `line` - La línea a evaluar.
/// * `glyphs` - El slice completo de glifos de la página.
/// * `page_bbox` - La caja delimitadora de la página completa.
pub fn is_page_number(line: &Line, glyphs: &[GlyphInput], page_bbox: BBox) -> bool {
    let page_height = page_bbox.height();
    if page_height <= 0.0 {
        return false;
    }

    // 1. Verificar si está en el 5 % inferior de la página (Y <= 0.05 de la altura)
    let relative_y = (line.baseline_y - page_bbox.y0) / page_height;
    if relative_y > 0.05 {
        return false;
    }

    // 2. Extraer el texto e ignorar espacios
    let text: String = line
        .glyph_indices
        .iter()
        .map(|&i| glyphs[i].unicode)
        .filter(|c| !c.is_whitespace())
        .collect();

    if text.is_empty() {
        return false;
    }

    // Debe contener únicamente dígitos ascii
    text.chars().all(|c| c.is_ascii_digit())
}

/// Filtra todas las líneas de una página descartando aquellas clasificadas como ruido.
///
/// # Argumentos
///
/// * `lines` - Las líneas extraídas originales.
/// * `glyphs` - El slice de glifos original de la página.
/// * `page_bbox` - La caja delimitadora de la página.
pub fn filter_noise_lines(lines: &[Line], glyphs: &[GlyphInput], page_bbox: BBox) -> Vec<Line> {
    lines
        .iter()
        .filter(|line| {
            !is_arxiv_watermark(line, glyphs, page_bbox)
                && !is_stray_char(line, glyphs)
                && !is_page_number(line, glyphs, page_bbox)
        })
        .cloned()
        .collect()
}

/// Calcula la mediana de los tamaños de fuente de los glifos asociados a una línea.
fn median_font_size(line: &Line, glyphs: &[GlyphInput]) -> f32 {
    let mut sizes: Vec<f32> = line
        .glyph_indices
        .iter()
        .map(|&i| glyphs[i].font_size)
        .filter(|s| s.is_finite() && *s > 0.0)
        .collect();

    if sizes.is_empty() {
        return 0.0;
    }

    sizes.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    sizes[sizes.len() / 2]
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn create_glyph(unicode: char, x0: f32, y0: f32, font_size: f32) -> GlyphInput {
        GlyphInput {
            bbox: BBox::new(x0, y0, x0 + 8.0, y0 + font_size).unwrap(),
            font_size,
            unicode,
        }
    }

    fn create_line(baseline_y: f32, glyph_indices: Vec<usize>, glyphs: &[GlyphInput]) -> Line {
        let mut bbox = glyphs[glyph_indices[0]].bbox;
        for &idx in &glyph_indices[1..] {
            bbox = bbox.union(glyphs[idx].bbox);
        }
        Line {
            bbox,
            baseline_y,
            glyph_indices,
        }
    }

    #[test]
    fn test_is_arxiv_watermark_positive() {
        // Un número de versión arXiv en el tercio superior de una página estándar A4 (595 x 842 pt)
        // baseline_y = 800 (en el tercio superior)
        // font_size = 14.0 pt (grande)
        // text = "1706.03762v5"
        let text_chars = vec!['1', '7', '0', '6', '.', '0', '3', '7', '6', '2', 'v', '5'];
        let glyphs: Vec<GlyphInput> = text_chars
            .iter()
            .enumerate()
            .map(|(i, &c)| create_glyph(c, i as f32 * 10.0, 800.0, 14.0))
            .collect();

        let indices: Vec<usize> = (0..glyphs.len()).collect();
        let line = create_line(800.0, indices, &glyphs);
        let page_bbox = BBox::new(0.0, 0.0, 595.0, 842.0).unwrap();

        assert!(is_arxiv_watermark(&line, &glyphs, page_bbox));
    }

    #[test]
    fn test_is_arxiv_watermark_negative_low_y() {
        // Lo mismo pero en el centro de la página (no es marca de agua)
        let text_chars = vec!['1', '7', '0', '6', '.', '0', '3', '7', '6', '2', 'v', '5'];
        let glyphs: Vec<GlyphInput> = text_chars
            .iter()
            .enumerate()
            .map(|(i, &c)| create_glyph(c, i as f32 * 10.0, 400.0, 14.0))
            .collect();

        let indices: Vec<usize> = (0..glyphs.len()).collect();
        let line = create_line(400.0, indices, &glyphs);
        let page_bbox = BBox::new(0.0, 0.0, 595.0, 842.0).unwrap();

        assert!(!is_arxiv_watermark(&line, &glyphs, page_bbox));
    }

    #[test]
    fn test_is_arxiv_watermark_negative_small_font() {
        // En el tercio superior pero tamaño pequeño (podría ser texto normal, ej. nota al margen)
        let text_chars = vec!['1', '7', '0', '6', '.', '0', '3', '7', '6', '2', 'v', '5'];
        let glyphs: Vec<GlyphInput> = text_chars
            .iter()
            .enumerate()
            .map(|(i, &c)| create_glyph(c, i as f32 * 10.0, 800.0, 9.0))
            .collect();

        let indices: Vec<usize> = (0..glyphs.len()).collect();
        let line = create_line(800.0, indices, &glyphs);
        let page_bbox = BBox::new(0.0, 0.0, 595.0, 842.0).unwrap();

        assert!(!is_arxiv_watermark(&line, &glyphs, page_bbox));
    }

    #[test]
    fn test_is_arxiv_watermark_negative_letters() {
        // En el tercio superior y tamaño grande pero contiene letras comunes (ej. un título de sección)
        let text_chars = vec!['I', 'n', 't', 'r', 'o', 'd', 'u', 'c', 't', 'i', 'o', 'n'];
        let glyphs: Vec<GlyphInput> = text_chars
            .iter()
            .enumerate()
            .map(|(i, &c)| create_glyph(c, i as f32 * 10.0, 800.0, 16.0))
            .collect();

        let indices: Vec<usize> = (0..glyphs.len()).collect();
        let line = create_line(800.0, indices, &glyphs);
        let page_bbox = BBox::new(0.0, 0.0, 595.0, 842.0).unwrap();

        assert!(!is_arxiv_watermark(&line, &glyphs, page_bbox));
    }

    #[test]
    fn test_is_stray_char_positive() {
        // Un único carácter no alfanumérico como '#'
        let glyphs = vec![create_glyph('#', 10.0, 100.0, 10.0)];
        let line = create_line(100.0, vec![0], &glyphs);

        assert!(is_stray_char(&line, &glyphs));
    }

    #[test]
    fn test_is_stray_char_negative_alphanumeric() {
        // Un único carácter alfanumérico como '3' o 'A'
        let glyphs1 = vec![create_glyph('3', 10.0, 100.0, 10.0)];
        let line1 = create_line(100.0, vec![0], &glyphs1);
        assert!(!is_stray_char(&line1, &glyphs1));

        let glyphs2 = vec![create_glyph('A', 10.0, 100.0, 10.0)];
        let line2 = create_line(100.0, vec![0], &glyphs2);
        assert!(!is_stray_char(&line2, &glyphs2));
    }

    #[test]
    fn test_is_stray_char_negative_multiple() {
        // Múltiples caracteres no alfanuméricos como "##"
        let glyphs = vec![
            create_glyph('#', 10.0, 100.0, 10.0),
            create_glyph('#', 20.0, 100.0, 10.0),
        ];
        let line = create_line(100.0, vec![0, 1], &glyphs);

        assert!(!is_stray_char(&line, &glyphs));
    }

    #[test]
    fn test_is_page_number_positive() {
        // Número de página "15" en el 5 % inferior de la página (y=20 en página de 842)
        let glyphs = vec![
            create_glyph('1', 290.0, 20.0, 10.0),
            create_glyph('5', 298.0, 20.0, 10.0),
        ];
        let line = create_line(20.0, vec![0, 1], &glyphs);
        let page_bbox = BBox::new(0.0, 0.0, 595.0, 842.0).unwrap();

        assert!(is_page_number(&line, &glyphs, page_bbox));
    }

    #[test]
    fn test_is_page_number_negative_high_y() {
        // Los mismos números pero en el cuerpo del texto (y=100)
        let glyphs = vec![
            create_glyph('1', 290.0, 100.0, 10.0),
            create_glyph('5', 298.0, 100.0, 10.0),
        ];
        let line = create_line(100.0, vec![0, 1], &glyphs);
        let page_bbox = BBox::new(0.0, 0.0, 595.0, 842.0).unwrap();

        assert!(!is_page_number(&line, &glyphs, page_bbox));
    }

    #[test]
    fn test_is_page_number_negative_with_letters() {
        // En la parte inferior pero contiene letras (ej. "Page 15" o "15 pt")
        let text_chars = vec!['P', 'a', 'g', 'e', ' ', '1', '5'];
        let glyphs: Vec<GlyphInput> = text_chars
            .iter()
            .enumerate()
            .map(|(i, &c)| create_glyph(c, i as f32 * 10.0, 20.0, 10.0))
            .collect();

        let indices: Vec<usize> = (0..glyphs.len()).collect();
        let line = create_line(20.0, indices, &glyphs);
        let page_bbox = BBox::new(0.0, 0.0, 595.0, 842.0).unwrap();

        assert!(!is_page_number(&line, &glyphs, page_bbox));
    }

    #[test]
    fn test_filter_noise_lines() {
        let page_bbox = BBox::new(0.0, 0.0, 595.0, 842.0).unwrap();

        // 1. Una marca de agua de arXiv (ruido)
        let glyphs1 = vec![
            create_glyph('1', 10.0, 800.0, 14.0),
            create_glyph('.', 18.0, 800.0, 14.0),
            create_glyph('2', 26.0, 800.0, 14.0),
        ];
        // 2. Un texto normal (cuerpo)
        let glyphs2 = vec![
            create_glyph('T', 10.0, 400.0, 10.0),
            create_glyph('e', 18.0, 400.0, 10.0),
            create_glyph('x', 26.0, 400.0, 10.0),
            create_glyph('t', 34.0, 400.0, 10.0),
        ];
        // 3. Un stray char (ruido)
        let glyphs3 = vec![create_glyph('*', 10.0, 300.0, 10.0)];
        // 4. Un número de página (ruido)
        let glyphs4 = vec![create_glyph('3', 290.0, 20.0, 10.0)];

        let mut all_glyphs = Vec::new();
        all_glyphs.extend(glyphs1);
        all_glyphs.extend(glyphs2);
        all_glyphs.extend(glyphs3);
        all_glyphs.extend(glyphs4);

        // Índices en el vector global
        let line1 = create_line(800.0, vec![0, 1, 2], &all_glyphs);
        let line2 = create_line(400.0, vec![3, 4, 5, 6], &all_glyphs);
        let line3 = create_line(300.0, vec![7], &all_glyphs);
        let line4 = create_line(20.0, vec![8], &all_glyphs);

        let lines = vec![line1, line2, line3, line4];
        let filtered = filter_noise_lines(&lines, &all_glyphs, page_bbox);

        // Debería quedar solo la línea 2 (cuerpo de texto)
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].baseline_y, 400.0);
    }
}
