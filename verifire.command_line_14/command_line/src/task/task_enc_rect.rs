use serde::Deserialize;
use foundation::res::viewport::Viewport;

#[derive(Deserialize, Debug)]
pub struct RectEnclosureTaskDef {
    pub outer: Viewport,
    pub rects: Viewport,
    pub expression: String,
    pub value_0: f64,
    pub value_1: Option<f64>,
    pub only_inner: Option<bool>, // by default true
}

impl RectEnclosureTaskDef {
    pub fn title(&self) -> String {
        let only_inner = self.only_inner.unwrap_or(true);
        format!(
            "RectEnclosure float: {}, rects: {}, only_inner: {}, expression: {}",
            self.outer,
            self.rects,
            only_inner,
            self.expression
        )
    }
}