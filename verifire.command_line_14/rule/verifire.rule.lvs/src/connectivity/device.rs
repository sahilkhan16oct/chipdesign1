use std::collections::{HashMap, HashSet};
use foundation::fix::layer::LayerData;
use foundation::fix::layer_binding::{LayerBind, LayerBinds};
use foundation::i_overlay::i_shape::int::shape::IntShapes;
use foundation::res::store::StoreRes;
use crate::connectivity::connector::Connector;
use crate::connectivity::layer::StackLayer;
use crate::connectivity::marker::CellLabel;

pub(crate) type Cell = String;
pub(crate) type Label = String;

pub(crate) struct Device {
    pub(crate) id: usize,
    pub(crate) index: usize,
    pub(crate) labels: HashMap<Label, HashSet<Cell>>,
    pub(crate) shapes: HashMap<LayerData, Vec<usize>>,
}

#[derive(Debug, Clone)]
struct ShapeId {
    layer: usize,
    shape: usize,
}

#[derive(Debug, Clone)]
struct ShapeNode {
    links: Vec<ShapeId>,
}

struct Graph {
    lands: Vec<Vec<ShapeNode>>,
}

impl Device {
    pub(crate) fn devices(start_index: usize, store: &HashMap<LayerData, IntShapes>, bindings: &Vec<LayerBind>) -> Vec<Device> {
        let layers = bindings.layers();

        let mut stacks = Vec::with_capacity(layers.len());
        let mut lands = Vec::with_capacity(layers.len());
        for layer_index in 0..layers.len() {
            let layer_data = layers[layer_index];
            if let Ok(shapes) = store.shapes(&layer_data) {
                let n = shapes.len();
                stacks.push(StackLayer::new(shapes));
                lands.push(vec![ShapeNode { links: vec![] }; n]);
            }
        }

        for jumper in bindings.iter() {
            let li0 = layers.iter().position(|ld| ld.eq(&jumper[0])).unwrap();
            let li1 = layers.iter().position(|ld| ld.eq(&jumper[1])).unwrap();

            let s0 = &stacks[li0];
            let s1 = &stacks[li1];

            let connections = Connector::find_connections(s0, s1);
            for connection in connections.iter() {
                let i0 = connection[0];
                let i1 = connection[1];

                let shape0 = ShapeId { layer: li0, shape: i0 };
                let shape1 = ShapeId { layer: li1, shape: i1 };

                lands.get_mut(li0).unwrap().get_mut(i0).unwrap().links.push(shape1);
                lands.get_mut(li1).unwrap().get_mut(i1).unwrap().links.push(shape0);
            }
        }

        Graph { lands }.devices(start_index, &layers)
    }

    pub(crate) fn add_cell_label(&mut self, cell_label: &CellLabel, layer: &LayerData, shapes: &Vec<usize>) {
        let device_shapes = if let Some(shapes) = self.shapes.get(layer) {
            shapes
        } else {
            return;
        };

        for shape in shapes.iter() {
            if let Ok(_) = device_shapes.binary_search(&shape) {
                if let Some(item) = self.labels.get_mut(&cell_label.label) {
                    item.insert(cell_label.cell.clone());
                } else {
                    let mut hash_set = HashSet::with_capacity(2);
                    hash_set.insert(cell_label.cell.clone() as Cell);
                    self.labels.insert(cell_label.label.clone(), hash_set);
                }
            }
        }
    }

    pub fn name(&self) -> String {
        let n = self.labels.len();
        if n == 0 {
            format!("device_{}", self.id)
        } else {
            let mut name = String::new();
            for (index, key) in self.labels.keys().enumerate() {
                name.push_str(key);
                if index != n - 1 {
                    name.push_str("_");
                }
            }
            name
        }
    }
}

impl Graph {
    pub(crate) fn devices(&self, start_index: usize, layers: &Vec<LayerData>) -> Vec<Device> {
        let mut map = Vec::with_capacity(self.lands.len());
        for land_index in 0..self.lands.len() {
            let n = self.lands[land_index].len();
            map.push(vec![true; n]);
        }

        let mut devices = Vec::new();

        for land_index in 0..self.lands.len() {
            let land = &self.lands[land_index];
            for shape_index in 0..land.len() {
                if map[land_index][shape_index] {
                    map[land_index][shape_index] = false;
                    let start = ShapeId { layer: land_index, shape: shape_index };
                    let shapes = self.travel(start, &mut map);
                    let dev_map = shapes.into_iter()
                        .map(|(key, value)| (layers[key].clone(), value))
                        .collect();
                    devices.push(Device {
                        id: start_index + devices.len(),
                        index: devices.len(),
                        labels: Default::default(),
                        shapes: dev_map,
                    })
                }
            }
        }

        devices
    }

    fn travel(&self, start: ShapeId, map: &mut Vec<Vec<bool>>) -> HashMap<usize, Vec<usize>> {
        let mut shapes = HashMap::new();
        shapes.insert(start.layer, vec![start.shape]);

        let mut wave = Vec::new();
        wave.push(start);

        let mut next = Vec::new();
        while !wave.is_empty() {
            for shape in wave.iter() {
                let links = &self.lands[shape.layer][shape.shape].links;
                for link in links.iter() {
                    let id = map.get_mut(link.layer).unwrap().get_mut(link.shape).unwrap();
                    if *id {
                        *id = false;
                        next.push(link.clone());
                        shapes.entry(link.layer).or_insert_with(Vec::new).push(link.shape);
                    }
                }
            }
            std::mem::swap(&mut wave, &mut next);
            next.clear()
        }

        for (_, item) in shapes.iter_mut() {
            item.sort_unstable();
        }

        shapes
    }
}