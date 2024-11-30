use foundation::i_overlay::core::fill_rule::FillRule;
use foundation::i_overlay::core::overlay_rule::OverlayRule;
use foundation::i_overlay::core::overlay::Overlay;
use foundation::i_overlay::core::solver::Solver;
use foundation::i_overlay::i_float::point::IntPoint;
use foundation::i_overlay::i_shape::int::path::IntPath;
use foundation::i_overlay::i_shape::int::shape::{IntShape, IntShapes};
use foundation::i_overlay::vector::vector::{CLIP_LEFT, CLIP_RIGHT, VectorEdge};
use foundation::res::shape_res::ShapeRes;
use foundation::task::direction::SpaceDirection;
use spacing::solution::ab_space::ABSpace;
use spacing::solution::context::SpaceContext;

pub struct EnclosurePolySolver {
    float_res: ShapeRes,
    poly_res: ShapeRes,
    value: i32,
    direction: SpaceDirection,
}

impl EnclosurePolySolver {
    pub fn new(float_res: ShapeRes, poly_res: ShapeRes, value: i32, direction: SpaceDirection) -> Self {
        Self { float_res, poly_res, value, direction }
    }

    pub fn find_problems(&self) -> Vec<[IntPoint; 2]> {
        let poly_shapes = self.float_res.shapes();
        let float_shapes = self.poly_res.shapes();

        let i_edges = Self::intersection_test(&poly_shapes, &float_shapes);
        if !i_edges.is_empty() || self.value == 0 {
            return if self.value > 0 {
                i_edges.iter().map(|e| [e.a, e.b]).collect()
            } else {
                i_edges.iter().filter(|e| e.fill & CLIP_RIGHT == 0).map(|e| [e.a, e.b]).collect()
            };
        }

        let shapes_a = poly_shapes;
        let mut shapes_b = float_shapes;
        shapes_b.invert();

        let space = ABSpace { shapes_a, shapes_b };
        SpaceContext::new(space, self.value as i64, self.direction).find_problems()
    }

    fn intersection_test(poly_shapes: &IntShapes, float_shapes: &IntShapes) -> Vec<VectorEdge> {
        let overlay = Overlay::with_shapes(poly_shapes, float_shapes);
        let v_shapes = overlay.into_vectors(FillRule::NonZero, OverlayRule::Subject, Solver::AUTO);

        let mut result = Vec::new();
        for v_shape in v_shapes.iter() {
            for v_path in v_shape.iter() {
                for v in v_path.iter() {
                    if v.fill & CLIP_LEFT == 0 {
                        result.push(v.clone());
                    }
                }
            }
        }

        result
    }
}

trait Invert {
    fn invert(&mut self);
}

impl Invert for IntShapes {
    fn invert(&mut self) {
        for i in 0..self.len() {
            self[i].invert()
        }
    }
}

impl Invert for IntShape {
    fn invert(&mut self) {
        for i in 0..self.len() {
            self[i].invert()
        }
    }
}

impl Invert for IntPath {
    fn invert(&mut self) {
        self.reverse();
    }
}