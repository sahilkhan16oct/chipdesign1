use crate::i_overlay::core::overlay_rule::OverlayRule;
use crate::i_overlay::i_shape::int::shape::IntShapes;

pub enum AreaRes {
    Boolean(BooleanAreaRes),
    Select(SelectAreaRes)
}

pub struct BooleanAreaRes {
    pub shapes: IntShapes,
    pub operation: OverlayRule
}

pub struct SelectAreaRes {
    pub shapes: IntShapes,
    pub inverted: bool,         // default is false
    pub full_include: bool,     // default is false
}

pub struct FilterRes {
    pub areas: Vec<AreaRes>,
}