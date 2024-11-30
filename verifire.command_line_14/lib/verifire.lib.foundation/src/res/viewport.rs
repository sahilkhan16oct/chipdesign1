use std::fmt;
use serde::Deserialize;
use design_format::fix::layer::LayerData;

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum AreaOperation {
    Union,
    Intersection,
    Difference,
    Exclusion,
    Select,
    InvertSelect,
    SelectFull,
    InvertSelectFull
}

#[derive(Deserialize, Debug, Clone)]
pub struct Area {
    pub layer: LayerData,
    pub operation: AreaOperation,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Filter {
    pub areas: Vec<Area>,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Viewport {
    pub base: LayerData,
    pub filter: Option<Filter>,
}

impl Viewport {
    pub fn layers(&self) -> Vec<LayerData> {
        let mut result = Vec::new();
        result.push(self.base);
        if let Some(filter) = &self.filter {
            for area in filter.areas.iter() {
                result.push(area.layer)
            }
        }

        result
    }
}

impl fmt::Display for Area {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "layer: {}, operation: {:?}", self.layer, self.operation)
    }
}

impl fmt::Display for Filter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[")?;
        if !self.areas.is_empty() {
            let last_index = self.areas.len() - 1;
            for i in 0..last_index {
                write!(f, "{}, ", self.areas[i])?
            }
            write!(f, "{}", self.areas[last_index])?
        }
        write!(f, "]")
    }
}

impl fmt::Display for Viewport {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if let Some(filter) = &self.filter {
            write!(f, "(base: {}, filter: {})", self.base, filter)
        } else {
            write!(f, "(base: {})", self.base)
        }
    }
}