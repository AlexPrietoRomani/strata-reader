//! Módulo: headings
//! Modificación: 2026-05-22
//! Autor: Antigravity
//!
//! Descripción:
//! Clasificación del nivel de encabezado basada en la agrupación por tamaño de fuente.
//! Adicionalmente, este módulo aplica filtros avanzados basados en el contenido de la línea
//! y su posición vertical en la página para evitar clasificar erróneamente ruidos, marcas
//! de agua o números sueltos como encabezados.
//!
//! Estructura Interna:
//! - `HeadingClass`: Enum que representa Body o Heading con nivel.
//! - `heading_content_filter`: Filtra textos cortos o no alfabéticos.
//! - `heading_position_filter`: Filtra líneas en los márgenes extremos de la página.
//! - `classify_headings`: Clasificación principal combinando tamaño, contenido y posición.

use serde::{Deserialize, Serialize};
use strata_core::BBox;

/// Resultado de [`classify_headings`].
#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "kind")]
pub enum HeadingClass {
    /// Encabezado a nivel 1 (= el más grande), 2, 3, ...
    Heading { level: u8 },
    /// Cuerpo del texto (no es un encabezado).
    Body,
}

const MAX_HEADING_LEVELS: u8 = 6;
/// Relación mínima entre el tamaño de la línea candidata y el promedio del cuerpo
/// para aceptarla como encabezado. 1.10 = el tamaño de fuente debe superar al cuerpo por >= 10 %.
const MIN_RELATIVE_SIZE: f32 = 1.10;

/// Filtra líneas que tienen un contenido no apto para ser un encabezado.
///
/// Retorna `false` si el texto tiene 2 o menos caracteres alfanuméricos,
/// o si consiste únicamente en números o símbolos (sin letras).
///
/// # Argumentos
///
/// * `text` - El texto de la línea a evaluar.
pub fn heading_content_filter(text: &str) -> bool {
    let trimmed = text.trim();

    // Contar caracteres alfanuméricos
    let alnum_count = trimmed.chars().filter(|c| c.is_alphanumeric()).count();
    if alnum_count <= 2 {
        return false;
    }

    // Debe contener al menos una letra (evita solo números o símbolos sueltos de arXiv)
    let has_letter = trimmed.chars().any(|c| c.is_alphabetic());
    if !has_letter {
        return false;
    }

    true
}

/// Filtra líneas ubicadas en los extremos superior o inferior de la página.
///
/// Retorna `false` si la línea está en el 8 % superior de la página
/// o en el 5 % inferior de la página (que típicamente corresponden a headers/footers).
///
/// # Argumentos
///
/// * `bbox` - La caja delimitadora de la línea.
/// * `page_bbox` - La caja delimitadora de la página completa.
pub fn heading_position_filter(bbox: BBox, page_bbox: BBox) -> bool {
    let page_height = page_bbox.height();
    if page_height <= 0.0 {
        return true;
    }

    let relative_y = (bbox.y0 - page_bbox.y0) / page_height;

    // 5 % inferior
    if relative_y < 0.05 {
        return false;
    }
    // 8 % superior
    if relative_y > 0.92 {
        return false;
    }

    true
}

/// Clasifica cada línea según su `dominant_font_size`, aplicando filtros espaciales y de contenido.
///
/// # Argumentos
///
/// * `line_font_sizes` - Lista de tamaños de fuente de las líneas.
/// * `line_bboxes` - Lista de cajas delimitadoras de las líneas.
/// * `line_texts` - Lista de contenidos de texto de las líneas.
/// * `page_bbox` - Caja delimitadora de la página.
pub fn classify_headings(
    line_font_sizes: &[f32],
    line_bboxes: &[BBox],
    line_texts: &[String],
    page_bbox: BBox,
) -> Vec<HeadingClass> {
    if line_font_sizes.is_empty() {
        return Vec::new();
    }

    // Limpiar tamaños no válidos
    let cleaned: Vec<f32> = line_font_sizes
        .iter()
        .map(|s| if s.is_finite() && *s > 0.0 { *s } else { 0.0 })
        .collect();

    let body_size = body_text_size(&cleaned);
    if body_size <= 0.0 {
        return vec![HeadingClass::Body; cleaned.len()];
    }
    let levels = build_heading_levels(&cleaned, body_size);

    cleaned
        .iter()
        .enumerate()
        .map(|(i, &s)| {
            let bbox = line_bboxes[i];
            let text = &line_texts[i];

            // 1. Aplicar los filtros de contenido y posición
            if !heading_content_filter(text)
                || !heading_position_filter(bbox, page_bbox)
                || s < body_size * MIN_RELATIVE_SIZE
            {
                HeadingClass::Body
            } else {
                let level = levels
                    .iter()
                    .position(|&l| (l - s).abs() < 0.51)
                    .map(|idx| (idx as u8 + 1).min(MAX_HEADING_LEVELS))
                    .unwrap_or(MAX_HEADING_LEVELS);
                HeadingClass::Heading { level }
            }
        })
        .collect()
}

/// Retorna el tamaño de fuente del cuerpo de texto — el más frecuente en la página.
fn body_text_size(sizes: &[f32]) -> f32 {
    let bins = histogram(sizes);
    bins.into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(size, _)| size)
        .unwrap_or(0.0)
}

/// Agrupa tamaños superiores al del cuerpo y los ordena de mayor a menor (H1, H2, ...).
fn build_heading_levels(sizes: &[f32], body_size: f32) -> Vec<f32> {
    let bins = histogram(sizes);
    let mut larger: Vec<(f32, u32)> = bins
        .into_iter()
        .filter(|(s, _)| *s >= body_size * MIN_RELATIVE_SIZE)
        .collect();
    if larger.is_empty() {
        return Vec::new();
    }
    larger.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(std::cmp::Ordering::Equal));

    let mut merged: Vec<f32> = Vec::new();
    for (s, _) in larger {
        match merged.last_mut() {
            Some(last) if (*last - s).abs() < 1.0 => {
                *last = (*last + s) * 0.5;
            }
            _ => merged.push(s),
        }
    }
    merged
}

fn histogram(values: &[f32]) -> Vec<(f32, u32)> {
    let mut bins: std::collections::BTreeMap<i32, (f32, u32)> = Default::default();
    for &v in values {
        if !v.is_finite() || v <= 0.0 {
            continue;
        }
        let key = (v * 2.0).round() as i32;
        let entry = bins.entry(key).or_insert((0.0, 0));
        let n = entry.1 as f32;
        entry.0 = entry.0 + (v - entry.0) / (n + 1.0);
        entry.1 += 1;
    }
    bins.into_values().collect()
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn dummy_bbox(y: f32) -> BBox {
        BBox::new(50.0, y, 200.0, y + 10.0).unwrap()
    }

    #[test]
    fn test_heading_content_filter() {
        assert!(heading_content_filter("Introduction"));
        assert!(heading_content_filter("1. Related Work"));
        assert!(!heading_content_filter("A")); // muy corto
        assert!(!heading_content_filter("12.34")); // solo números
        assert!(!heading_content_filter("####")); // solo símbolos
    }

    #[test]
    fn test_heading_position_filter() {
        let page_bbox = BBox::new(0.0, 0.0, 595.0, 842.0).unwrap();

        // 400 pt es la mitad (válido)
        assert!(heading_position_filter(dummy_bbox(400.0), page_bbox));
        // y=20 es < 5 % (inválido, footer/page number)
        assert!(!heading_position_filter(dummy_bbox(20.0), page_bbox));
        // y=800 es > 92 % (inválido, header)
        assert!(!heading_position_filter(dummy_bbox(800.0), page_bbox));
    }

    #[test]
    fn empty_input_yields_empty() {
        let page_bbox = BBox::new(0.0, 0.0, 595.0, 842.0).unwrap();
        assert!(classify_headings(&[], &[], &[], page_bbox).is_empty());
    }

    #[test]
    fn uniform_size_is_all_body() {
        let page_bbox = BBox::new(0.0, 0.0, 595.0, 842.0).unwrap();
        let sizes = vec![10.0_f32; 20];
        let bboxes = vec![dummy_bbox(400.0); 20];
        let texts = vec!["This is a body line text example.".to_string(); 20];

        let r = classify_headings(&sizes, &bboxes, &texts, page_bbox);
        assert!(r.iter().all(|c| matches!(c, HeadingClass::Body)));
    }

    #[test]
    fn one_big_line_becomes_h1() {
        let page_bbox = BBox::new(0.0, 0.0, 595.0, 842.0).unwrap();
        let mut sizes = vec![10.0_f32; 10];
        sizes.push(18.0);
        let mut bboxes = vec![dummy_bbox(400.0); 10];
        bboxes.push(dummy_bbox(600.0));
        let mut texts: Vec<String> = vec!["This is a body line.".to_string(); 10];
        texts.push("Main Title of the Document".to_string());

        let r = classify_headings(&sizes, &bboxes, &texts, page_bbox);
        assert_eq!(r[10], HeadingClass::Heading { level: 1 });
        for c in &r[..10] {
            assert_eq!(*c, HeadingClass::Body);
        }
    }
}
