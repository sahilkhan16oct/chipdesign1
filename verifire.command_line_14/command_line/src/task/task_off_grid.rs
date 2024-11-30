use serde::Deserialize;
use foundation::res::viewport::Viewport;

#[derive(Deserialize, Debug)]
pub struct OffGridTaskDef {
    pub viewport: Viewport,
    pub step: f64,
}

impl OffGridTaskDef {
    pub fn title(&self) -> String {
        format!(
            "OffGrid viewport: {}, step: {}",
            self.viewport,
            self.step
        )
    }
}