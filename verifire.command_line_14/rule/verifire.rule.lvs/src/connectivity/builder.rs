use std::collections::HashMap;
use foundation::fix::layer::LayerData;
use foundation::fix::layer_binding::{LayerBind, LayerBinding, LayerBinds};
use foundation::fix::mark::Mark;
use foundation::i_overlay::i_shape::int::shape::IntShapes;
use foundation::res::store::StoreRes;
use crate::connectivity::device::Device;
use crate::connectivity::marker::Marker;

impl Device {
    pub(crate) fn build(
        start_index: usize,
        shape_store: &HashMap<LayerData, IntShapes>,
        stack_binds: &Vec<LayerBind>,
        mark_binds: &Vec<LayerBind>,
        mark_store: &HashMap<LayerData, Vec<Mark>>,
    ) -> Result<Vec<Device>, String> {
        let mut devices = Device::devices(start_index, &shape_store, stack_binds);

        let layers = stack_binds.layers();

        for layer in layers.iter() {
            let marks = Self::marks_by_filter(layer, mark_binds, mark_store);
            if marks.is_empty() {
                continue;
            }

            let shapes = shape_store.shapes(layer)?;
            let map = shapes.mark(&marks);
            for (cell_label, shapes) in map.iter() {
                for device in devices.iter_mut() {
                    device.add_cell_label(cell_label, layer, shapes)
                }
            }
        }

        Ok(devices)
    }

    fn marks_by_filter(stack_layer: &LayerData, mark_binds: &Vec<LayerBind>, mark_store: &HashMap<LayerData, Vec<Mark>>) -> Vec<Mark> {
        let mut result = Vec::new();
        for mark_binding in mark_binds.iter() {
            let layer = mark_binding.opposite(stack_layer);
            if let Some(marks) = mark_store.get(&layer) {
                result.append(&mut marks.clone());
            }
        }

        result
    }
}