use serde::Deserialize;
use design_format::fix::layer::LayerData;
use foundation::algorithm::slice::slice_type::SliceType;
use foundation::task::orientation::Orientation;

#[derive(Deserialize, Debug)]
pub struct SliceActionDef {
    pub cell: String,
    pub layer_data: LayerData,
    pub slice_type: SliceType,
    pub orientation: Orientation,
    pub self_only: bool
}

impl SliceActionDef {
    pub fn title(&self) -> String {
        format!(
            "Slice cell: {}, layer_data: {}, self_only: {}",
            self.cell,
            self.layer_data,
            self.self_only
        )
    }
}