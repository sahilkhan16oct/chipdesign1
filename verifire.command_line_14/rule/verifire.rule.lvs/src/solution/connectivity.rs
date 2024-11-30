use std::collections::{HashMap, HashSet};
use foundation::export::convert::PathToExpShape;
use foundation::fix::layer::LayerData;
use foundation::fix::layer_binding::LayerBind;
use foundation::fix::mark::Mark;
use foundation::i_overlay::i_shape::int::shape::IntShapes;
use foundation::res::shape_res::ShapeRes;
use foundation::res::store::StoreRes;
use foundation::task::task::Content;
use crate::connectivity::device::{Cell, Device, Label};
use crate::connectivity::problems::{ConnectivityProblem, OpenError, OpenWarning, ShortError, ShortWarning};

type DeviceIndex = usize;

pub struct ConnectivitySolver {
    shapes_res: HashMap<LayerData, ShapeRes>,
    stack_binds: Vec<LayerBind>,
    mark_store: HashMap<LayerData, Vec<Mark>>,
    mark_binds: Vec<LayerBind>,
    only_errors: bool,
}

impl ConnectivitySolver {
    pub fn new(
        shapes_res: HashMap<LayerData, ShapeRes>,
        stack_binds: Vec<LayerBind>,
        mark_store: HashMap<LayerData, Vec<Mark>>,
        mark_binds: Vec<LayerBind>,
        only_errors: bool,
    ) -> Self {
        Self { shapes_res, stack_binds, mark_store, mark_binds, only_errors }
    }

    pub fn find_problems(&self) -> Result<Vec<Content>, String> {
        let shape_store = self.shapes_store();

        let devices = Device::build(0, &shape_store, &self.stack_binds, &self.mark_binds, &self.mark_store)?;

        let problems_map = Self::map_problems(&devices);

        let result = self.generate_result(devices, problems_map, &shape_store)?;

        Ok(result)
    }

    fn map_problems(devices: &Vec<Device>) -> HashMap<usize, Vec<ConnectivityProblem>> {
        let mut problems_map = HashMap::new();

        // short problems
        for device in devices.iter() {
            if device.labels.len() < 2 {
                continue;
            }

            for (i, it0) in device.labels.iter().enumerate() {
                for (j, it1) in device.labels.iter().enumerate() {
                    if i < j {
                        let problems = problems_map.entry(device.index).or_insert(Vec::with_capacity(1));
                        if it0.1.is_disjoint(it1.1) {
                            let short_warning = ShortWarning {
                                label_a: it0.0.clone(),
                                label_b: it1.0.clone(),
                            };

                            problems.push(ConnectivityProblem::ShortWarning(short_warning));
                        } else {
                            let short_error = ShortError {
                                label_a: it0.0.clone(),
                                label_b: it1.0.clone(),
                            };

                            problems.push(ConnectivityProblem::ShortError(short_error));
                        }
                    }
                }
            }
        }

        // open problems
        let mut map_by_labels: HashMap<Label, Vec<usize>> = HashMap::new();

        for device in devices.iter() {
            for label in device.labels.keys() {
                let map_by_devices = map_by_labels.entry(label.clone()).or_insert(Vec::with_capacity(2));
                map_by_devices.push(device.index);
            }
        }

        let mut visited_labels = HashSet::new();

        for target in devices.iter() {
            for label in target.labels.keys() {
                if !visited_labels.insert(label.clone()) {
                    continue;
                }

                if let Some(indices) = map_by_labels.get(label) {
                    for &index in indices.iter() {
                        if index == target.index {
                            continue;
                        }

                        let other = &devices[index];

                        let target_cells = target.labels.get(label).unwrap();
                        let other_cells = other.labels.get(label).unwrap();

                        let common: Vec<Cell> = target_cells.intersection(other_cells).cloned().collect();

                        if common.is_empty() {
                            problems_map.add(target.index, ConnectivityProblem::OpenWarning(OpenWarning {
                                label: label.clone(),
                                device: other.index,
                            }));
                            problems_map.add(other.index, ConnectivityProblem::OpenWarning(OpenWarning {
                                label: label.clone(),
                                device: target.index,
                            }));
                        } else {
                            problems_map.add(target.index, ConnectivityProblem::OpenError(OpenError {
                                label: label.clone(),
                                cells: common.clone(),
                                device: other.index,
                            }));
                            problems_map.add(other.index, ConnectivityProblem::OpenError(OpenError {
                                label: label.clone(),
                                cells: common,
                                device: target.index,
                            }));
                        }
                    }
                }
            }
        }

        problems_map
    }

    fn generate_result(
        &self, devices: Vec<Device>,
        problems_map: HashMap<usize, Vec<ConnectivityProblem>>,
        shape_store: &HashMap<LayerData, IntShapes>,
    ) -> Result<Vec<Content>, String> {
        let mut result = Vec::with_capacity(devices.len());
        for device in devices.iter() {
            let problems = problems_map.get(&device.index);

            if self.only_errors && problems.is_none() {
                continue;
            }

            let mut e_shapes = Vec::new();
            for (layer, indices) in device.shapes.iter() {
                let shapes = shape_store.shapes(layer)?;
                for &index in indices.iter() {
                    e_shapes.append(&mut shapes[index].clone().into_exp_shapes())
                }
            }

            let mut text = format!("[{}]", device.index);

            if !device.labels.is_empty() {
                text.push_str(" labels: (");
                let mut i = 0;
                let last_index = device.labels.len() - 1;
                let mut tags = String::new();
                for (label, cells_set) in device.labels.iter() {
                    let cells = cells_set.iter().cloned().collect::<Vec<_>>().join(", ");
                    tags.push_str(format!("{}: {}", label, cells).as_str());
                    if i != last_index {
                        tags.push_str(", ");
                        i += 1;
                    }
                }
                text.push_str(&tags);
                text.push_str(")");
            };

            if let Some(problems) = problems {
                let str_problems = Self::problems_as_string(problems);
                text.push_str(&str_problems);
            }

            result.push(Content { text, shapes: e_shapes });
        }

        Ok(result)
    }

    fn shapes_store(&self) -> HashMap<LayerData, IntShapes> {
        let mut map = HashMap::with_capacity(self.shapes_res.len());

        for (layer, shape_res) in self.shapes_res.iter() {
            let shapes = shape_res.shapes();
            map.insert(layer.clone(), shapes);
        }

        map
    }

    fn problems_as_string(problems: &Vec<ConnectivityProblem>) -> String {
        let mut open_errors = Vec::new();
        let mut open_warnings = Vec::new();
        let mut short_errors = Vec::new();
        let mut short_warnings = Vec::new();

        for problem in problems.iter() {
            match problem {
                ConnectivityProblem::OpenError(data) => {
                    open_errors.push(data.to_string());
                }
                ConnectivityProblem::OpenWarning(data) => {
                    open_warnings.push(data.to_string());
                }
                ConnectivityProblem::ShortError(data) => {
                    short_errors.push(data.to_string());
                }
                ConnectivityProblem::ShortWarning(data) => {
                    short_warnings.push(data.to_string());
                }
            }
        }

        let mut text = String::new();

        if !open_errors.is_empty() {
            text.push_str(&format!("; open errors({}): [{}]", open_errors.len(), open_errors.join(", ")));
        }

        if !open_warnings.is_empty() {
            text.push_str(&format!("; open warnings({}): [{}]", open_warnings.len(), open_warnings.join(", ")));
        }

        if !short_errors.is_empty() {
            text.push_str(&format!("; short errors({}): [{}]", short_errors.len(), short_errors.join(", ")));
        }

        if !short_warnings.is_empty() {
            text.push_str(&format!("; short warnings({}): [{}]", short_warnings.len(), short_warnings.join(", ")));
        }

        text
    }
}

trait Problems {
    fn add(&mut self, device: DeviceIndex, problem: ConnectivityProblem);
}

impl Problems for HashMap<usize, Vec<ConnectivityProblem>> {
    fn add(&mut self, device: DeviceIndex, problem: ConnectivityProblem) {
        self.entry(device).or_insert(Vec::with_capacity(1)).push(problem);
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};
    use crate::connectivity::device::{Cell, Device, Label};
    use crate::connectivity::problems::ConnectivityProblem;
    use crate::solution::connectivity::ConnectivitySolver;

    impl Device {
        fn with_labels(index: usize, labels: HashMap<Label, HashSet<Cell>>) -> Self {
            Self {
                index,
                labels,
                shapes: Default::default(),
            }
        }
    }

    impl ConnectivityProblem {
        fn is_open_error(&self) -> bool {
            match self {
                ConnectivityProblem::OpenError(_) => {
                    true
                }
                _ => { false }
            }
        }
        fn is_open_warning(&self) -> bool {
            match self {
                ConnectivityProblem::OpenWarning(_) => {
                    true
                }
                _ => { false }
            }
        }
        fn is_short_error(&self) -> bool {
            match self {
                ConnectivityProblem::ShortError(_) => {
                    true
                }
                _ => { false }
            }
        }
        fn is_short_warning(&self) -> bool {
            match self {
                ConnectivityProblem::ShortWarning(_) => {
                    true
                }
                _ => { false }
            }
        }
    }

    #[test]
    fn test_validate_problems_0() {
        let labels_0 = vec![
            ("Label_A".to_string(), HashSet::from(["Cell_A".to_string()])),
            ("Label_A".to_string(), HashSet::from(["Cell_B".to_string()])),
            ("Label_A".to_string(), HashSet::from(["Cell_C".to_string()])),
        ].into_iter().collect();

        let devices = vec![
            Device::with_labels(0, labels_0)
        ];

        let problems_map = ConnectivitySolver::map_problems(&devices);

        assert_eq!(problems_map.is_empty(), true);
    }

    #[test]
    fn test_validate_problems_1() {
        let labels_0 = vec![
            ("Label_A".to_string(), HashSet::from(["Cell_A".to_string()])),
            ("Label_B".to_string(), HashSet::from(["Cell_A".to_string()])),
        ].into_iter().collect();

        let devices = vec![
            Device::with_labels(0, labels_0)
        ];

        let problems_map = ConnectivitySolver::map_problems(&devices);

        for device in devices.iter() {
            let problems = problems_map.get(&device.index).unwrap();
            assert!(problems[0].is_short_error());
        }
    }

    #[test]
    fn test_validate_problems_2() {
        let labels_0 = vec![
            ("Label_A".to_string(), HashSet::from(["Cell_A".to_string()])),
            ("Label_B".to_string(), HashSet::from(["Cell_B".to_string()])),
        ].into_iter().collect();

        let devices = vec![
            Device::with_labels(0, labels_0)
        ];

        let problems_map = ConnectivitySolver::map_problems(&devices);

        for device in devices.iter() {
            let problems = problems_map.get(&device.index).unwrap();
            assert!(problems[0].is_short_warning());
        }
    }

    #[test]
    fn test_validate_problems_3() {
        let labels_0 = vec![
            ("Label_A".to_string(), HashSet::from(["Cell_A".to_string()]))
        ].into_iter().collect();

        let labels_1 = vec![
            ("Label_A".to_string(), HashSet::from(["Cell_A".to_string()]))
        ].into_iter().collect();

        let devices = vec![
            Device::with_labels(0, labels_0),
            Device::with_labels(1, labels_1),
        ];

        let problems_map = ConnectivitySolver::map_problems(&devices);

        for device in devices.iter() {
            let problems = problems_map.get(&device.index).unwrap();
            assert!(problems[0].is_open_error());
        }
    }

    #[test]
    fn test_validate_problems_4() {
        let labels_0 = vec![
            ("Label_A".to_string(), HashSet::from(["Cell_A".to_string()]))
        ].into_iter().collect();

        let labels_1 = vec![
            ("Label_A".to_string(), HashSet::from(["Cell_B".to_string()]))
        ].into_iter().collect();

        let devices = vec![
            Device::with_labels(0, labels_0),
            Device::with_labels(1, labels_1),
        ];

        let problems_map = ConnectivitySolver::map_problems(&devices);

        for device in devices.iter() {
            let problems = problems_map.get(&device.index).unwrap();
            assert!(problems[0].is_open_warning());
        }
    }

    #[test]
    fn test_validate_problems_5() {
        let labels_0 = vec![
            ("Label_A".to_string(), HashSet::from(["Cell_A".to_string(), "Cell_B".to_string()]))
        ].into_iter().collect();

        let labels_1 = vec![
            ("Label_B".to_string(), HashSet::from(["Cell_B".to_string(), "Cell_C".to_string()]))
        ].into_iter().collect();

        let labels_2 = vec![
            ("Label_C".to_string(), HashSet::from(["Cell_C".to_string(), "Cell_A".to_string()]))
        ].into_iter().collect();

        let devices = vec![
            Device::with_labels(0, labels_0),
            Device::with_labels(1, labels_1),
            Device::with_labels(2, labels_2),
        ];

        let problems_map = ConnectivitySolver::map_problems(&devices);
        assert_eq!(problems_map.is_empty(), true);
    }
}