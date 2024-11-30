use serde::Deserialize;
use foundation::res::viewport::Viewport;

#[derive(Deserialize, Debug)]
pub struct MinAreaTaskDef {
    pub viewport: Viewport,
    pub min_area: f64,
}

impl MinAreaTaskDef {
    pub fn title(&self) -> String {
        format!(
            "MinArea viewport: {}, min_area: {}",
            self.viewport,
            self.min_area
        )
    }
}