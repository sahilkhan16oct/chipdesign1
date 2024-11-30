use design_format::i_overlay::vector::vector::CLIP_RIGHT;
use crate::i_overlay::core::fill_rule::FillRule;
use crate::i_overlay::core::overlay::Overlay;
use crate::i_overlay::i_shape::int::shape::IntShapes;
use crate::res::filter::{AreaRes, FilterRes};
use crate::res::solver::Solver;

pub struct ShapeRes {
    pub base: IntShapes,
    pub filter: Option<FilterRes>
}

impl ShapeRes {

    pub fn new(base: IntShapes) -> Self {
        Self { base, filter: None }
    }

    pub fn shapes(&self) -> IntShapes {
        let filter = match &self.filter {
            None => {
                return self.base.clone();
            }
            Some(filter) => {
                filter
            }
        };

        let mut shapes = self.base.clone();

        for area in filter.areas.iter() {
            match area {
                AreaRes::Boolean(res) => {
                    let overlay = Overlay::with_shapes(&shapes, &res.shapes);
                    shapes = overlay.into_graph(FillRule::NonZero).extract_shapes(res.operation);
                }
                AreaRes::Select(res) => {
                    shapes = Solver::select(&shapes, &res.shapes, CLIP_RIGHT, !res.inverted, res.full_include);
                }
            }
        }

        shapes
    }
}