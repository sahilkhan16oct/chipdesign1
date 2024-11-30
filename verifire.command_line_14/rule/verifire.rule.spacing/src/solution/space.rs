use foundation::i_overlay::i_float::fix_vec::FixVec;
use foundation::space_extension::square_index::SquareIndex;
use foundation::space_extension::square_vector::SquareVector;

pub trait Space {
    fn vectors(&self) -> Vec<SquareVector>;
    fn path_test(&self, index: &SquareIndex, p: FixVec) -> bool;
    fn is_contain_clock_wise(&self, index: &SquareIndex, p: FixVec) -> bool;
    fn are_colliding(&self, i: &SquareIndex, j: &SquareIndex) -> bool;
}