use serde::Deserialize;
use foundation::res::viewport::Viewport;

#[derive(Deserialize, Debug)]
pub struct ExtensionTaskDef {
    pub poly: Viewport,
    pub diffusion: Viewport,
    pub value: f64
}

impl ExtensionTaskDef {
    pub fn title(&self) -> String {
        format!(
            "Extension poly: {}, diffusion: {}, value: {}",
            self.poly,
            self.diffusion,
            self.value
        )
    }
}