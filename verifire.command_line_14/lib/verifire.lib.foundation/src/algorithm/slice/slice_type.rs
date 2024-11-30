use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SliceType {
    #[serde(rename = "original")]
    Original,
    #[serde(rename = "outer_box")]
    OuterBox,
}