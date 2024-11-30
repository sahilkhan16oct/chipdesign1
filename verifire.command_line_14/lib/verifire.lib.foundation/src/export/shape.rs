use crate::i_overlay::i_float::point::IntPoint;
use crate::i_overlay::i_shape::int::path::IntPath;

#[derive(Clone)]
pub enum ExpShape {
    Dot(IntPoint),
    Edge([IntPoint; 2]),
    Path(IntPath),
}