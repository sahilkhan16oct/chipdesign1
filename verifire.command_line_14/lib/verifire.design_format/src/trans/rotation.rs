#[derive(Debug, Clone, Copy, PartialEq)]
pub(crate) enum Rotation {
    None,
    Deg90,
    Deg180,
    Deg270,
    Deg(f64),
}


impl Rotation {
    pub(crate) fn value(&self) -> Option<f64> {
        match self {
            Rotation::None => { None }
            Rotation::Deg90 => { Some(90.0) }
            Rotation::Deg180 => { Some(180.0) }
            Rotation::Deg270 => { Some(270.0) }
            Rotation::Deg(degrees) => { Some(degrees.clone()) }
        }
    }

    pub(crate) fn with_degrees(angle: f64) -> Self {
        let q = angle / 90.0;
        let r = q.round();
        if (q - r).abs() < 0.0001 {
            let mut quarter = (r as i64) % 4;
            if quarter < 0 {
                quarter = 2 - quarter
            }

            match quarter {
                0 => Rotation::None,
                1 => Rotation::Deg90,
                2 => Rotation::Deg180,
                3 => Rotation::Deg270,
                _ => Rotation::Deg(angle)
            }
        } else {
            Rotation::Deg(angle)
        }
    }
}