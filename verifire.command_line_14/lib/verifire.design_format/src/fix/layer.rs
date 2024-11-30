use std::cmp::Ordering;
use std::fmt;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct LayerData {
    #[serde(rename = "layer_number")]
    pub number: i32,
    #[serde(rename = "datatype_number")]
    pub datatype: i32,
}

impl LayerData {
    pub fn new(data: &[i32; 2]) -> Self {
        Self { number: data[0], datatype: data[1] }
    }
}

impl PartialOrd for LayerData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for LayerData {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        let a = self.number.cmp(&other.number);
        if a == Ordering::Equal {
            self.datatype.cmp(&other.datatype)
        } else {
            a
        }
    }
}

impl fmt::Display for LayerData {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}/{})", self.number, self.datatype)
    }
}