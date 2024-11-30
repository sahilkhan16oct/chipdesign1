use gds21::{GdsPoint, GdsStrans};
use i_overlay::i_float::point::IntPoint;
use crate::fix::scaler::UnitScaler;
use crate::trans::reflection::Reflection;
use crate::trans::rotation::Rotation;
use crate::trans::scale::Scale;
use crate::trans::translation::Translation;

#[derive(Debug, Clone, Copy)]
pub(crate) struct Transform {
    pub(crate) translate: Translation,
    pub(crate) reflection: Reflection,
    pub(crate) rotation: Rotation,
    pub(crate) scale: Scale,
}

impl Transform {

    pub(crate) fn new(origin: IntPoint, magnitude: f64, reflected: bool, degree: f64) -> Self {
        let translate = Translation::with_point(origin);
        let scale = Scale::with_magnitude(magnitude);
        let rotation = Rotation::with_degrees(degree);
        let reflection = Reflection::new(reflected);

        Self { translate, reflection, rotation, scale }
    }

    pub(crate) fn with_gds(point: &GdsPoint, strans: &Option<GdsStrans>, scaler: UnitScaler) -> Self {
        let origin = scaler.gds_point_to_unit(point);
        if let Some(strans) = strans {
            let degree = strans.angle.unwrap_or(0.0);
            let magnitude = strans.mag.unwrap_or(1.0);
            let reflected = strans.reflected;
            Self::new(origin, magnitude, reflected, degree)
        } else {
            Self::new(origin, 1.0, false, 0.0)
        }
    }

    pub(crate) fn gds_strans(&self) -> Option<GdsStrans> {
        let mag = self.scale.value();
        let angle = self.rotation.value();
        let reflection = self.reflection.value();

        if mag.is_none() && angle.is_none() && !reflection {
            None
        } else {
            Some(GdsStrans {
                reflected: reflection,
                abs_mag: false,
                abs_angle: false,
                mag,
                angle,
            })
        }
    }
}