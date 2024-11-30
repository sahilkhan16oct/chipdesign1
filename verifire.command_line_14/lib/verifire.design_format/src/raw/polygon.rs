use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Polygon {
    pub layer_number: i32,
    pub datatype_number: i32,
    pub coordinates: Vec<[f64; 2]>,
}
