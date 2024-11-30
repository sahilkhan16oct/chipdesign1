use serde::Deserialize;
use foundation::res::viewport::Viewport;

#[derive(Deserialize, Debug)]
pub struct AngleTaskDef {
    pub viewport: Viewport,
    pub is_45_allowed: Option<bool>,
}

impl AngleTaskDef {
    pub fn title(&self) -> String {
        format!(
            "Angle viewport: {}",
            self.viewport
        )
    }
}