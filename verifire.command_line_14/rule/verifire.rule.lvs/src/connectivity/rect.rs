use foundation::i_overlay::i_float::rect::IntRect;
use foundation::i_overlay::i_shape::int::path::IntPath;
use foundation::i_overlay::i_shape::int::shape::{IntShape, IntShapes};

pub(crate) trait Rect {
    fn area(&self) -> i64;
    fn new_empty() -> Self;
    #[cfg(debug_assertions)]
    fn collides_x(&self, other: &Self) -> bool;
    fn collides_y(&self, other: &Self) -> bool;
    fn with_path(path: &IntPath) -> Self;
    fn with_shape(shape: &IntShape) -> Self;
    fn with_shapes(shapes: &IntShapes) -> Self;
}

impl Rect for IntRect {
    #[inline(always)]
    fn area(&self) -> i64 {
        let a = self.max_x as i64 - self.min_x as i64;
        let b = self.max_y as i64 - self.min_y as i64;
        a * b
    }

    #[inline(always)]
    fn new_empty() -> Self {
        Self {
            min_x: i32::MAX,
            max_x: -i32::MAX,
            min_y: i32::MAX,
            max_y: -i32::MAX,
        }
    }

    #[cfg(debug_assertions)]
    fn collides_x(&self, other: &Self) -> bool {
        self.min_x <= other.max_x && self.max_x >= other.min_x
    }

    #[inline(always)]
    fn collides_y(&self, other: &Self) -> bool {
        self.min_y <= other.max_y && self.max_y >= other.min_y
    }

    fn with_path(path: &IntPath) -> Self {
        let mut rect = Self::new_empty();

        for p in path.iter() {
            rect.add_point(p);
        }

        rect
    }

    fn with_shape(shape: &IntShape) -> Self {
        let mut rect = Self::new_empty();

        for path in shape.iter() {
            for p in path.iter() {
                rect.add_point(p);
            }
        }

        rect
    }

    fn with_shapes(shapes: &IntShapes) -> Self {
        let mut rect = Self::new_empty();

        for shape in shapes {
            for path in shape.iter() {
                for p in path.iter() {
                    rect.add_point(p);
                }
            }
        }

        rect
    }
}