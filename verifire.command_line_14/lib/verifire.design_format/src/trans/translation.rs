use i_overlay::i_float::point::IntPoint;

#[derive(Debug, Clone, Copy)]
pub(crate) enum Translation {
    None,
    Int(IntPoint),
}

impl Translation {
    pub(crate) fn with_point(vec: IntPoint) -> Self {
        if vec.x == 0 && vec.y == 0 {
            Self::None
        } else {
            Self::Int(vec)
        }
    }

    pub(crate) fn value(&self) -> IntPoint {
        match self {
            Translation::None => {
                IntPoint::ZERO
            }
            Translation::Int(vec) => {
                vec.clone()
            }
        }
    }
}