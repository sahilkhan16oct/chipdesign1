use serde::Deserialize;
use foundation::res::viewport::Viewport;

#[derive(Deserialize, Debug)]
pub struct NotchTaskDef {
    pub viewport: Viewport,
    pub min_length: f64,
}

impl NotchTaskDef {
    pub fn title(&self) -> String {
        format!(
            "Notch viewport: {}, min_length: {}",
            self.viewport,
            self.min_length
        )
    }
}