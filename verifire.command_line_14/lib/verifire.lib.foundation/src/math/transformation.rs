use crate::i_overlay::i_float::point::IntPoint;

#[derive(Debug, Clone, Copy)]
pub enum Transformation {
    None,
    ReflectY,
    ReflectX,
    RotateCW90,
    RotateCW180,
    RotateCW270,
    RotateCCW90,
    RotateCCW180,
    RotateCCW270,
}

pub trait Rotate {
    fn rotate(&self, transformation: Transformation) -> Self;
    fn apply_transformation(&mut self, transformation: Transformation);
}

impl Rotate for IntPoint {
    fn rotate(&self, transformation: Transformation) -> Self {
        let mut point = self.clone();
        point.apply_transformation(transformation);
        point
    }

    fn apply_transformation(&mut self, transformation: Transformation) {
        match transformation {
            Transformation::RotateCW90 | Transformation::RotateCCW270 => {
                let x = self.x;
                self.x = self.y;
                self.y = -x;
            }
            Transformation::RotateCW180 | Transformation::RotateCCW180 => {
                self.x = -self.x;
                self.y = -self.y;
            }
            Transformation::RotateCW270 | Transformation::RotateCCW90 => {
                let x = self.x;
                self.x = -self.y;
                self.y = x;
            }
            Transformation::ReflectY => {
                self.y = -self.y;
            }
            Transformation::ReflectX => {
                self.x = -self.x;
            }
            _ => {}
        }
    }
}