use crate::i_overlay::line_range::LineRange;

#[derive(Copy, Clone)]
pub struct LineSegment<Id: Copy> {
    pub id: Id,
    pub range: LineRange
}