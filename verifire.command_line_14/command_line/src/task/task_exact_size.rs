use serde::Deserialize;
use foundation::res::viewport::Viewport;

#[derive(Deserialize, Debug)]
pub struct ExactSizeTaskDef {
    pub viewport: Viewport,
    pub width: f64,
    pub height: f64,
}

impl ExactSizeTaskDef {
    pub fn title(&self) -> String {
        format!(
            "ExactSize viewport: {}, width: {}, height: {}",
            self.viewport,
            self.width,
            self.height
        )
    }
}