use std::collections::{HashMap, HashSet};
use i_overlay::i_shape::int::shape::IntShapes;
use crate::fix::doc::FixDoc;
use crate::fix::layer::LayerData;

impl FixDoc {
    /// Computes and caches the union of shapes for specified layers within a cell.
    ///
    /// # Arguments
    /// * `cell_name` - The name of the target cell.
    /// * `layers` - Layers within the cell for which to compute the union of shapes.
    ///
    /// # Returns
    /// HashMap<Layer, IntShapes> - A map from Layer to its corresponding union FixShapes.
    pub fn cell_shape_store(&mut self, cell_name: &String, layers: &Vec<LayerData>) -> Result<HashMap<LayerData, IntShapes>, String> {
        let hashset: HashSet<LayerData> = layers.clone().into_iter().collect();

        let mut result: HashMap<LayerData, IntShapes> = HashMap::new();

        let mut new_cache: HashMap<String, HashMap<LayerData, IntShapes>> = HashMap::new();

        for &layer in hashset.iter() {
            let value = self.get_and_cache_shapes(&mut new_cache, cell_name, layer)?;
            if let Some(shapes) = value {
                result.insert(layer.clone(), shapes);
            }
        }

        self.shape_cache.extend(new_cache);

        Ok(result)
    }

    /// Computes and caches the union of shapes for specified layers within a cell.
    ///
    /// # Arguments
    /// * `cell_name` - The name of the target cell.
    ///
    /// # Returns
    /// HashMap<Layer, IntShapes> - A map from Layer to its corresponding union FixShapes.
    pub fn cell_shapes(&mut self, cell_name: &String) -> Result<HashMap<LayerData, IntShapes>, String> {
        let hashset = self.get_layers_set_for_cell(cell_name)?;

        let mut result: HashMap<LayerData, IntShapes> = HashMap::new();

        let mut new_cache: HashMap<String, HashMap<LayerData, IntShapes>> = HashMap::new();

        for &layer in hashset.iter() {
            let value = self.get_and_cache_shapes(&mut new_cache, cell_name, layer)?;
            if let Some(shapes) = value {
                result.insert(layer.clone(), shapes);
            }
        }

        self.shape_cache.extend(new_cache);

        Ok(result)
    }
}
