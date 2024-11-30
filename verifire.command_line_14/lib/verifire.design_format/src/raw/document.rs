use serde::{Serialize, Deserialize};
use crate::raw::cell::RawCell;
use crate::raw::unit_size::UnitSize;

#[derive(Serialize, Deserialize)]
pub struct RawDocument {
    #[serde(default)]
    pub name: String,
    pub units: UnitSize,
    pub cells: Vec<RawCell>
}
