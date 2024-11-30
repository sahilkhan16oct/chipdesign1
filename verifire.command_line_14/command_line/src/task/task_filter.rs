use serde::Deserialize;
use foundation::res::viewport::Viewport;

#[derive(Deserialize, Debug)]
pub struct FilterTaskDef {
    pub viewport: Viewport
}

impl FilterTaskDef {
    pub fn title(&self) -> String {
        format!(
            "Filter viewport: {}",
            self.viewport
        )
    }
}