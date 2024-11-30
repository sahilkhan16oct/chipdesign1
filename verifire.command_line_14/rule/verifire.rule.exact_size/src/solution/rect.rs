use foundation::i_overlay::i_shape::int::path::PointPathExtension;
use foundation::i_overlay::i_shape::int::shape::IntShape;

pub(super) struct Size {
    pub(super) width: i32,
    pub(super) height: i32,
}

pub(super) trait RectShape {
    fn size(&self) -> Option<Size>;
}

impl RectShape for IntShape {
    fn size(&self) -> Option<Size> {
        if self.len() > 1 || self[0].len() != 4 {
            return None;
        }

        let mut min_x = i32::MAX;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut max_y = i32::MIN;

        for p in self[0].iter() {
            min_x = min_x.min(p.x);
            max_x = max_x.max(p.x);
            min_y = min_y.min(p.y);
            max_y = max_y.max(p.y);
        }

        let width = max_x - min_x;
        let height = max_y - min_y;

        let s0 = width as i64 * height as i64;
        let s1 = self[0].unsafe_area() >> 1;

        if s0 == s1 {
            Some(Size { width, height })
        } else {
            None
        }
    }
}