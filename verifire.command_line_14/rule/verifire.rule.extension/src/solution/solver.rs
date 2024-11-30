use foundation::i_overlay::core::fill_rule::FillRule;
use foundation::i_overlay::core::overlay_rule::OverlayRule;
use foundation::i_overlay::core::overlay::Overlay;
use foundation::i_overlay::core::solver::Solver;
use foundation::i_overlay::i_float::point::IntPoint;
use foundation::i_overlay::i_shape::int::shape::{IntShapes, PointsCount};
use foundation::i_overlay::vector::vector::{CLIP_LEFT, CLIP_RIGHT, SUBJ_RIGHT, VectorShape};
use foundation::math::transformation::{Rotate, Transformation};
use foundation::res::shape_res::ShapeRes;
use crate::solution::dist_solver::DistSolver;
use crate::solution::segment::ScanSegment;

pub struct ExtensionSolver {
    poly_res: ShapeRes,
    diffusion_res: ShapeRes,
    value: i32,
}

const CLIP_OUT: u8 = SUBJ_RIGHT | CLIP_LEFT | CLIP_RIGHT;

impl ExtensionSolver {
    pub fn new(poly_res: ShapeRes, diffusion_res: ShapeRes, value: i32) -> Self {
        Self { poly_res, diffusion_res, value }
    }

    pub fn find_problems(&self) -> Vec<[IntPoint; 2]> {
        let poly = self.poly_res.shapes();
        let diff = self.diffusion_res.shapes();

        let overlay = Overlay::with_shapes(&poly, &diff);
        let gate_shapes = overlay.into_vectors(FillRule::NonZero, OverlayRule::Intersect, Solver::AUTO);
        let gate_segments = Self::side_segments(&gate_shapes);
        let diff_segments = Self::diff_segments(&diff);

        let mut result = Vec::new();

        // right
        let mut right = DistSolver::test_vertical_polygon_enclosure(
            Self::segments(&diff_segments, Transformation::None),
            Self::segments(&gate_segments, Transformation::None),
            self.value,
        );
        result.append(&mut right);

        // left
        let mut left = DistSolver::test_vertical_polygon_enclosure(
            Self::segments(&diff_segments, Transformation::RotateCW180),
            Self::segments(&gate_segments, Transformation::RotateCW180),
            self.value,
        );
        Self::rotate_edges(&mut left, Transformation::RotateCCW180);
        result.append(&mut left);

        // top
        let mut top = DistSolver::test_vertical_polygon_enclosure(
            Self::segments(&diff_segments, Transformation::RotateCW90),
            Self::segments(&gate_segments, Transformation::RotateCW90),
            self.value,
        );
        Self::rotate_edges(&mut top, Transformation::RotateCCW90);
        result.append(&mut top);

        let mut bottom = DistSolver::test_vertical_polygon_enclosure(
            Self::segments(&diff_segments, Transformation::RotateCCW90),
            Self::segments(&gate_segments, Transformation::RotateCCW90),
            self.value,
        );
        Self::rotate_edges(&mut bottom, Transformation::RotateCW90);
        result.append(&mut bottom);

        result
    }

    fn side_segments(shapes: &Vec<VectorShape>) -> Vec<ScanSegment> {
        let mut result = Vec::with_capacity(4 * shapes.len());

        for shape in shapes.iter() {
            let contour = &shape[0];
            for edge in contour.iter() {
                let clip_test = edge.fill == CLIP_OUT;
                if clip_test {
                    result.push(ScanSegment { a: edge.a, b: edge.b });
                }
            }
        }

        result
    }

    fn diff_segments(shapes: &IntShapes) -> Vec<ScanSegment> {
        let mut segments = Vec::with_capacity(shapes.points_count());
        for shape in shapes.iter() {
            for path in shape.iter() {
                let mut a = path[path.len() - 1];
                for &p in path.iter() {
                    let b = p;
                    segments.push(ScanSegment { a, b });
                    a = b;
                }
            }
        }
        segments
    }

    fn segments(segments: &Vec<ScanSegment>, transformation: Transformation) -> Vec<ScanSegment> {
        let mut result = Vec::with_capacity(segments.len() / 4);
        for si in segments.iter() {
            let s = si.rotate(transformation);
            if s.a.x < s.b.x {
                result.push(s.clone());
            }
        }
        result
    }

    fn rotate_edges(edges: &mut Vec<[IntPoint; 2]>, transformation: Transformation) {
        for e in edges.iter_mut() {
            e[0].apply_transformation(transformation);
            e[1].apply_transformation(transformation);
        }
    }
}

impl ScanSegment {
    fn rotate(&self, rotation: Transformation) -> ScanSegment {
        ScanSegment { a: self.a.rotate(rotation), b: self.b.rotate(rotation) }
    }
}