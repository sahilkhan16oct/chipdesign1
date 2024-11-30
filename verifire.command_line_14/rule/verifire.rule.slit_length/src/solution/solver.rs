use foundation::i_overlay::i_float::point::IntPoint;
use foundation::res::shape_res::ShapeRes;
use min_max_width::solution::solver::{MinMaxAlgorithm, MinMaxSolver};
use crate::solution::inverter::Inverter;

pub struct SlitLengthSolver {
    pub(super) shapes_res: ShapeRes,
    pub(super) algorithm: MinMaxAlgorithm,
    pub(super) value: i32,
}

impl SlitLengthSolver {
    pub fn new(shapes_res: ShapeRes, algorithm: MinMaxAlgorithm, value: i32) -> Self {
        Self { shapes_res, algorithm, value }
    }

    pub fn find_problems(&self) -> Vec<[IntPoint; 2]> {
        let original = self.shapes_res.shapes();
        let shapes = Inverter::invert(original);
        MinMaxSolver::solve(shapes, self.algorithm, self.value)
    }
}