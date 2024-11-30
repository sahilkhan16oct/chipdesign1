use std::collections::{HashMap, HashSet};
use gds21::GdsTextElem;
use i_overlay::i_float::point::IntPoint;
use serde::Deserialize;
use crate::fix::doc::FixDoc;
use crate::fix::layer::LayerData;
use crate::fix::scaler::UnitScaler;

#[derive(Deserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct Mark {
    pub cell: String, // cell name which first spawn this label
    pub label: String,
    pub point: IntPoint,
    pub layer: LayerData
}

impl Mark {
    pub(super) fn with_gds(owner: String, text: &GdsTextElem, scaler: &UnitScaler) -> Self {
        let label = text.string.clone();
        let point = scaler.gds_point_to_unit(&text.xy);
        let layer = LayerData { number: text.layer as i32, datatype: text.texttype as i32 };
        Self { cell: owner, label, point, layer }
    }

    pub(crate) fn as_gds(&self, scaler: UnitScaler) -> GdsTextElem {
        let xy = scaler.unit_point_to_gds(&self.point);
        GdsTextElem {
            string: self.label.clone(),
            layer: self.layer.number as i16,
            texttype: self.layer.datatype as i16,
            xy,
            presentation: None,
            path_type: None,
            width: None,
            strans: None,
            elflags: None,
            plex: None,
            properties: vec![],
        }
    }
}

impl FixDoc {
    /// Computes and caches the union of shapes for specified layers within a cell.
    ///
    /// # Arguments
    /// * `cell_name` - The name of the target cell.
    /// * `layers` - Layers within the cell for which to compute the union of shapes.
    ///
    /// # Returns
    /// HashMap<Layer, IntShapes> - A map from Layer to its corresponding union FixShapes.
    pub fn marks(&mut self, cell_name: &String, layers: &Vec<LayerData>) -> Result<HashMap<LayerData, Vec<Mark>>, String> {
        let hashset: HashSet<LayerData> = layers.clone().into_iter().collect();

        let mut result: HashMap<LayerData, Vec<Mark>> = HashMap::new();

        let mut new_cache: HashMap<String, HashMap<LayerData, Vec<Mark>>> = HashMap::new();

        for &layer in hashset.iter() {
            let value = self.get_and_cache_marks(&mut new_cache, cell_name, layer)?;
            if let Some(shapes) = value {
                result.insert(layer.clone(), shapes);
            }
        }

        self.mark_cache.extend(new_cache);

        Ok(result)
    }
}