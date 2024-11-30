use foundation::algorithm::slice::slice::Slice;
use foundation::algorithm::slice::slice_type::SliceType;
use foundation::i_overlay::i_float::rect::IntRect;
use foundation::i_overlay::i_shape::int::path::{IntPath, PointPathExtension};
use foundation::i_overlay::i_shape::int::shape::IntShapes;
use crate::connectivity::rect::Rect;

pub(super) struct Piece {
    pub(super) rect: IntRect,
    pub(super) path: IntPath,
    pub(super) is_rect: bool
}

pub(super) struct Bundle {
    pub(super) rect: IntRect,
    pub(super) pieces: Vec<Piece>,
}

pub(super) struct StackLayer {
    pub(super) bundles: Vec<Bundle>,
}

impl StackLayer {
    pub fn new(shapes: IntShapes) -> Self {
        let mut bundles = Vec::with_capacity(shapes.len());

        for (_, shape) in shapes.iter().enumerate() {
            let shape_rect = IntRect::with_shape(shape);
            let slices = shape.vertical_slice(SliceType::Original);
            let mut pieces = Vec::with_capacity(slices.len());
            for slice in slices.into_iter() {
                let rect: IntRect = Rect::with_path(&slice);

                let area = slice.unsafe_area();
                debug_assert!(area > 0);
                let area = area >> 1;

                let is_rect = area == rect.area();

                pieces.push(Piece { rect, path: slice, is_rect });
            }
            pieces.sort_unstable_by(|a, b|a.rect.min_x.cmp(&b.rect.min_x));

            bundles.push(Bundle {
                rect: shape_rect,
                pieces,
            });
        }

        bundles.sort_unstable_by(|a, b|a.rect.min_x.cmp(&b.rect.min_x));

        Self { bundles }
    }
}