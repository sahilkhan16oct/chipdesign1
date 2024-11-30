use serde::Deserialize;
use foundation::res::viewport::Viewport;
use foundation::task::direction::SpaceDirection;

#[derive(Deserialize, Debug)]
pub struct SpacingTaskDef {
    pub viewport_0: Viewport,
    pub viewport_1: Option<Viewport>,
    pub spacing: f64,
    pub direction: SpaceDirection,
}

impl SpacingTaskDef {
    pub fn title(&self) -> String {
        if let Some(cell_1) = &self.viewport_1 {
            format!(
                "Spacing viewport_0: {}, viewport_1: {}, direction: {:?}, spacing: {}",
                self.viewport_0,
                cell_1,
                self.direction,
                self.spacing
            )
        } else {
            format!(
                "Spacing cell: {}, direction: {:?}, spacing: {}",
                self.viewport_0,
                self.direction,
                self.spacing
            )
        }
    }
}