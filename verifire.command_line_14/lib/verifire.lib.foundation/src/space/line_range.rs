use crate::i_overlay::line_range::LineRange;

pub trait LineRangeExt {
    fn is_overlap(&self, other: &LineRange) -> bool;
    fn clamp(&self, range: &LineRange) -> LineRange;
}

impl LineRangeExt for LineRange {
    fn is_overlap(&self, other: &LineRange) -> bool {
        self.min <= other.max && self.max >= other.min
    }

    fn clamp(&self, range: &LineRange) -> LineRange {
        let min = self.min.clamp(range.min, range.max);
        let max = self.max.clamp(range.min, range.max);
        Self { min, max }
    }
}