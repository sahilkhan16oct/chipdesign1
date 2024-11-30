use design_format::i_overlay::core::fill_rule::FillRule;
use design_format::i_overlay::core::overlay_rule::OverlayRule;
use design_format::i_overlay::core::overlay::Overlay;
use design_format::i_overlay::vector::vector::VectorShape;
use crate::i_overlay::i_shape::int::shape::{IntShape, IntShapes};

pub(super) struct Solver;

impl Solver {
    pub(super) fn select(target: &IntShapes, area: &IntShapes, probe: u8, overlap: bool, full_include: bool) -> IntShapes {
        let overlay = Overlay::with_shapes(target, area);
        let v_shapes = overlay.into_graph(FillRule::NonZero).extract_vectors(OverlayRule::Subject);

        let mut result = Vec::with_capacity(target.len());

        for v_shape in v_shapes.iter() {
            let v_path = &v_shape[0];

            let mut is_overlap = full_include;

            for v in v_path.iter() {
                let inner_clip = v.fill & probe != 0;

                // For full_include, we look for any case where inner_clip is false to set is_overlap to false.
                // For !full_include, we look for any case where inner_clip is true to set is_overlap to true.
                if inner_clip != full_include {
                    is_overlap = !full_include;
                    break;
                }
            }

            // After checking all vertices in v_path, add the shape to results if it matches the overlap condition.
            if is_overlap == overlap {
                let shape = Self::fix_shape(v_shape);
                result.push(shape);
            }
        }

        result
    }

    fn fix_shape(shape: &VectorShape) -> IntShape {
        shape.iter().map(|path| path.iter().map(|e| e.a).collect()).collect()
    }
}