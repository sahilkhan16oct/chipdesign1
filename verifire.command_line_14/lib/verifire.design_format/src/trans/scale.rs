#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Scale {
    None,
    Int(i32),
    Float(f64),
}

impl Scale {
    pub(crate) fn with_magnitude(magnitude: f64) -> Self {
        let m = magnitude.round();
        if (m - magnitude).abs() < 0.00001 {
            let i = m as i32;
            if i == 1 {
                Self::None
            } else {
                Self::Int(i)
            }
        } else {
            Self::Float(magnitude)
        }
    }

    pub(crate) fn value(&self) -> Option<f64> {
        match self {
            Scale::None => {
                None
            }
            Scale::Int(value) => {
                Some(value.clone() as f64)
            }
            Scale::Float(value) => {
                Some(value.clone())
            }
        }
    }
}