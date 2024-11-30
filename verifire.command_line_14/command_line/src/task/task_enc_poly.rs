use serde::Deserialize;
use foundation::res::viewport::Viewport;
use foundation::task::direction::SpaceDirection;

#[derive(Deserialize, Debug)]
pub struct PolyEnclosureTaskDef {
    pub outer: Viewport,
    pub inner: Viewport,
    pub value: f64,
    pub direction: SpaceDirection,
}

impl PolyEnclosureTaskDef {
    pub fn title(&self) -> String {
        format!(
            "PolyEnclosure float: {}, poly: {}, value: {}, direction: {:?}",
            self.outer,
            self.inner,
            self.value,
            self.direction,
        )
    }
}