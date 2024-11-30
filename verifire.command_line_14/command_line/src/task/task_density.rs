use serde::Deserialize;
use density::solution::solver::DensityResultType;
use foundation::res::viewport::Viewport;

#[derive(Deserialize, Debug)]
pub struct DensityTaskDef {
    pub viewport: Viewport,
    pub density: f64,
    pub sampling_level: usize,
    pub window_width: f64,
    pub window_height: f64,
    pub result_type: DensityResultType,
    pub is_debug: Option<bool>,
}

impl DensityTaskDef {
    pub fn title(&self) -> String {
        format!(
            "Density viewport: {}, density: {}, sampling_level: {}, window_size: ({}, {})",
            self.viewport,
            self.density,
            self.sampling_level,
            self.window_width,
            self.window_height
        )
    }
}
