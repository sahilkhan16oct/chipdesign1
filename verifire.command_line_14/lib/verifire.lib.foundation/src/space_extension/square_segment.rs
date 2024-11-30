use crate::space_extension::square_range::SquareRange;

#[derive(Debug, Clone)]
pub struct SquareSegment {
    pub id: usize,
    pub range: SquareRange
}