use serde::Deserialize;
use exact_size::solution::check_shape_solver::ShapeForm;
use foundation::res::viewport::Viewport;

#[derive(Deserialize, Debug)]
pub struct CheckShapeTaskDef {
    pub viewport: Viewport,
    pub form: ShapeForm
}

impl CheckShapeTaskDef {
    pub fn title(&self) -> String {
        format!(
            "ExactSize viewport: {}, form: {:?}",
            self.viewport,
            self.form
        )
    }
}