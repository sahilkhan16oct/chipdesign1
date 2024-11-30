use crate::i_overlay::i_float::fix_vec::FixVec;
use crate::i_overlay::i_float::point::IntPoint;
use crate::i_overlay::i_shape::int::shape::IntShape;
use crate::space_extension::square_index::SquareIndex;

pub trait Corner {
    fn is_contain_counter_clock_wise(&self, index: &SquareIndex, p: FixVec) -> bool;

    fn is_contain_clock_wise(&self, index: &SquareIndex, p: FixVec) -> bool;

    fn path_test(&self, index: &SquareIndex, p: FixVec) -> bool;
}

impl Corner for [IntShape] {
    fn is_contain_counter_clock_wise(&self, index: &SquareIndex, p: FixVec) -> bool {
        self[index.shape_index()].is_contain_counter_clock_wise(index, p)
    }

    fn is_contain_clock_wise(&self, index: &SquareIndex, p: FixVec) -> bool {
        self[index.shape_index()].is_contain_clock_wise(index, p)
    }

    fn path_test(&self, index: &SquareIndex, p: FixVec) -> bool {
        self[index.shape_index()].path_test(index, p)
    }
}

impl Corner for IntShape {
    fn is_contain_counter_clock_wise(&self, index: &SquareIndex, p: FixVec) -> bool {
        let points = direct_order(self, index);
        is_contain_counter_clock_wise(points.0, points.1, points.2, p)
    }

    fn is_contain_clock_wise(&self, index: &SquareIndex, p: FixVec) -> bool {
        let points = direct_order(self, index);
        is_contain_counter_clock_wise(points.2, points.1, points.0, p)
    }

    fn path_test(&self, index: &SquareIndex, p: FixVec) -> bool {
        let points = direct_order(self, index);

        let b = points.1;
        let m = b + b - p;

        let is_contain_p = is_contain_counter_clock_wise(points.2, points.1, points.0, p);
        let is_contain_m = is_contain_counter_clock_wise(points.2, points.1, points.0, m);

        is_contain_p && !is_contain_m
    }
}

fn direct_order(shape: &IntShape, index: &SquareIndex) -> (FixVec, FixVec, FixVec) {
    let path = &shape[index.path_index as usize];
    let vert_index = index.vert_index as usize;
    let b = path[vert_index];
    let a: IntPoint;
    let c: IntPoint;
    if index.is_last() {
        a = path[vert_index - 1];
        c = path[0];
    } else if index.is_first() {
        a = path[index.last_index as usize];
        c = path[vert_index + 1];
    } else {
        a = path[vert_index - 1];
        c = path[vert_index + 1];
    }

    (FixVec::new_point(a), FixVec::new_point(b), FixVec::new_point(c))
}

fn is_contain_counter_clock_wise(a: FixVec, b: FixVec, c: FixVec, p: FixVec) -> bool {
    let ab = b - a;
    let bc = c - b;

    let cross = ab.cross_product(bc);
    let q = p - b;

    let sa = ab.cross_product(q);
    let sb = bc.cross_product(q);

    if cross > 0 {
        sa < 0 || sb < 0
    } else {
        sa < 0 && sb < 0 || sa < 0 && sb < 0
    }
}