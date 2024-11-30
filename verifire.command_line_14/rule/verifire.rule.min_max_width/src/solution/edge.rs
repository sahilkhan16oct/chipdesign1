use foundation::i_overlay::i_float::bit_pack::BitPackVec;
use foundation::i_overlay::i_float::point::IntPoint;
use foundation::i_overlay::i_shape::int::path::IntPath;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge {
    // start < end
    pub(super) a: IntPoint,
    // start
    pub(super) b: IntPoint,        // end
}

impl Edge {
    pub(super) fn is_vertical(&self) -> bool { self.a.x == self.b.x }

    pub fn new(a: IntPoint, b: IntPoint) -> Self {
        if a.bit_pack() <= b.bit_pack() {
            Self { a, b }
        } else {
            Self { a: b, b: a }
        }
    }

    pub fn with_array(array: [i32; 4]) -> Self {
        let a = IntPoint::new(array[0], array[1]);
        let b = IntPoint::new(array[2], array[3]);

        if a.bit_pack() <= b.bit_pack() {
            Self { a, b }
        } else {
            Self { a: b, b: a }
        }
    }

    pub(super) fn vertical_intersection(&self, x: i32) -> i32 {
        if self.a.y == self.b.y {
            return self.a.y;
        }

        if self.a.x == x {
            return self.a.y;
        }

        if self.b.x == x {
            return self.b.y;
        }

        let v = self.a.subtract(self.b);
        let x0 = (x - self.a.x) as i64;
        let dx = v.x;
        let dy = v.y;

        return ((dy * x0) / dx) as i32 + self.a.y;
    }

    pub(super) fn invert(&mut self) {
        self.a = self.a.invert();
        self.b = self.b.invert();
    }
}

pub(super) trait CreatEdges {
    fn create_edges(&self, invert: bool) -> Vec<Edge>;
}

impl CreatEdges for IntPath {
    fn create_edges(&self, invert: bool) -> Vec<Edge> {
        let n = self.len();
        if n < 2 {
            return Vec::new();
        }

        let mut edges = Vec::with_capacity(n);

        let i0 = n - 1;
        let mut a = self[i0];

        for &b in self.iter() {
            if invert {
                edges.push(Edge::new(a.invert(), b.invert()));
            } else {
                edges.push(Edge::new(a, b));
            }
            a = b;
        }

        edges
    }
}

pub(super) trait Invert {
    fn invert(&self) -> IntPoint;
}

impl Invert for IntPoint {
    fn invert(&self) -> IntPoint {
        IntPoint { x: self.y, y: self.x }
    }
}