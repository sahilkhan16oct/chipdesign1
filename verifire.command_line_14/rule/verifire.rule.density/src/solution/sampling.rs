use foundation::i_overlay::i_float::point::IntPoint;
use foundation::i_overlay::i_shape::int::shape::IntShapes;

pub(super) struct Metric {
    pub(super) log2: usize,
    pub(super) count: usize,
}

impl Metric {
    pub(super) fn step(&self) -> i32 {
        if self.log2 <= 0 {
            1
        } else {
            1 << self.log2
        }
    }

    pub(super) fn offset(&self) -> i32 {
        if self.log2 <= 0 {
            0
        } else {
            1 << (self.log2 - 1)
        }
    }
}

impl Metric {
    fn new(min: i32, max: i32, discrete_count: usize) -> Self {
        let max_len = 1 << discrete_count;

        let log2: usize;
        let count: usize;

        let len = (max - min) as u32;

        if len <= max_len {
            log2 = 0;
            count = len as usize;
        } else {
            let n = len.max_bit();
            log2 = n - discrete_count;
            count = (len >> log2) as usize;
        }

        Self {
            log2,
            count,
        }
    }
}

pub(super) struct Sampling {
    pub(super) origin: IntPoint,
    pub(super) x_metric: Metric,
    pub(super) y_metric: Metric,
}

impl Sampling {
    pub(super) fn new(shapes: &IntShapes, discrete_count: usize) -> Self {
        let mut x_min = i32::MAX;
        let mut x_max = i32::MIN;
        let mut y_min = i32::MAX;
        let mut y_max = i32::MIN;

        for shape in shapes.iter() {
            for path in shape.iter() {
                for &p in path.iter() {
                    x_min = x_min.min(p.x);
                    x_max = x_max.max(p.x);
                    y_min = y_min.min(p.y);
                    y_max = y_max.max(p.y);
                }
            }
        }

        Self {
            origin: IntPoint { x: x_min, y: y_min },
            x_metric: Metric::new(x_min, x_max, discrete_count),
            y_metric: Metric::new(y_min, y_max, discrete_count),
        }
    }
}

trait BitMath {
    fn max_bit(self) -> usize;
}

impl BitMath for u32 {
    fn max_bit(self) -> usize {
        let z_count = self.leading_zeros();
        (32 - z_count) as usize
    }
}