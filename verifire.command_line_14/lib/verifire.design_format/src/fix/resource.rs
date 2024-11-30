use crate::fix::array::ArrayRef;
use crate::fix::mark::Mark;
use crate::fix::path_data::PathData;
use crate::fix::polygon_res::PolygonRes;
use crate::fix::reference::Reference;

pub enum Resource {
    Array(ArrayRef),
    Reference(Reference),
    PathData(PathData),
    Polygons(PolygonRes),
    Mark(Mark),
}