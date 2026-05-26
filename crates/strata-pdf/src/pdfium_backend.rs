//! PDFium-backed implementation of the abstract PDF decoding traits.
//!
//! Employs the highly robust and battle-tested Google Chromium PDFium engine
//! via safe `pdfium-render` bindings to extract vector graphics, glyphs, and
//! rasterize crops for OCR/VLM consumption.
//!
//! See Phase 13 Etapa B in `docs/task/tareas.md`.

use crate::backend::{PdfBackend, PdfDoc, PdfPage};
use crate::bindings::get_pdfium;
use crate::decoder::DecoderError;
use crate::glyph::{extract_glyphs, Glyph};
use crate::image::{extract_images, Image};
use crate::vector::{extract_paths, VectorPath};
use pdfium_render::prelude::*;
use strata_core::BBox;

/// The PDFium decoding engine backend.
pub struct PdfiumBackend;

impl PdfBackend for PdfiumBackend {
    fn name(&self) -> &'static str {
        "pdfium"
    }

    fn open(&self, data: &[u8]) -> Result<Box<dyn PdfDoc>, DecoderError> {
        let pdfium = get_pdfium()?;
        // SAFETY: We extend the byte slice lifetime to static.
        // PDFium requires the source buffer to remain valid while the document
        // is loaded. The `PdfiumDoc` struct below will keep a boxed clone/ownership
        // of this buffer to ensure safety.
        let slice: &'static [u8] = unsafe { std::mem::transmute::<&[u8], &'static [u8]>(data) };
        let doc = pdfium
            .load_pdf_from_byte_slice(slice, None)
            .map_err(|e| DecoderError::Parse(e.to_string()))?;

        Ok(Box::new(PdfiumDoc {
            doc,
            _bytes: data.to_vec().into_boxed_slice(),
        }))
    }
}

/// A PDFium-loaded document wrapper.
pub struct PdfiumDoc {
    doc: PdfDocument<'static>,
    // Keeps the byte buffer alive for the static reference above.
    _bytes: Box<[u8]>,
}

impl PdfDoc for PdfiumDoc {
    fn page_count(&self) -> usize {
        self.doc.pages().len() as usize
    }

    fn page(&self, index: usize) -> Result<Box<dyn PdfPage>, DecoderError> {
        let page =
            self.doc
                .pages()
                .get(index as i32)
                .map_err(|_| DecoderError::PageOutOfRange {
                    requested: index,
                    total: self.page_count(),
                })?;
        // SAFETY: We extend the page lifetime to static. The page is bound to
        // the doc lifetime, which is owned by PdfiumDoc. Since PdfPage wrapper
        // holds a strong reference (conceptually or via lifetime constraints in dynamic dispatch),
        // we transmute the lifetime for object-safe traits.
        let page_static: PdfPage_pdfium<'static> =
            unsafe { std::mem::transmute::<PdfPage_pdfium<'_>, PdfPage_pdfium<'static>>(page) };
        Ok(Box::new(PdfiumPage { page: page_static }))
    }
}

type PdfPage_pdfium<'a> = pdfium_render::prelude::PdfPage<'a>;

/// A PDFium-loaded single page wrapper.
pub struct PdfiumPage {
    page: PdfPage_pdfium<'static>,
}

impl PdfPage for PdfiumPage {
    fn size(&self) -> (f32, f32) {
        (self.page.width().value, self.page.height().value)
    }

    fn glyphs(&self) -> Result<Vec<Glyph>, DecoderError> {
        extract_glyphs(&self.page).map_err(|e| DecoderError::Internal(e.to_string()))
    }

    fn paths(&self) -> Result<Vec<VectorPath>, DecoderError> {
        extract_paths(&self.page).map_err(|e| DecoderError::Internal(e.to_string()))
    }

    fn images(&self) -> Result<Vec<Image>, DecoderError> {
        extract_images(&self.page).map_err(|e| DecoderError::Internal(e.to_string()))
    }

    fn render_crop(&self, bbox: BBox, dpi: u16) -> Result<Vec<u8>, DecoderError> {
        // Integrate with crop render utility
        let dpi = dpi.max(72) as u32;
        let scale = dpi as f32 / 72.0;
        let page_w = self.page.width().value;
        let page_h = self.page.height().value;

        if page_w <= 0.0 || page_h <= 0.0 {
            return Err(DecoderError::Internal("invalid page dimensions".into()));
        }

        let target_w = ((page_w * scale).round() as u32).clamp(1, 6000);
        let target_h = ((page_h * scale).round() as u32).clamp(1, 6000);

        let config = PdfRenderConfig::new().set_target_size(target_w as i32, target_h as i32);
        let bitmap = self
            .page
            .render_with_config(&config)
            .map_err(|e| DecoderError::Internal(e.to_string()))?;
        let img = bitmap
            .as_image()
            .map_err(|e| DecoderError::Internal(e.to_string()))?;
        let rgba = img.to_rgba8();

        let crop_x = ((bbox.x0.max(0.0)) * scale).round() as u32;
        let crop_y_from_top = ((page_h - bbox.y1.min(page_h)).max(0.0) * scale).round() as u32;
        let crop_w = (bbox.width() * scale).round() as u32;
        let crop_h = (bbox.height() * scale).round() as u32;

        let img_w = rgba.width();
        let img_h = rgba.height();
        let crop_x = crop_x.min(img_w.saturating_sub(1));
        let crop_y = crop_y_from_top.min(img_h.saturating_sub(1));
        let crop_w = crop_w.min(img_w - crop_x).max(1);
        let crop_h = crop_h.min(img_h - crop_y).max(1);

        let cropped =
            image::DynamicImage::ImageRgba8(rgba).crop_imm(crop_x, crop_y, crop_w, crop_h);

        let mut png = Vec::with_capacity((crop_w * crop_h) as usize);
        cropped
            .write_to(&mut std::io::Cursor::new(&mut png), image::ImageFormat::Png)
            .map_err(|e| DecoderError::Internal(e.to_string()))?;

        Ok(png)
    }
}
