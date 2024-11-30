use serde::{Serialize, Deserialize};
use crate::raw::instance::Instance;
use crate::raw::polygon::Polygon;
use crate::raw::rectangle::Rectangle;
use crate::raw::label::Label;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum Property {
    Instance(Instance),
    Polygon(Polygon),
    Rectangle(Rectangle),
    Label(Label),
}
