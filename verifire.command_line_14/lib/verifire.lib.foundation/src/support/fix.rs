use crate::i_overlay::i_float::fix_vec::FixVec;
use crate::i_overlay::i_shape::fix::path::FixPath;
use crate::i_overlay::i_shape::fix::shape::FixShape;
use crate::i_overlay::i_shape::fix::shapes::FixShapes;
use crate::i_overlay::i_shape::int::path::IntPath;
use crate::i_overlay::i_shape::int::shape::{IntShape, IntShapes};

pub trait ToFixPath {
    fn to_fix(&self) -> FixPath;
}

pub trait ToFixShape {
    fn to_fix(&self) -> FixShape;
}

pub trait ToFixShapes {
    fn to_fix(&self) -> FixShapes;
}

impl ToFixPath for IntPath {
    fn to_fix(&self) -> FixPath {
        self.iter().map(|&p| FixVec::new_point(p)).collect()
    }
}

impl ToFixShape for IntShape {
    fn to_fix(&self) -> FixShape {
        FixShape::new(self.iter().map(|path| path.to_fix()).collect())
    }
}

impl ToFixShapes for IntShapes {
    fn to_fix(&self) -> FixShapes {
        self.iter().map(|shape| shape.to_fix()).collect()
    }
}