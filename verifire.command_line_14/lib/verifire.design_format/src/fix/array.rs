use gds21::GdsArrayRef;
use i_overlay::i_float::point::IntPoint;
use crate::fix::scaler::UnitScaler;
use crate::trans::transform::Transform;

pub struct ArrayRef {
    pub(crate) cell: String,
    pub(crate) rows: i32,
    pub(crate) columns: i32,
    pub(crate) size: IntPoint,
    pub(crate) transform: Transform,
}

impl ArrayRef {
    pub(crate) fn with_gds(data: &GdsArrayRef, scaler: UnitScaler) -> Self {
        let p0 = scaler.gds_point_to_unit(&data.xy[0]);
        let p1 = scaler.gds_point_to_unit(&data.xy[1]);
        let p2 = scaler.gds_point_to_unit(&data.xy[2]);

        let w = p1.x - p0.x;
        let h = p2.y - p0.y;

        let size = IntPoint { x: w, y: h };

        let transform = Transform::with_gds(&data.xy[0], &data.strans, scaler);

        Self {
            cell: data.name.clone(),
            rows: data.rows as i32,
            columns: data.cols as i32,
            size,
            transform,
        }
    }

    pub(crate) fn as_gds(&self, scaler: UnitScaler) -> GdsArrayRef {
        let p0 = self.transform.translate.value();
        let p1 = IntPoint::new(p0.x + self.size.x, p0.y);
        let p2 = IntPoint::new(p0.y, p0.y + self.size.y);

        let v0 = scaler.unit_point_to_gds(&p0);
        let v1 = scaler.unit_point_to_gds(&p1);
        let v2 = scaler.unit_point_to_gds(&p2);

        GdsArrayRef {
            name: self.cell.clone(),
            xy: [v0, v1, v2],
            cols: self.columns as i16,
            rows: self.rows as i16,
            strans: self.transform.gds_strans(),
            elflags: None,
            plex: None,
            properties: vec![],
        }
    }
}