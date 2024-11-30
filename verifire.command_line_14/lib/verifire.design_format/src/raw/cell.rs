use serde::{Serialize, Deserialize};
use crate::raw::property::Property;

#[derive(Serialize, Deserialize)]
pub struct RawCell {
    pub cell_id: String,
    pub cell_name: String,
    pub properties: Vec<Property>,
}
