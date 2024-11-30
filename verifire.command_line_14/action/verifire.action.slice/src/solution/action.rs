use foundation::algorithm::slice::slice_type::SliceType;
use foundation::fix::doc::FixDoc;
use foundation::fix::layer::LayerData;
use foundation::task::orientation::Orientation;
use crate::solution::slice::Slice;

pub struct SliceAction {
    slice_type: SliceType,
    orientation: Orientation,
    cell_name: String,
    layer_data: LayerData,
    only_self: bool,
}

impl SliceAction {
    pub fn new(slice_type: SliceType, orientation: Orientation, cell_name: String, layer_data: LayerData, only_self: bool) -> Self {
        Self { slice_type, orientation, cell_name, layer_data, only_self }
    }

    pub fn execute(&self, doc: &mut FixDoc) {
        doc.slice(self.slice_type, self.orientation, &self.cell_name, self.layer_data, self.only_self)
    }
}