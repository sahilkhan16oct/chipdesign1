use foundation::i_overlay::i_float::fix_vec::FixVec;
use foundation::i_overlay::i_shape::int::shape::IntShape;
use foundation::space_extension::builder::SquareVectorBuilder;
use foundation::space_extension::corner::Corner;
use foundation::space_extension::square_index::{ShapeLayer, SquareIndex};
use foundation::space_extension::square_vector::SquareVector;
use crate::solution::space::Space;

pub struct ABSpace {
    pub shapes_a: Vec<IntShape>,
    pub shapes_b: Vec<IntShape>,
}

impl Space for ABSpace {
    fn vectors(&self) -> Vec<SquareVector> {
        let mut vectors_a = self.shapes_a.build_square_vectors(ShapeLayer::A, 0);
        let last_id = vectors_a.last().map_or(0, |v| v.id + 1);
        let mut vectors_b = self.shapes_b.build_square_vectors(ShapeLayer::B, last_id);

        let mut vectors = Vec::with_capacity(vectors_a.len() + vectors_b.len());
        vectors.append(&mut vectors_a);
        vectors.append(&mut vectors_b);

        vectors
    }

    fn path_test(&self, index: &SquareIndex, p: FixVec) -> bool {
        match index.layer() {
            ShapeLayer::A => {
                self.shapes_a[index.shape_index()].path_test(&index, p)
            }
            ShapeLayer::B => {
                self.shapes_b[index.shape_index()].path_test(&index, p)
            }
        }
    }

    fn is_contain_clock_wise(&self, index: &SquareIndex, p: FixVec) -> bool {
        match index.layer() {
            ShapeLayer::A => {
                self.shapes_a[index.shape_index()].is_contain_clock_wise(&index, p)
            }
            ShapeLayer::B => {
                self.shapes_b[index.shape_index()].is_contain_clock_wise(&index, p)
            }
        }
    }

    fn are_colliding(&self, i: &SquareIndex, j: &SquareIndex) -> bool {
        i.layer() != j.layer()
    }
}