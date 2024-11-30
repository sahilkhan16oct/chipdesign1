use std::collections::HashSet;
use crate::fix::layer::LayerData;
use crate::fix::layer_binding::LayerBind;

pub type LayerChain = Vec<[i32; 2]>;

pub trait ToLayerBinds {
    fn layers(&self) -> Vec<LayerBind>;
}

impl ToLayerBinds for LayerChain {
    fn layers(&self) -> Vec<LayerBind> {
        let mut set: HashSet<LayerBind> = HashSet::with_capacity(self.len());
        fill(&mut set, self);
        set.into_iter().collect()
    }
}

impl ToLayerBinds for Vec<LayerChain> {
    fn layers(&self) -> Vec<LayerBind> {
        let mut set: HashSet<LayerBind> = HashSet::with_capacity(3 * self.len());
        for chain in self.iter() {
            fill(&mut set, chain);
        }
        set.into_iter().collect()
    }
}

fn fill(set: &mut HashSet<LayerBind>, chains: &LayerChain) {
    let mut i = 1;
    while i < chains.len() {
        let a = chains[i - 1];
        let b = chains[i];

        let is_less = if a[0] == b[0] { a[1] < b[1] } else { a[0] < b[0] };

        if is_less {
            set.insert([LayerData::new(&a), LayerData::new(&b)]);
        } else {
            set.insert([LayerData::new(&b), LayerData::new(&a)]);
        }
        i += 1;
    }
}