use std::collections::{HashMap, HashSet};
use crate::fix::doc::FixDoc;
use crate::fix::layer::LayerData;
use crate::fix::mark::Mark;

impl FixDoc {

    pub fn cell_mark_store(&mut self, cell_name: &String, layers: &Vec<LayerData>) -> Result<HashMap<LayerData, Vec<Mark>>, String> {
        let hashset: HashSet<LayerData> = layers.clone().into_iter().collect();

        let mut result: HashMap<LayerData, Vec<Mark>> = HashMap::new();

        let mut new_cache: HashMap<String, HashMap<LayerData, Vec<Mark>>> = HashMap::new();

        for &layer in hashset.iter() {
            let value = self.get_and_cache_marks(&mut new_cache, cell_name, layer)?;
            if let Some(marks) = value {
                result.insert(layer.clone(), marks);
            }
        }

        self.mark_cache.extend(new_cache);

        Ok(result)
    }
}