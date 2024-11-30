use foundation::i_overlay::i_float::fix_vec::FixVec;
use foundation::i_overlay::i_float::point::IntPoint;
use foundation::space_extension::square_range::SquareRange;
use foundation::space_extension::square_vector::SquareVector;
use foundation::task::direction::SpaceDirection;


#[derive(Debug, Clone)]
pub(super) struct Dist {
    range: SquareRange,
    pub(super) a: FixVec,
    pub(super) b: FixVec,
}

impl Dist {
    pub(super) fn new(a: FixVec, b: FixVec) -> Self {
        Self { range: SquareRange::from_points(IntPoint::new(a.x as i32, a.y as i32), IntPoint::new(b.x as i32, b.y as i32)), a, b }
    }

    pub(super) fn validate(&self, direction: SpaceDirection) -> bool {
        match direction {
            SpaceDirection::X => { self.a.y == self.b.y }
            SpaceDirection::Y => { self.a.x == self.b.x }
            SpaceDirection::XY => { self.a.x == self.b.x || self.a.y == self.b.y }
            SpaceDirection::DXY => { true }
        }
    }

    pub(super) fn is_cross(&self, vector: &SquareVector) -> bool {
        if !vector.range.is_overlap(&self.range) {
            return false;
        }

        if vector.a == self.a || vector.a == self.b || vector.b == self.a || vector.b == self.b {
            return false;
        }

        let is_div0 = Dist::is_divide(self.a, self.b, vector.a, vector.b);
        let is_div1 = Dist::is_divide(vector.a, vector.b, self.a, self.b);

        is_div0 && is_div1
    }

    fn is_divide(v0: FixVec, v1: FixVec, e0: FixVec, e1: FixVec) -> bool {
        let v = v0 - v1;
        let a = v.cross_product(v0 - e0);
        let b = v.cross_product(v0 - e1);

        !((a < 0 && b < 0) || (a > 0 && b > 0))
    }
}