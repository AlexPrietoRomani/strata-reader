//! [`Page`] node of the AST.

use std::sync::Arc;

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::bbox::{BBox, Size};
use crate::block::{Block, BlockId};

#[derive(Copy, Clone, Debug, PartialEq, Eq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "kebab-case")]
pub enum PageOrientation {
    Portrait,
    Landscape,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Page {
    /// 1-indexed page number (matches what PDF readers display).
    pub number: u32,
    /// Media-box size in PDF points.
    pub size: Size,
    pub orientation: PageOrientation,
    /// Blocks on this page, in undefined order. The semantic order is given
    /// by [`Page::reading_order`].
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocks: Vec<Arc<Block>>,
    /// Sequence of block ids resulting from the XY-Cut++ reading-order
    /// algorithm (Plan Maestro §8.T3.3).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub reading_order: Vec<BlockId>,
    /// Media-box rectangle in PDF user space.
    pub media_box: BBox,
}

impl Page {
    pub fn new(number: u32, size: Size, media_box: BBox) -> Self {
        let orientation = if size.width >= size.height {
            PageOrientation::Landscape
        } else {
            PageOrientation::Portrait
        };
        Self {
            number,
            size,
            orientation,
            blocks: Vec::new(),
            reading_order: Vec::new(),
            media_box,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn portrait_when_height_greater_than_width() {
        let size = Size::new(595.0, 842.0).unwrap(); // A4 portrait in points
        let bb = BBox::new(0.0, 0.0, 595.0, 842.0).unwrap();
        let p = Page::new(1, size, bb);
        assert_eq!(p.orientation, PageOrientation::Portrait);
    }

    #[test]
    fn landscape_when_width_greater_than_height() {
        let size = Size::new(842.0, 595.0).unwrap();
        let bb = BBox::new(0.0, 0.0, 842.0, 595.0).unwrap();
        let p = Page::new(1, size, bb);
        assert_eq!(p.orientation, PageOrientation::Landscape);
    }
}
