use std::cmp::Ordering;
use foundation::i_overlay::i_float::point::IntPoint;
use foundation::i_overlay::i_float::triangle::Triangle;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct ScanSegment {
    pub(super) a: IntPoint,
    pub(super) b: IntPoint,
}

impl ScanSegment {
    pub(super) fn is_under_point_or_equal(&self, p: IntPoint) -> bool {
        debug_assert!(self.a.x <= p.x && p.x <= self.b.x);
        Triangle::area_two_point(self.a, p, self.b) >= 0
    }

    pub(super) fn is_under_segment(&self, other: &Self) -> bool {
        if self.a == other.a {
            Triangle::is_clockwise_point(self.a, other.b, self.b)
        } else if self.a.x < other.a.x {
            Triangle::is_clockwise_point(self.a, other.a, self.b)
        } else {
            Triangle::is_clockwise_point(other.a, other.b, self.a)
        }
    }
}

impl PartialOrd for ScanSegment {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScanSegment {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_under_segment(other) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }
}