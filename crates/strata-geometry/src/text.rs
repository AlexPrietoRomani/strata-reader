//! Módulo: text
//! Modificación: 2026-05-22
//! Autor: Antigravity
//!
//! Descripción:
//! Este módulo contiene utilidades para la normalización de texto extraído
//! de documentos PDF, corrigiendo espaciados dobles o múltiples y resolviendo
//! problemas de espaciado artificial entre letras (letter spacing).
//!
//! Estructura Interna:
//! - `normalize_whitespace`: Reduce espacios en blanco múltiples a uno solo y hace trim.
//! - `fix_letter_spacing`: Colapsa espacios artificiales dentro de palabras si detecta
//!   que un porcentaje significativo del texto consiste en caracteres individuales sueltos.
//! - `normalize_text`: Aplica ambas transformaciones secuencialmente.

/// Normaliza el espaciado en blanco colapsando secuencias de 2 o más espacios
/// en un único espacio simple, y eliminando los espacios al inicio y final del texto.
///
/// # Argumentos
///
/// * `text` - El texto de entrada a normalizar.
pub fn normalize_whitespace(text: &str) -> String {
    let trimmed = text.trim();
    let mut result = String::with_capacity(trimmed.len());
    let mut last_was_space = false;

    for c in trimmed.chars() {
        if c.is_whitespace() {
            if !last_was_space {
                result.push(' ');
                last_was_space = true;
            }
        } else {
            result.push(c);
            last_was_space = false;
        }
    }
    result
}

/// Corrige problemas de espaciado artificial entre letras individuales (letter-spacing).
///
/// Evalúa si más del 30 % de las palabras en el texto son caracteres individuales de longitud 1.
/// Si es así, asume que hay un error de letter-spacing artificial y colapsa los espacios
/// entre letras individuales, preservando los espacios dobles o mayores como separadores de palabras reales.
///
/// # Argumentos
///
/// * `text` - El texto a corregir.
pub fn fix_letter_spacing(text: &str) -> String {
    let tokens: Vec<&str> = text.split_whitespace().collect();
    if tokens.is_empty() {
        return text.to_string();
    }

    // Contar cuántos tokens consisten en un solo carácter unicode
    let single_char_count = tokens.iter().filter(|t| t.chars().count() == 1).count();
    let ratio = (single_char_count as f32) / (tokens.len() as f32);

    if ratio > 0.30 {
        let mut words = Vec::new();
        let mut consecutive_spaces = 0;
        let mut current_segment = String::new();

        for c in text.chars() {
            if c == ' ' {
                consecutive_spaces += 1;
            } else {
                if consecutive_spaces >= 2 && !current_segment.is_empty() {
                    words.push(current_segment.replace(' ', ""));
                    current_segment.clear();
                }
                consecutive_spaces = 0;
                current_segment.push(c);
            }
        }
        if !current_segment.is_empty() {
            words.push(current_segment.replace(' ', ""));
        }

        words.join(" ")
    } else {
        text.to_string()
    }
}

/// Normaliza un texto aplicando corrección de espaciado entre letras y colapso de espacios en blanco.
///
/// # Argumentos
///
/// * `raw` - El texto crudo a procesar.
pub fn normalize_text(raw: &str) -> String {
    let fixed = fix_letter_spacing(raw);
    normalize_whitespace(&fixed)
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normalize_whitespace() {
        assert_eq!(normalize_whitespace("   hello   world   "), "hello world");
        assert_eq!(normalize_whitespace("hello\n\t world"), "hello world");
        assert_eq!(normalize_whitespace("single"), "single");
        assert_eq!(normalize_whitespace(""), "");
    }

    #[test]
    fn test_fix_letter_spacing_positive() {
        // "j o u r n a l" -> ratio de caracteres sueltos es 100% (>30%)
        assert_eq!(fix_letter_spacing("j o u r n a l"), "journal");

        // "t h i s  i s  a  t e s t" con doble espacio entre palabras reales
        assert_eq!(
            fix_letter_spacing("t h i s  i s  a  t e s t"),
            "this is a test"
        );

        // Mezcla de espaciado artificial
        assert_eq!(
            fix_letter_spacing("u n i v e r s i t y  p r e s s"),
            "university press"
        );
    }

    #[test]
    fn test_fix_letter_spacing_negative() {
        // Frase normal: "a beautiful day in the neighborhood"
        // Tokens: "a" (largo 1), "beautiful", "day", "in", "the", "neighborhood"
        // 1 single out of 6 total = 16.6% (<= 30%), no debe colapsar espacios.
        let sentence = "a beautiful day in the neighborhood";
        assert_eq!(fix_letter_spacing(sentence), sentence);
    }

    #[test]
    fn test_normalize_text() {
        assert_eq!(
            normalize_text("  t h i s  i s   a  t e s t   "),
            "this is a test"
        );
        assert_eq!(
            normalize_text("normal text   with double spaces"),
            "normal text with double spaces"
        );
    }
}
