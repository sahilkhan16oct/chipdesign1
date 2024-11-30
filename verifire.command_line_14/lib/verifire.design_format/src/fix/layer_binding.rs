use std::collections::HashSet;

use crate::fix::layer::LayerData;

pub type LayerBind = [LayerData; 2];

pub trait LayerBinding {
    fn opposite(&self, layer: &LayerData) -> LayerData;
}

pub trait LayerBinds {
    fn layers(&self) -> Vec<LayerData>;
}

impl LayerBinds for [LayerBind] {
    fn layers(&self) -> Vec<LayerData> {
        let mut set: HashSet<LayerData> = HashSet::with_capacity(2 * self.len());
        for bindings in self.iter() {
            set.insert(bindings[0]);
            set.insert(bindings[1]);
        }

        let mut list: Vec<LayerData> = set.into_iter().collect();
        list.sort_unstable();
        list
    }
}

impl LayerBinding for LayerBind {
    fn opposite(&self, layer: &LayerData) -> LayerData {
        if self[0].eq(layer) {
            self[1].clone()
        } else {
            self[0].clone()
        }
    }
}