use serde::Serialize;
use design_format::fix::scaler::UnitScaler;
use crate::export::shape::ExpShape;

#[derive(Serialize, Debug, Clone)]
pub struct Violation {
    pub r#type: String,
    pub location: Vec<[f64; 2]>,
}

impl Violation {
    pub fn with_shape(shape: &ExpShape, scaler: &UnitScaler) -> Self {
        match shape {
            ExpShape::Dot(dot) => {
                Self {
                    r#type: "point".to_string(),
                    location: vec![scaler.unit_point_to_raw(dot)],
                }
            }
            ExpShape::Path(path) => {
                let location = path.iter().map(|p| scaler.unit_point_to_raw(p)).collect();
                Self {
                    r#type: "path".to_string(),
                    location,
                }
            }
            ExpShape::Edge(edge) => {
                Self {
                    r#type: "edge".to_string(),
                    location: vec![
                        scaler.unit_point_to_raw(&edge[0]),
                        scaler.unit_point_to_raw(&edge[1]),
                    ],
                }
            }
        }
    }
}