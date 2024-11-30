use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Instance {
    pub name: String,
    pub origin: Vec<[f64; 2]>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rotation: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mirror_x: Option<bool>,
}
