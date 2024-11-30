use std::collections::HashMap;
use i_overlay::i_float::point::IntPoint;
use crate::fix::doc::FixDoc;
use crate::fix::layer::LayerData;
use crate::fix::mark::Mark;
use crate::fix::resource::Resource;
use crate::trans::transformable::Translate;
use crate::trans::translation::Translation;

impl FixDoc {
    /// Extract marks for specified layer within a cell.
    ///
    /// # Arguments
    /// * `cell_name` - The name of the target cell.
    /// * `layer` - Layers within the cell for which to compute the marks.
    ///
    /// # Returns
    /// Vec<Mark> - marks.
    pub fn get_marks_for_cell(&self, cell_name: &String, layer: LayerData) -> Result<Vec<Mark>, String> {
        let mut new_cache: HashMap<String, HashMap<LayerData, Vec<Mark>>> = HashMap::new();

        if let Some(marks) = self.get_and_cache_marks(&mut new_cache, cell_name, layer)? {
            Ok(marks)
        } else {
            Err("No data is found".to_string())
        }
    }

    pub(super) fn get_and_cache_marks(&self, new_cache: &mut HashMap<String, HashMap<LayerData, Vec<Mark>>>, cell_name: &String, layer: LayerData) -> Result<Option<Vec<Mark>>, String> {
        // try old cache
        if let Some(cell) = self.mark_cache.get(cell_name) {
            if let Some(marks) = cell.get(&layer) {
                return Ok(Some(marks.clone()));
            }
        }

        // try new cache
        if let Some(cell) = new_cache.get(cell_name) {
            if let Some(marks) = cell.get(&layer) {
                return Ok(Some(marks.clone()));
            }
        }

        let cell = if let Some(cell) = self.cells.get(cell_name) {
            cell
        } else {
            return Err(format!("Cell '{cell_name}' does not exist"));
        };

        let mut result = Vec::new();
        for res in cell.resources.iter() {
            match res {
                Resource::Array(array) => {
                    let value = self.get_and_cache_marks(new_cache, &array.cell, layer)?;
                    let marks = if let Some(template) = value {
                        template
                    } else {
                        continue;
                    };

                    if marks.is_empty() {
                        continue;
                    }

                    let cols = array.columns;
                    let rows = array.rows;
                    let w = array.size.x as f64;
                    let h = array.size.y as f64;
                    let sx = w / cols as f64;
                    let sy = h / rows as f64;

                    let origin = array.transform.translate.value();
                    let x0 = origin.x;
                    let y0 = origin.y;

                    for iy in 0..rows {
                        let y = ((iy as f64) * sy).round() as i32 + y0;
                        for ix in 0..cols {
                            let x = ((ix as f64) * sx).round() as i32 + x0;
                            let translation = Translation::with_point(IntPoint::new(x, y));
                            for mark in marks.iter() {
                                let mut m = mark.clone();
                                m.translate(&translation);
                                result.push(m);
                            }
                        }
                    }
                }
                Resource::Reference(cell_ref) => {
                    let value = self.get_and_cache_marks(new_cache, &cell_ref.cell, layer)?;
                    let marks = if let Some(template) = value {
                        template
                    } else {
                        continue;
                    };

                    if marks.is_empty() {
                        continue;
                    }

                    for mark in marks.iter() {
                        let mut m = mark.clone();
                        m.transform(&cell_ref.transform);
                        result.push(m);
                    }
                }
                Resource::Mark(mark) => {
                    if layer == mark.layer {
                        result.push(mark.clone());
                    }
                }
                _ => {}
            }
        }

        let cell = new_cache.entry(cell_name.clone()).or_insert(Default::default());
        cell.insert(layer.clone(), result.clone());

        Ok(Some(result))
    }
}