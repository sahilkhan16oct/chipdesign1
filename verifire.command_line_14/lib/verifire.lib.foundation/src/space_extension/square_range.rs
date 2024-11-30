use crate::i_overlay::i_float::fix_vec::FixVec;
use crate::i_overlay::i_float::point::IntPoint;
use crate::i_overlay::line_range::LineRange;

#[derive(Debug, Clone)]
pub struct SquareRange {
    pub x_min: i32,
    pub y_min: i32,
    pub x_max: i32,
    pub y_max: i32,
}

impl SquareRange {
    pub fn new(x_min: i32, y_min: i32, x_max: i32, y_max: i32) -> Self {
        SquareRange { x_min, y_min, x_max, y_max }
    }

    pub fn from_points(a: IntPoint, b: IntPoint) -> Self {
        let (x_min, x_max) = if a.x < b.x {
            (a.x, b.x)
        } else {
            (b.x, a.x)
        };

        let (y_min, y_max) = if a.y < b.y {
            (a.y, b.y)
        } else {
            (b.y, a.y)
        };

        SquareRange::new(x_min, y_min, x_max, y_max)
    }

    pub fn x_range(&self) -> LineRange {
        LineRange { min: self.x_min, max: self.x_max }
    }

    pub fn y_range(&self) -> LineRange {
        LineRange { min: self.y_min, max: self.y_max }
    }

    pub fn width(&self) -> i32 {
        self.x_max - self.x_min
    }

    pub fn height(&self) -> i32 {
        self.y_max - self.y_min
    }

    pub fn area(&self) -> i64 {
        (self.width() as i64) * (self.height() as i64)
    }

    pub fn path(&self) -> [IntPoint; 4] {
        let p0 = IntPoint::new(self.x_min, self.y_min);
        let p1 = IntPoint::new(self.x_min, self.y_max);
        let p2 = IntPoint::new(self.x_max, self.y_max);
        let p3 = IntPoint::new(self.x_max, self.y_min);
        [p0, p1, p2, p3]
    }

    pub fn delta_and_clamp(&self, delta: i32, y_clamp: LineRange) -> SquareRange {
        SquareRange::new(
            self.x_min - delta,
            (self.y_min - delta).clamp(y_clamp.min, y_clamp.max),
            self.x_max + delta,
            (self.y_max + delta).clamp(y_clamp.min, y_clamp.max),
        )
    }

    pub fn delta(&self, delta: i32) -> SquareRange {
        SquareRange::new(
            self.x_min - delta,
            self.y_min - delta,
            self.x_max + delta,
            self.y_max + delta,
        )
    }

    pub fn is_overlap(&self, other: &SquareRange) -> bool {
        self.x_min <= other.x_max && self.x_max >= other.x_min && self.y_min <= other.y_max && self.y_max >= other.y_min
    }

    pub fn is_contain(&self, point: FixVec) -> bool {
        let x = point.x as i32;
        let y = point.y as i32;

        self.x_min < x && x < self.x_max && self.y_min < y && y < self.y_max
    }
}
