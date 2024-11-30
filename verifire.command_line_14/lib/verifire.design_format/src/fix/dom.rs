use std::collections::HashSet;
use crate::fix::doc::FixDoc;
use crate::fix::layer::LayerData;
use crate::fix::resource::Resource;

impl FixDoc {

    pub fn find_all_polygon_reference(&self, cell_name: &String, layer: Option<LayerData>) -> Vec<String> {
        let mut result = HashSet::new();
        let mut round = Vec::new();
        let mut next_round = HashSet::new();

        round.push(cell_name.clone());

        while !round.is_empty() {
            for name in round.iter() {
                let cell = if let Some(cell) = self.cells.get(name) {
                    cell
                } else {
                    continue;
                };
                let mut is_cell_not_added = true;
                for res in cell.resources.iter() {
                    match res {
                        Resource::Array(array) => {
                            next_round.insert(array.cell.clone());
                        }
                        Resource::Reference(reference) => {
                            next_round.insert(reference.cell.clone());
                        }
                        Resource::Polygons(polygons) => {
                            if is_cell_not_added {
                                if let Some(layer) = layer {
                                    if layer == polygons.layer {
                                        is_cell_not_added = false;
                                        result.insert(name.clone());
                                    }
                                } else {
                                    is_cell_not_added = false;
                                    result.insert(name.clone());
                                }
                            }
                        }
                        _ => {
                            log::error!("FixDoc(Dom) Resource::Mark is not implemented")
                        }
                    }
                }
            }
            round.clear();
            round.extend(next_round.drain());
            next_round.clear();
        }

        result.into_iter().collect()
    }
}