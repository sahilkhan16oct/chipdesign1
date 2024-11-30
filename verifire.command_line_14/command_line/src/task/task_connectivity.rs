use std::collections::{HashMap, HashSet};
use serde::Deserialize;
use design_format::fix::layer::LayerData;
use design_format::fix::layer_chain::{LayerChain, ToLayerBinds};
use foundation::i_overlay::i_shape::int::shape::IntShapes;
use foundation::res::shape_res::ShapeRes;
use foundation::res::store::StoreRes;
use foundation::res::viewport::Viewport;

#[derive(Deserialize, Debug)]
pub struct ConnectivityTaskDef {
    pub labels: Vec<LayerChain>,
    pub layer_stack: Vec<LayerChain>,
    pub filters: Option<Vec<Viewport>>,
    pub only_errors: Option<bool>, // by default true
}

impl ConnectivityTaskDef {
    pub fn title(&self) -> String {
        format!(
            "Connectivity layer_stack: {:?}",
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

        if let Some(filters) = &self.filters {
            for viewport in filters {
                set.extend(viewport.layers());
            }
        }

        set.into_iter().collect()
    }

    pub fn shapes_res(&self, shape_store: &HashMap<LayerData, IntShapes>) -> Result<HashMap<LayerData, ShapeRes>, String> {
        let layers = self.shape_layers();
        let mut map = HashMap::with_capacity(layers.len());

        for layer in layers.iter() {
            if let Some(filters) = &self.filters {
                if let Some(viewport) = filters.iter().find(|it| it .base.eq(layer)) {
                    let res = shape_store.resource_by_viewport(viewport)?;
                    map.insert(layer.clone(), res);
                    continue;
                }
            }

            if let Some(shapes) = shape_store.get(layer) {
                map.insert(layer.clone(), ShapeRes::new(shapes.clone()));
            } else {
                return Err(format!("layer {} is not exist", layer))
            }
        }

        Ok(map)
    }
}