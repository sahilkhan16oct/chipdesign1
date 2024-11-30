#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShapeLayer {
    A,
    B,
}

#[derive(Debug, Clone)]
pub struct SquareIndex {
    pub shape_id: u32,
    pub path_index: u32,
    pub vert_index: u32,
    pub last_index: u32,
}

impl SquareIndex {
    pub fn new(layer: ShapeLayer, shape_index: usize, path_index: usize, vert_index: usize, last_index: usize) -> Self {
        let shape_id = match layer {
            ShapeLayer::A => shape_index as u32,
            ShapeLayer::B => (shape_index as u32).set_last_bit(),
        };

        Self {
            shape_id,
            path_index: path_index as u32,
            vert_index: vert_index as u32,
            last_index: last_index as u32,
        }
    }

    pub fn layer(&self) -> ShapeLayer {
        if self.shape_id.is_last_bit() {
            ShapeLayer::B
        } else {
            ShapeLayer::A
        }
    }

    pub fn shape_index(&self) -> usize {
        self.shape_id.clear_last_bit() as usize
    }

    pub fn is_first(&self) -> bool {
        self.vert_index == 0
    }

    pub fn is_last(&self) -> bool {
        self.vert_index == self.last_index
    }

    pub fn next_index(&self) -> SquareIndex {
        if self.is_last() {
            SquareIndex { shape_id: self.shape_id, path_index: self.path_index, vert_index: 0, last_index: self.last_index }
        } else {
            SquareIndex { shape_id: self.shape_id, path_index: self.path_index, vert_index: self.vert_index + 1, last_index: self.last_index }
        }
    }
}

trait UInt32Extensions {
    fn set_last_bit(self) -> u32;
    fn clear_last_bit(self) -> u32;
    fn is_last_bit(self) -> bool;
}

const LAST_BIT: u32 = 1 << (32 - 1);

impl UInt32Extensions for u32 {
    fn set_last_bit(self) -> u32 {
        LAST_BIT | self
    }

    fn clear_last_bit(self) -> u32 {
        let mask = !LAST_BIT;
        mask & self
    }

    fn is_last_bit(self) -> bool {
        LAST_BIT & self == LAST_BIT
    }
}
