use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Label {
    pub name: String,
    pub layer_number: u16,
    pub datatype_number: i16,
    pub coordinates: Vec<(f64, f64)>,
    pub rotation: Option<f64>,
}
