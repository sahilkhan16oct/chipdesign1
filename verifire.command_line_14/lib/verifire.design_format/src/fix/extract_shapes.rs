use std::collections::HashMap;
use i_overlay::core::fill_rule::FillRule;
use i_overlay::core::overlay::{Overlay, ShapeType};
use i_overlay::core::overlay_rule::OverlayRule;
use i_overlay::core::simplify::Simplify;
use i_overlay::i_float::point::IntPoint;
use i_overlay::i_shape::int::shape::IntShapes;
use crate::fix::doc::FixDoc;
use crate::fix::layer::LayerData;
use crate::fix::resource::Resource;
use crate::trans::transformable::Translate;
use crate::trans::translation::Translation;

impl FixDoc {
    /// Compute the union of shape for specified layer within a cell.
    ///
    /// # Arguments
    /// * `cell_name` - The name of the target cell.
    /// * `layer` - Layers within the cell for which to compute the union of shapes.
    ///
    /// # Returns
    /// IntShapes - All geometry to its corresponding union FixShapes.
    pub fn get_shapes_for_cell(&self, cell_name: &String, layer: LayerData) -> Result<IntShapes, String> {
        let mut new_cache: HashMap<String, HashMap<LayerData, IntShapes>> = HashMap::new();

        if let Some(shapes) = self.get_and_cache_shapes(&mut new_cache, cell_name, layer)? {
            Ok(shapes)
        } else {
            Err("No data is found".to_string())
        }
    }

    pub(super) fn get_and_cache_shapes(&self, new_cache: &mut HashMap<String, HashMap<LayerData, IntShapes>>, cell_name: &String, layer: LayerData) -> Result<Option<IntShapes>, String> {
        // try old cache
        if let Some(cell) = self.shape_cache.get(cell_name) {
            if let Some(shapes) = cell.get(&layer) {
                return Ok(Some(shapes.clone()));
            }
        }

        // try new cache
        if let Some(cell) = new_cache.get(cell_name) {
            if let Some(shapes) = cell.get(&layer) {
                return Ok(Some(shapes.clone()));
            }
        }

        let cell = if let Some(cell) = self.cells.get(cell_name) {
            cell
        } else {
            return Err(format!("Cell '{cell_name}' does not exist"));
        };

        let mut overlay = Overlay::new(32);
        for res in cell.resources.iter() {
            match res {
                Resource::Array(array) => {
                    let value = self.get_and_cache_shapes(new_cache, &array.cell, layer)?;
                    let template = if let Some(template) = value {
                        template
                    } else {
                        continue;
                    };

                    if template.is_empty() {
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
                            let mut pawn = template.clone();
                            pawn.translate(&translation);

                            overlay.add_shapes(&pawn, ShapeType::Subject);
                        }
                    }
                }
                Resource::Reference(cell_ref) => {
                    let value = self.get_and_cache_shapes(new_cache, &cell_ref.cell, layer)?;
                    let mut template = if let Some(template) = value {
                        template
                    } else {
                        continue;
                    };

                    if template.is_empty() {
                        continue;
                    }

                    template.transform(&cell_ref.transform);

                    overlay.add_shapes(&template, ShapeType::Subject);
                }
                Resource::Polygons(poly) => {
                    if layer == poly.layer {
                        for path in poly.paths.iter() {
                            let clean_shapes = path.simplify(FillRule::NonZero, 0);
                            overlay.add_shapes(&clean_shapes, ShapeType::Subject);
                        }
                    }
                }
                Resource::PathData(path_data) => {
                    if layer == path_data.layer {
                        let shapes = path_data.as_shapes();
                        overlay.add_shapes(&shapes, ShapeType::Subject);
                    }
                }
                _ => {}
            }
        }

        let graph = overlay.into_graph(FillRule::NonZero);

        if graph.links().is_empty() {
            Ok(None)
        } else {
            let shapes = graph.extract_shapes(OverlayRule::Subject);
            let cell = new_cache.entry(cell_name.clone()).or_insert(Default::default());
            debug_assert!(!cell.contains_key(&layer));
            cell.insert(layer.clone(), shapes.clone());

            if shapes.is_empty() {
                Ok(None)
            } else {
                Ok(Some(shapes))
            }
        }
    }
}