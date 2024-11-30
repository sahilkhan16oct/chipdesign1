use gds21::GdsBoundary;
use i_overlay::i_shape::int::path::IntPath;
use crate::fix::layer::LayerData;
use crate::fix::scaler::UnitScaler;
use crate::raw::polygon::Polygon;
use crate::raw::property::Property;
use crate::raw::rectangle::Rectangle;

pub struct PolygonRes {
    pub layer: LayerData,
    pub paths: Vec<IntPath>,
}

impl PolygonRes {

    pub(crate) fn add_polygon(&mut self, polygon: &Polygon, scaler: UnitScaler) {
        self.paths.push(scaler.raw_points_to_unit(&polygon.coordinates));
    }

    pub(crate) fn add_rectangle(&mut self, rectangle: &Rectangle, scaler: UnitScaler) {
        self.paths.push(scaler.raw_points_to_unit(&rectangle.coordinates));
    }

    pub(crate) fn add_boundary(&mut self, boundary: &GdsBoundary, scaler: UnitScaler) {
        let mut points = scaler.gds_points_to_unit(&boundary.xy);
        if points.len() > 3 {
            if points[0] == points[points.len() - 1] {
                points.pop();
            }
            self.paths.push(points);
        }
    }

    pub(crate) fn as_gds(&self, scaler: UnitScaler) -> Vec<GdsBoundary> {
        let mut result = Vec::with_capacity(self.paths.len());
        for path in self.paths.iter() {
            let mut points = scaler.unit_points_to_gds(path);
            if points.len() < 4 {
                continue;
            }

            if points[0] != points[points.len() - 1] {
                points.push(points[0].clone());
            }

            let boundary = GdsBoundary {
                layer: self.layer.number as i16,
                datatype: self.layer.datatype as i16,
                xy: points,
                elflags: None,
                plex: None,
                properties: vec![],
            };

            result.push(boundary);
        }

        result
    }

    pub(crate) fn as_raw(&self, scaler: UnitScaler) -> Vec<Property> {
        let mut result = Vec::with_capacity(self.paths.len());
        for path in self.paths.iter() {
            let points = scaler.unit_points_to_raw(path);
            let prop = if Self::is_rect(&path) {
                Property::Rectangle(Rectangle {
                    layer_number: self.layer.number,
                    datatype_number: self.layer.datatype,
                    coordinates: points,
                })
            } else {
                Property::Polygon(Polygon {
                    layer_number: self.layer.number,
                    datatype_number: self.layer.datatype,
                    coordinates: points,
                })
            };
            result.push(prop);
        }

        result
    }

    fn is_rect(path: &IntPath) -> bool {
        if path.len() != 4 {
            return false;
        }

        let mut pi = path[3];
        let mut x = 0;
        let mut y = 0;
        for &pj in path {
            let d = pi.subtract(pj);
            if d.x == 0 {
                x += 1;
            }
            if d.y == 0 {
                y += 1;
            }
            pi = pj;
        }

        return x == 2 && y == 2;
    }
}
