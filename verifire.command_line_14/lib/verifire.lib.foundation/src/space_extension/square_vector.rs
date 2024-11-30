use crate::i_overlay::i_float::f64_point::F64Point;
use crate::i_overlay::i_float::fix_vec::FixVec;
use crate::i_overlay::i_float::point::IntPoint;
use crate::space_extension::square_index::SquareIndex;
use crate::space_extension::square_range::SquareRange;

#[derive(Debug, Clone)]
pub struct SquareVector {
    pub id: usize,
    pub a: FixVec,
    pub b: FixVec,
    pub range: SquareRange,
    pub index: SquareIndex,
}

impl SquareVector {
    pub fn vector(&self) -> FixVec {
        self.b - self.a
    }

    pub fn normalize(&self) -> F64Point {
        let v = self.b - self.a;
        v.like_f64vec().normalize()
    }

    pub fn normalize_for_diagonal(&self) -> FixVec {
        let v = self.vector();
        let x = if v.x < 0 { -1 } else { 1 };
        let y = if v.y < 0 { -1 } else { 1 };
        FixVec::new(x, y)
    }

    pub fn new(id: usize, a: IntPoint, b: IntPoint, index: SquareIndex) -> Self {
        let range = SquareRange::from_points(a, b);
        Self { id, a: FixVec::new_point(a), b: FixVec::new_point(b), range, index }
    }
}