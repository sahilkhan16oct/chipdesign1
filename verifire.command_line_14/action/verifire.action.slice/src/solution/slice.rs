use foundation::algorithm::slice::slice_type::SliceType;
use foundation::fix::cell::Cell;
use foundation::fix::doc::FixDoc;
use foundation::fix::layer::LayerData;
use foundation::fix::polygon_res::PolygonRes;
use foundation::fix::resource::Resource::Polygons;
use foundation::i_overlay::core::fill_rule::FillRule;
use foundation::i_overlay::core::overlay::{Overlay, ShapeType};
use foundation::i_overlay::core::overlay_rule::OverlayRule;
use foundation::task::orientation::Orientation;
use crate::solution::solver::SliceSolver;


pub trait Slice {
    fn slice(&mut self, slice_type: SliceType, orientation: Orientation, cell_name: &String, layer_data: LayerData, only_self: bool);
}

impl Slice for FixDoc {
    fn slice(&mut self, slice_type: SliceType, orientation: Orientation, cell_name: &String, layer_data: LayerData, only_self: bool) {
        if only_self {
            if let Some(cell) = self.cells.get_mut(cell_name) {
                slice_cell(cell, orientation, slice_type, layer_data);
            }
        } else {
            let cell_names = self.find_all_polygon_reference(cell_name, Some(layer_data));
            for cell_name in cell_names.iter() {
                if let Some(cell) = self.cells.get_mut(cell_name) {
                    slice_cell(cell, orientation, slice_type, layer_data);
                }
            }
        }
    }
}

fn slice_cell(cell: &mut Cell, orientation: Orientation, slice_type: SliceType, layer_data: LayerData) {
    let mut overlay = Overlay::new(16);
    let mut i = 0;
    while i < cell.resources.len() {
        let res = &cell.resources[i];
        if let Polygons(polygons) = res {
            if layer_data == polygons.layer {
                overlay.add_paths(&polygons.paths, ShapeType::Subject);
                if i + 1 == cell.resources.len() {
                    cell.resources.pop();
                } else {
                    cell.resources.swap_remove(i);
                }
                continue;
            }
        }
        i += 1;
    }

    let union = overlay.into_graph(FillRule::NonZero).extract_shapes(OverlayRule::Subject);
    let slices = SliceSolver::slice(union, orientation, slice_type);

    let res = PolygonRes { layer: layer_data, paths: slices };

    cell.resources.push(Polygons(res));
}
