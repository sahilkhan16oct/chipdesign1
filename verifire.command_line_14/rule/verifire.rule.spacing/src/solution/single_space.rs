use foundation::i_overlay::i_float::fix_vec::FixVec;
use foundation::i_overlay::i_shape::int::shape::IntShape;
use foundation::space_extension::builder::SquareVectorBuilder;
use foundation::space_extension::corner::Corner;
use foundation::space_extension::square_index::{ShapeLayer, SquareIndex};
use foundation::space_extension::square_vector::SquareVector;
use crate::solution::space::Space;

pub struct SingleSpace {
    pub shapes: Vec<IntShape>,
}

impl Space for SingleSpace {
    fn vectors(&self) -> Vec<SquareVector> {
        self.shapes.build_square_vectors(ShapeLayer::A, 0)
    }

    fn path_test(&self, index: &SquareIndex, p: FixVec) -> bool {
        self.shapes[index.shape_index()].path_test(&index, p)
    }

    fn is_contain_clock_wise(&self, index: &SquareIndex, p: FixVec) -> bool {
        self.shapes[index.shape_index()].is_contain_clock_wise(&index, p)
    }

    fn are_colliding(&self, _i: &SquareIndex, _j: &SquareIndex) -> bool {
        true
    }
}