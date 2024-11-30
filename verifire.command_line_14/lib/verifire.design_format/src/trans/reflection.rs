#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Reflection {
    None,
    Horizontal,
}

impl Reflection {
    pub(crate) fn new(is_reflected: bool) -> Self {
        if is_reflected {
            Self::Horizontal
        } else {
            Self::None
        }
    }

    pub(crate) fn value(&self) -> bool {
        match self {
            Reflection::None => {
                false
            }
            Reflection::Horizontal => {
                true
            }
        }
    }
}