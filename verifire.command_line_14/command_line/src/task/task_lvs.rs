use std::collections::{HashMap, HashSet};
use serde::Deserialize;
use design_format::fix::layer::LayerData;
use design_format::fix::layer_chain::{LayerChain, ToLayerBinds};
use foundation::i_overlay::i_shape::int::shape::IntShapes;
use foundation::res::shape_res::ShapeRes;
use foundation::res::viewport::Viewport;
use lvs::solution::lvs::LvsTransistor;

#[derive(Deserialize, Clone, Debug)]
pub struct LvsTaskDef {
    pub labels: Vec<LayerChain>,
    pub layer_stack: Vec<LayerChain>,
    pub poly: Viewport,
    pub diffusion: Viewport,
    pub transistors: Vec<LvsTransistor>,
}

impl LvsTaskDef {
    pub fn title(&self) -> String {
        format!(
            "Lvs layer_stack: {:?}",
            self.layer_stack
        )
    }

    pub fn shape_layers(&self) -> Vec<LayerData> {
        let mut set: HashSet<LayerData> = HashSet::with_capacity(2 * self.layer_stack.len());
        let layer_binds = self.layer_stack.layers();

        for ls in layer_binds.iter() {
            set.insert(ls[0]);
            set.insert(ls[1]);
        }

        for transistor in self.transistors.iter() {
            set.insert(transistor.source_drain_implant);
            set.insert(transistor.body_implant);
            if let Some(x_well) = transistor.x_well {
                set.insert(x_well);
            }
        }

        set.extend(self.poly.layers());
        set.extend(self.diffusion.layers());

        set.into_iter().collect()
    }

    pub fn shapes_res(&self, shape_store: &HashMap<LayerData, IntShapes>) -> Result<HashMap<LayerData, ShapeRes>, String> {
        let layers = self.shape_layers();
        let mut map = HashMap::with_capacity(layers.len());

        for layer in layers.iter() {
            if let Some(shapes) = shape_store.get(layer) {
                map.insert(layer.clone(), ShapeRes::new(shapes.clone()));
            } else {
                return Err(format!("layer {} is not exist", layer));
            }
        }

        Ok(map)
    }
}