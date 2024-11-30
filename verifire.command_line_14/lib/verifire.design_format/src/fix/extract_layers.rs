use std::collections::HashSet;
use crate::fix::doc::FixDoc;
use crate::fix::layer::LayerData;
use crate::fix::resource::Resource;

impl FixDoc {

    pub fn get_layers_for_cell(&self, cell_name: &String) -> Result<Vec<LayerData>, String> {
        let hash_set = self.get_layers_set_for_cell(cell_name)?;
        Ok(hash_set.into_iter().collect())
    }

    pub(super) fn get_layers_set_for_cell(&self, cell_name: &String) -> Result<HashSet<LayerData>, String> {
        let mut hash_set = HashSet::new();

        let cell = if let Some(cell) = self.cells.get(cell_name) {
            cell
        } else {
            return Err(format!("Cell '{cell_name}' does not exist"));
        };

        for res in cell.resources.iter() {
            match res {
                Resource::Array(array) => {
                    let sub_layers = self.get_layers_for_cell(&array.cell)?;
                    hash_set.extend(sub_layers);
                }
                Resource::Reference(cell_ref) => {
                    let sub_layers = self.get_layers_for_cell(&cell_ref.cell)?;
                    hash_set.extend(sub_layers);
                }
                Resource::Polygons(poly) => {
                    hash_set.insert(poly.layer);
                }
                Resource::PathData(path_data) => {
                    hash_set.insert(path_data.layer);
                }
                Resource::Mark(mark) => {
                    hash_set.insert(mark.layer);
                }
            }
        }

        Ok(hash_set)
    }

}