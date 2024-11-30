use std::collections::{HashMap, HashSet};
use std::string::ToString;
use foundation::export::lvs_file::{LvsDevice, LvsFile};
use foundation::fix::layer::LayerData;
use foundation::fix::layer_binding::{LayerBind, LayerBinding};
use foundation::fix::mark::Mark;
use foundation::i_overlay::core::fill_rule::FillRule;
use foundation::i_overlay::core::overlay::Overlay;
use foundation::i_overlay::core::overlay_rule::OverlayRule;
use foundation::i_overlay::i_float::point::IntPoint;
use foundation::i_overlay::i_float::rect::IntRect;
use foundation::i_overlay::i_shape::int::shape::IntShapes;
use foundation::res::shape_res::ShapeRes;
use foundation::i_overlay::id_point::IdPoint;
use foundation::i_overlay::vector::vector::{VectorPath, VectorShape};
use serde::Deserialize;
use crate::connectivity::bool::BooleanOperation;
use crate::connectivity::device::Device;
use crate::connectivity::search::DeviceSearch;
use crate::connectivity::rect::Rect;

#[derive(Deserialize, Debug, Clone)]
pub struct LvsTransistor {
    pub name: String,
    pub id_prefix: String,
    pub source_drain_implant: LayerData,
    pub body_implant: LayerData,
    pub power_supply: String,
    pub x_well: Option<LayerData>,
}

struct Origin<'a> {
    transistor: &'a LvsTransistor,
    source_drain_shapes: IntShapes,
    body_shapes: IntShapes,
}

struct SiliconBundle<'a> {
    transistor: &'a LvsTransistor,
    devices: Vec<Device>,
    semiconductors: Vec<Semiconductor>,
}

struct Semiconductor {
    width: f64,
    length: f64,
    left: IntPoint,
    right: IntPoint,
    center: IntPoint,
    left_device: usize,
    right_device: usize,
    gate_device: usize,
    body_devices: Vec<usize>,
}

impl Semiconductor {
    fn new(width: f64, length: f64, left: IntPoint, right: IntPoint, center: IntPoint) -> Self {
        Self {
            width,
            length,
            left,
            right,
            center,
            left_device: usize::MAX,
            right_device: usize::MAX,
            gate_device: usize::MAX,
            body_devices: Vec::new(),
        }
    }
}

static EMPTY_DEVICE: &str = "None";

pub struct LvsSolver {
    cell_name: String,
    scale_to_physics: f64,
    poly: ShapeRes,
    diffusion: ShapeRes,
    poly_layer: LayerData,
    diffusion_layer: LayerData,
    transistors: Vec<LvsTransistor>,
    shapes_res: HashMap<LayerData, ShapeRes>,
    stack_binds: Vec<LayerBind>,
    mark_store: HashMap<LayerData, Vec<Mark>>,
    mark_binds: Vec<LayerBind>,
}

impl LvsSolver {
    pub fn new(
        cell_name: String,
        scale_to_physics: f64,
        poly: ShapeRes,
        diffusion: ShapeRes,
        poly_layer: LayerData,
        diffusion_layer: LayerData,
        transistors: Vec<LvsTransistor>,
        shapes_res: HashMap<LayerData, ShapeRes>,
        stack_binds: Vec<LayerBind>,
        mark_store: HashMap<LayerData, Vec<Mark>>,
        mark_binds: Vec<LayerBind>,
    ) -> Self {
        Self {
            cell_name,
            scale_to_physics,
            poly,
            diffusion,
            poly_layer,
            diffusion_layer,
            transistors,
            shapes_res,
            stack_binds,
            mark_store,
            mark_binds,
        }
    }

    pub fn find_problems(&self) -> Result<LvsFile, String> {
        let bundles = self.build_transistors()?;

        let labels = self.mark_store.values().flatten().map(|m| m.label.clone()).collect();

        log::info!("labels: {:?}", &labels);

        let lvs = self.generate_file(&self.cell_name, labels, bundles);

        Ok(lvs)
    }

    fn build_transistors(&self) -> Result<Vec<SiliconBundle>, String> {
        let origins = self.origins()?;

        let mut bundles = Vec::with_capacity(origins.len());

        let poly_shapes = self.poly.shapes();

        let mut start_index = 0;

        for origin in origins.iter() {
            let source_drain_and_poly_graph = Overlay::with_shapes(&origin.source_drain_shapes, &poly_shapes).into_graph(FillRule::NonZero);

            let gate_shapes = source_drain_and_poly_graph.extract_vectors(OverlayRule::Intersect);
            let source_drain_shapes = source_drain_and_poly_graph.extract_shapes(OverlayRule::Difference);

            let device_stack = self.build_stack(&origin);

            let mut semiconductors = self.semiconductors(gate_shapes);
            let shape_store = self.shapes_store(&origin, source_drain_shapes, origin.body_shapes.clone());

            let devices = Device::build(start_index, &shape_store, &device_stack, &self.mark_binds, &self.mark_store)?;

            start_index += devices.len();

            self.detect_source_drain(&mut semiconductors, &devices, &shape_store, origin.transistor.source_drain_implant);
            self.detect_gate(&mut semiconductors, &devices, &shape_store);
            self.detect_body(&origin, &mut semiconductors, &devices);

            let bundle = SiliconBundle {
                transistor: &origin.transistor,
                devices,
                semiconductors,
            };

            bundles.push(bundle);
        }

        Ok(bundles)
    }

    fn detect_gate(&self, semiconductors: &mut Vec<Semiconductor>, devices: &Vec<Device>, shape_store: &HashMap<LayerData, IntShapes>) {
        let mut id_points = semiconductors.iter().enumerate().map(|it| IdPoint::new(it.0, it.1.center)).collect();
        let pairs = devices.search(&mut id_points, shape_store, self.poly_layer);
        for pair in pairs.iter() {
            semiconductors[pair.point_id as usize].gate_device = pair.id as usize;
        }
    }

    fn detect_body(&self, origin: &Origin, semiconductors: &mut Vec<Semiconductor>, devices: &Vec<Device>) {
        let mut device_set = HashSet::new();
        for device in devices.iter() {
            if device.shapes.contains_key(&origin.transistor.body_implant) {
                device_set.insert(device.index);
            }
        }
        let body_devices: Vec<usize> = device_set.into_iter().collect();

        for semiconductor in semiconductors.iter_mut() {
            semiconductor.body_devices = body_devices.clone();
        }
    }

    fn detect_source_drain(
        &self,
        semiconductors: &mut Vec<Semiconductor>,
        devices: &Vec<Device>,
        shape_store: &HashMap<LayerData, IntShapes>,
        source_drain_implant: LayerData
    ) {
        let mut left_id_points = semiconductors.iter().enumerate().map(|it| IdPoint::new(it.0, it.1.left)).collect();
        let left_pairs = devices.search(&mut left_id_points, shape_store, source_drain_implant);
        for pair in left_pairs.iter() {
            semiconductors[pair.point_id as usize].left_device = pair.id as usize;
        }

        let mut right_id_points = semiconductors.iter().enumerate().map(|it| IdPoint::new(it.0, it.1.right)).collect();
        let right_pairs = devices.search(&mut right_id_points, shape_store, source_drain_implant);
        for pair in right_pairs.iter() {
            semiconductors[pair.point_id as usize].right_device = pair.id as usize;
        }
    }

    fn build_stack(&self, origin: &Origin) -> Vec<LayerBind> {
        let mut stack = Vec::with_capacity(self.stack_binds.len());
        for bind in self.stack_binds.iter() {
            if bind.contains(&self.diffusion_layer) {
                if bind.contains(&self.poly_layer) {
                    log::error!("stack_binds: poly and diffusion layers must not be connected!");
                }

                let other = bind.opposite(&self.diffusion_layer);

                let source_drain_bind: LayerBind = [origin.transistor.source_drain_implant, other.clone()];
                let body_bind: LayerBind = [origin.transistor.body_implant, other.clone()];

                stack.push(source_drain_bind);
                stack.push(body_bind);
            } else {
                stack.push(bind.clone());
            }
        }

        stack
    }

    fn shapes_store(&self, origin: &Origin, source_drain_shapes: IntShapes, body_shapes: IntShapes) -> HashMap<LayerData, IntShapes> {
        // replace diffusion layer with source/drain and body shapes
        let mut map = HashMap::with_capacity(self.shapes_res.len());

        for (layer, shape_res) in self.shapes_res.iter() {
            if layer.eq(&self.diffusion_layer) {
                continue;
            }

            let shapes = shape_res.shapes();
            map.insert(layer.clone(), shapes);
        }

        map.insert(origin.transistor.source_drain_implant, source_drain_shapes);
        map.insert(origin.transistor.body_implant, body_shapes);

        map
    }

    fn semiconductors(&self, shapes: Vec<VectorShape>) -> Vec<Semiconductor> {
        let mut gates = Vec::with_capacity(shapes.len());
        for shape in shapes.iter() {
            if shape.len() != 1 {
                continue;
            }

            let form = &shape[0];
            if form.len() != 4 {
                continue;
            }

            let is_vertical = form.is_vertical();
            let rect = form.rect();

            let center = IntPoint {
                x: ((rect.max_x as i64 + rect.min_x as i64) >> 1) as i32,
                y: ((rect.max_y as i64 + rect.min_y as i64) >> 1) as i32,
            };

            let gate = if is_vertical {
                let left = IntPoint { x: rect.min_x, y: center.y };
                let right = IntPoint { x: rect.max_x, y: center.y };
                let width: f64 = self.scale_to_physics * rect.height() as f64;
                let length: f64 = self.scale_to_physics * rect.width() as f64;
                Semiconductor::new(width, length, left, right, center)
            } else {
                let left = IntPoint { x: center.x, y: rect.max_y };
                let right = IntPoint { x: center.x, y: rect.min_y };
                let width: f64 = self.scale_to_physics * rect.height() as f64;
                let length: f64 = self.scale_to_physics * rect.width() as f64;
                Semiconductor::new(width, length, left, right, center)
            };

            gates.push(gate);
        }

        gates
    }

    fn generate_file(&self, cell_name: &String, labels: Vec<String>, bundles: Vec<SiliconBundle>) -> LvsFile {
        let mut devices = Vec::with_capacity(64 * bundles.len());
        for bundle in bundles.into_iter() {
            let mut sub_devices = bundle.into_lvs(devices.len());
            devices.append(&mut sub_devices);
        }

        let mut title = cell_name.clone();
        title.push(' ');
        title.push_str(&labels.join(" "));

        LvsFile { title, devices }
    }

    fn x_wells(&self) -> Result<Vec<IntShapes>, String> {
        let mut x_well_map = HashMap::with_capacity(self.transistors.len());
        for i in 0..self.transistors.len() {
            if let Some(x_well) = self.transistors[i].x_well {
                if let Some(res) = self.shapes_res.get(&x_well) {
                    x_well_map.insert(i, res.shapes());
                } else {
                    return Err(format!("X-well shapes is not found! Layer: {}", x_well));
                };
            }
        }

        if x_well_map.is_empty() {
            return Err("At least one x-well must be not null".to_string());
        }

        if x_well_map.len() == self.transistors.len() {
            return Ok(x_well_map.values().cloned().into_iter().collect());
        }

        let invert = Self::invert(&x_well_map);

        let mut x_well_shapes = Vec::with_capacity(self.transistors.len());

        for i in 0..self.transistors.len() {
            if let Some(shapes) = x_well_map.get(&i) {
                x_well_shapes.push(shapes.clone())
            } else {
                x_well_shapes.push(invert.clone())
            }
        }

        Ok(x_well_shapes)
    }

    fn invert(x_well_map: &HashMap<usize, IntShapes>) -> IntShapes {
        let boundary = Self::boundary(x_well_map);
        let mut subj = [[[
            IntPoint { x: boundary.min_x, y: boundary.min_y },
            IntPoint { x: boundary.min_x, y: boundary.max_y },
            IntPoint { x: boundary.max_x, y: boundary.max_y },
            IntPoint { x: boundary.max_x, y: boundary.min_y },
        ].to_vec()].to_vec()].to_vec();

        for shapes in x_well_map.values() {
            subj = subj.sub(shapes);
        }

        subj
    }

    fn boundary(map: &HashMap<usize, IntShapes>) -> IntRect {
        let a = i32::MAX / 2;
        let mut boundary = IntRect {
            min_x: -a,
            max_x: a,
            min_y: -a,
            max_y: a,
        };

        for shapes in map.values() {
            let rect = IntRect::with_shapes(shapes);
            boundary = IntRect::with_rects(&boundary, &rect);
        }

        boundary
    }

    fn origins(&self) -> Result<Vec<Origin>, String> {
        let x_wells = self.x_wells()?;
        let mut origins = Vec::with_capacity(self.transistors.len());
        let diffusion_shapes = self.diffusion.shapes();
        for (index, transistor) in self.transistors.iter().enumerate() {
            let source_drain_implant_shapes = if let Some(res) = self.shapes_res.get(&transistor.source_drain_implant) {
                res.shapes()
            } else {
                return Err(format!("source_drain_implant shapes is not found! Layer: {}", transistor.source_drain_implant));
            };
            let body_implant_shapes = if let Some(res) = self.shapes_res.get(&transistor.body_implant) {
                res.shapes()
            } else {
                return Err(format!("body_implant shapes is not found! Layer: {}", transistor.body_implant));
            };

            let x_well = &x_wells[index];

            let dif_and_x_well = diffusion_shapes.and(x_well);

            let source_drain_shapes = dif_and_x_well.and(&source_drain_implant_shapes);
            let body_shapes = dif_and_x_well.and(&body_implant_shapes);

            origins.push(Origin {
                transistor,
                source_drain_shapes,
                body_shapes,
            });
        }

        Ok(origins)
    }
}


trait Form {
    fn is_vertical(&self) -> bool;
    fn rect(&self) -> IntRect;
}

const SUBJ_BOTH: u8 = 0b0011;

impl Form for VectorPath {
    fn is_vertical(&self) -> bool {
        for edge in self.iter() {
            if edge.fill & SUBJ_BOTH == SUBJ_BOTH {
                let dx = (edge.a.x - edge.b.x).abs();
                let dy = (edge.a.y - edge.b.y).abs();
                return dx < dy;
            }
        }

        false
    }

    fn rect(&self) -> IntRect {
        let mut max_x = self[0].a.x;
        let mut min_x = self[0].a.x;
        let mut max_y = self[0].a.y;
        let mut min_y = self[0].a.y;

        for i in 1..4 {
            let a = self[i].a;
            max_x = max_x.max(a.x);
            min_x = min_x.min(a.x);
            max_y = max_y.max(a.y);
            min_y = min_y.min(a.y);
        }

        IntRect {
            min_x,
            max_x,
            min_y,
            max_y,
        }
    }
}

impl SiliconBundle<'_> {
    fn into_lvs(self, counter: usize) -> Vec<LvsDevice> {
        let mut list: Vec<LvsDevice> = Vec::with_capacity(self.semiconductors.len());
        let mut index = counter;
        for s in self.semiconductors.iter() {
            let left = if s.left_device < self.devices.len() {
                self.devices[s.left_device].name()
            } else {
                EMPTY_DEVICE.to_string()
            };

            let right = if s.right_device < self.devices.len() {
                self.devices[s.right_device].name()
            } else {
                EMPTY_DEVICE.to_string()
            };

            assert!(s.gate_device < self.devices.len());

            let gate= self.devices[s.gate_device].name();

            let id = format!("{}{}", self.transistor.id_prefix, index);
            index += 1;

            let body = if s.body_devices.is_empty() {
                self.transistor.power_supply.clone()
            } else {
                let mut body_devices = String::new();
                for &id in s.body_devices.iter() {
                    let device = &self.devices[id];
                    if !body_devices.is_empty() {
                        body_devices.push('_');
                    }
                    if device.labels.is_empty() {
                        body_devices.push_str(&self.transistor.power_supply);
                    } else {
                        body_devices.push_str(&device.name());
                    }
                }

                body_devices
            };

            let item = LvsDevice {
                id,
                left,
                right,
                gate,
                body,
                transistor: self.transistor.name.clone(),
                width: s.width,
                length: s.length,
            };

            list.push(item);
        }

        list
    }
}