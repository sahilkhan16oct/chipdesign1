use serde::{Deserialize, Serialize};
use crate::raw::polygon::Polygon;
use crate::raw::rectangle::Rectangle;

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
pub(crate) enum Composition {
    Polygon(Polygon),
    Rectangle(Rectangle),
}