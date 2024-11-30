use serde::Deserialize;
use foundation::res::viewport::Viewport;
use min_max_width::solution::solver::MinMaxAlgorithm;

#[derive(Deserialize, Debug)]
pub struct SlitLengthTaskDef {
    pub viewport: Viewport,
    pub algorithm: MinMaxAlgorithm,
    pub value: f64,
}

impl SlitLengthTaskDef {
    pub fn title(&self) -> String {
        format!(
            "SlitLength viewport: {}, algorithm: {:?}, value: {}",
            self.viewport,
            self.algorithm,
            self.value
        )
    }
}