use gds21::GdsStructRef;
use crate::fix::scaler::UnitScaler;
use crate::raw::instance::Instance;
use crate::trans::transform::Transform;

pub struct Reference {
    pub(crate) cell: String,
    pub(crate) transform: Transform,
}

impl Reference {
    pub(crate) fn with_raw(raw: &Instance, scaler: UnitScaler) -> Self {
        let origin = scaler.raw_point_to_unit(&raw.origin[0]);

        let transform = Transform::new(
            origin,
            1.0,
            raw.mirror_x.unwrap_or(false),
            raw.rotation.unwrap_or(0.0)
        );

        Self { cell: raw.name.clone(), transform }
    }

    pub(crate) fn with_gds(gds: &GdsStructRef, scaler: UnitScaler) -> Self {
        let transform = Transform::with_gds(&gds.xy, &gds.strans, scaler);
        Self { cell: gds.name.clone(), transform }
    }

    pub(crate) fn as_gds(&self, scaler: UnitScaler) -> GdsStructRef {
        let p0 = self.transform.translate.value();
        let v0 = scaler.unit_point_to_gds(&p0);

        GdsStructRef {
            name: self.cell.clone(),
            xy: v0,
            strans: self.transform.gds_strans(),
            elflags: None,
            plex: None,
            properties: vec![],
        }
    }

    pub(crate) fn as_raw(&self, scaler: UnitScaler) -> Instance {
        let p0 = self.transform.translate.value();
        let v0 = scaler.unit_point_to_raw(&p0);

        Instance {
            name: self.cell.clone(),
            origin: vec![v0],
            rotation: self.transform.rotation.value(),
            mirror_x: Some(self.transform.reflection.value()),
        }
    }
}
