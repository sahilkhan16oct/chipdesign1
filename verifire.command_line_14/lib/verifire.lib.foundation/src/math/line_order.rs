use std::cmp::Ordering;
use crate::i_overlay::i_float::point::IntPoint;

pub trait LineOrder {
    fn order_by_line(self, b: Self) -> Ordering;
    fn order_by_line_compare(self, other: Self) -> bool;
}

impl LineOrder for IntPoint {

    fn order_by_line(self, other: Self) -> Ordering {
        if self.order_by_line_compare(other) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }

    fn order_by_line_compare(self, other: Self) -> bool {
        self.x < other.x || self.x == other.x && self.y < other.y
    }
}
