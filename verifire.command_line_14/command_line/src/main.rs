mod task;
mod action;
mod output;

use std::{env, fs};
use std::path::Path;
use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::time::Instant;
use uuid::Uuid;
use tokio::sync::mpsc;
use futures::future::join_all;
use serde_json::to_string_pretty;
use design_format::fix::doc::FixDoc;
use design_format::fix::layer::LayerData;
use design_format::fix::layer_chain::ToLayerBinds;
use design_format::fix::mark::Mark;
use design_format::fix::scaler::UnitScaler;
use foundation::export::err_file::Violation;
use foundation::export::lvs_file::LvsFile;
use foundation::export::rve_file::RveFile;
use foundation::i_overlay::i_shape::int::shape::IntShapes;
use foundation::res::store::StoreRes;
use foundation::task::task::{VerificationProgress, VerificationResult};
use rule_runner::rule::angle_task::AngleTask;
use rule_runner::rule::check_shape_task::CheckShapeTask;
use rule_runner::rule::connectivity_task::ConnectivityTask;
use rule_runner::rule::density_task::DensityTask;
use rule_runner::rule::exact_size_task::ExactSizeTask;
use rule_runner::rule::extension_task::ExtensionTask;
use rule_runner::rule::filter_task::FilterTask;
use rule_runner::rule::lvs_task::LvsTask;
use rule_runner::rule::min_area_task::MinAreaTask;
use rule_runner::rule::min_max_width_task::MinMaxWidthTask;
use rule_runner::rule::notch_task::NotchTask;
use rule_runner::rule::off_grid_task::OffGridTask;
use rule_runner::rule::poly_enclosure_task::PolyEnclosureTask;
use rule_runner::rule::rect_enclosure_task::RectEnclosureTask;
use rule_runner::rule::slit_length::SlitLengthTask;
use rule_runner::rule::space_task::SpaceTask;
use rule_runner::task::rule::{RuleTask, VerificationTask};
use slice::solution::slice::Slice;
use crate::action::doc::{ActionDef, ActionDoc};
use crate::output::doc::{ErrorDoc, ResultCell, ResultRule};
use crate::task::doc::{TaskDoc, TaskDef, Format};


#[cfg(debug_assertions)]
fn set_debug_data(args_map: &mut HashMap<String, String>) {
    // args_map.insert("file".to_string(), "command_line/test_runner/debug/connectivity/test_0/test_0.json".to_string());
    // args_map.insert("file".to_string(), "command_line/test_runner/debug/lvs/test_0/test_0.json".to_string());
    args_map.insert("file".to_string(), "command_line/test_runner/debug/lvs/test_1/test_1.json".to_string());
    // args_map.insert("file".to_string(), "command_line/test_runner/debug/min_max_width/test_0/test_0.json".to_string());
}

#[tokio::main]
async fn main() {
    env_logger::Builder::from_default_env()
        .filter(None, log::LevelFilter::Info) // Set log level to Info
        .init();

    let path = match read_console() {
        None => { return; }
        Some(path) => { path }
    };

    let file = Path::new(&path);

    if let Some(file_name) = file.file_name() {
        if let Some(file_name_str) = file_name.to_str() {
            log::info!("");
            log::info!("--- {file_name_str} ---");
        }
    }

    if TaskDoc::is_valid(file) {
        run_tasks(file).await;
    } else if ActionDoc::is_valid(file) {
        run_actions(file);
    }
}

async fn run_tasks(file: &Path) {
    let task_doc = TaskDoc::load(file);
    let gds_path = Path::new(&task_doc.input);
    validate_path(gds_path);

    let mut fix_doc = match FixDoc::load_from_file(gds_path) {
        Ok(doc) => {
            doc
        }
        Err(error) => {
            log::error!("{error}");
            panic!("Can not open file");
        }
    };

    // union start
    let time_0 = Instant::now();

    // prepare layers for tasks
    let shape_layers = task_doc.shape_layers();

    // expensive operation
    let shape_store = match fix_doc.cell_shape_store(&task_doc.cell, &shape_layers) {
        Ok(store) => { store }
        Err(error) => {
            log::error!("{error}");
            panic!("Can not open file");
        }
    };

    let mark_layers = task_doc.mark_layers();

    log::info!("mark_layers: {:?}", &mark_layers);

    let mark_store = match fix_doc.cell_mark_store(&task_doc.cell, &mark_layers) {
        Ok(store) => { store }
        Err(error) => {
            log::error!("{error}");
            panic!("Can not open file");
        }
    };

    log::info!("mark_store: {:?}", &mark_store);

    let time_1 = Instant::now();
    let union_time = time_1.duration_since(time_0);
    log::info!("Union time elapsed: {:?}", union_time);

    // create
    let (tasks, titles) = create(&shape_store, &mark_store, fix_doc.scaler, &task_doc);

    // run

    let (sender, mut receiver) = mpsc::channel::<VerificationProgress<Uuid>>(1);

    let progress_task = tokio::spawn(async move {
        while let Some(progress) = receiver.recv().await {
            log::info!("Id: {}, Progress: {:.1}", progress.task_id, 100.0 * progress.progress);
        }
    });

    let futures = tasks.into_iter().map(|task| {
        let task_sender = sender.clone();
        tokio::spawn(async move {
            task.run(task_sender).await
        })
    });

    let future_results = join_all(futures).await;

    let results: HashMap<Uuid, VerificationResult> = future_results.into_iter().filter_map(|result| {
        match result {
            Ok(result) => Some((result.id, result.result)),
            Err(e) => {
                log::error!("A task failed to complete: {}", e);
                None
            }
        }
    }).collect();

    drop(sender);

    progress_task.await.unwrap();

    let time_2 = Instant::now();
    let tasks_time = time_2.duration_since(time_1);
    log::info!("Tasks time elapsed: {:?}", tasks_time);

    // save
    save(&task_doc, results, titles, fix_doc.scaler);

    let time_3 = Instant::now();
    let save_time = time_3.duration_since(time_2);
    log::info!("Save time elapsed: {:?}", save_time);

    let all_time = time_3.duration_since(time_0);
    log::info!("Full time elapsed: {:?}", all_time);
}

fn run_actions(file: &Path) {
    let action_doc = ActionDoc::load(file);
    let target_path = Path::new(&action_doc.file);
    validate_path(target_path);

    let mut fix_doc = match FixDoc::load_from_file(target_path) {
        Ok(doc) => {
            doc
        }
        Err(error) => {
            log::error!("{error}");
            panic!("Can not open file");
        }
    };

    for def in action_doc.actions {
        match def {
            ActionDef::Slice(slice_def) => {
                log::info!("Start: {}", slice_def.title());
                fix_doc.slice(
                    slice_def.slice_type,
                    slice_def.orientation,
                    &slice_def.cell,
                    slice_def.layer_data,
                    slice_def.self_only,
                )
            }
        }
    }

    match fix_doc.save(target_path, true) {
        Ok(_) => {
            log::info!("Saved: {:?}", target_path);
        }
        Err(e) => {
            log::error!("Can not save file. {e}");
        }
    }
}

fn read_console() -> Option<String> {
    let args = env::args();
    let mut args_iter = args.peekable();
    let mut args_map = HashMap::new();

    while let Some(arg) = args_iter.next() {
        if arg.starts_with("--") {
            let key = arg.trim_start_matches("--").to_owned();
            // If the next argument is also a key, store a boolean flag; otherwise, store the value.
            let value = if args_iter.peek().map_or(false, |a| a.starts_with("--")) {
                "true".to_string()
            } else {
                args_iter.next().unwrap()
            };
            args_map.insert(key, value);
        }
    }

    #[cfg(debug_assertions)]
    {
        if args_map.is_empty() {
            set_debug_data(&mut args_map);
        }
    }

    if args_map.is_empty() || args_map.contains_key("help") {
        log::info!("Usage:");
        log::info!("--file <path> : Config file with rule");
        return None;
    }

    let file = args_map.get("file")?;
    let file_path = Path::new(file);
    validate_path(file_path);

    Some(file.clone())
}

fn create(shape_store: &HashMap<LayerData, IntShapes>, mark_store: &HashMap<LayerData, Vec<Mark>>, scaler: UnitScaler, document: &TaskDoc) -> (Vec<VerificationTask<Uuid>>, HashMap<Uuid, String>) {
    let mut tasks = Vec::new();
    let mut map: HashMap<Uuid, String> = HashMap::new();

    for def in document.tasks.iter() {
        let task_id = Uuid::new_v4();
        map.insert(task_id, def.title());
        match create_task(&document.cell, def, task_id, shape_store, mark_store, scaler) {
            Ok(task) => {
                tasks.push(task);
            }
            Err(err) => {
                log::error!("Task will be skipped {}. {}", def.title(), err);
            }
        }
    }

    (tasks, map)
}

fn create_task(cell_name: &String, def: &TaskDef, task_id: Uuid, shape_store: &HashMap<LayerData, IntShapes>, mark_store: &HashMap<LayerData, Vec<Mark>>, scaler: UnitScaler) -> Result<VerificationTask<Uuid>, String> {
    match def {
        TaskDef::Angle(data) => {
            let res = shape_store.resource_by_viewport(&data.viewport)?;
            let task = AngleTask::new(res, data.is_45_allowed.unwrap_or(false));
            Ok(VerificationTask::new(task_id, RuleTask::Angle(task)))
        }
        TaskDef::CheckShape(data) => {
            let res = shape_store.resource_by_viewport(&data.viewport)?;
            let task = CheckShapeTask::new(res, data.form);
            Ok(VerificationTask::new(task_id, RuleTask::CheckShape(task)))
        }
        TaskDef::Connectivity(data) => {
            let shapes_res = data.shapes_res(shape_store)?;
            let task = ConnectivityTask::new(
                shapes_res,
                data.layer_stack.layers(),
                mark_store.clone(),
                data.labels.layers(),
                data.only_errors.unwrap_or(true),
            );
            Ok(VerificationTask::new(task_id, RuleTask::Connectivity(task)))
        }
        TaskDef::Density(data) => {
            let res = shape_store.resource_by_viewport(&data.viewport)?;
            let window_width = scaler.user_value_to_unit(data.window_width);
            let window_height = scaler.user_value_to_unit(data.window_height);

            let task = DensityTask::new(res, data.density, data.sampling_level, window_width, window_height, data.result_type, data.is_debug)?;
            Ok(VerificationTask::new(task_id, RuleTask::Density(task)))
        }
        TaskDef::Lvs(data) => {
            let shapes_res = data.shapes_res(shape_store)?;
            let poly_res = shape_store.resource_by_viewport(&data.poly)?;
            let diffusion_res = shape_store.resource_by_viewport(&data.diffusion)?;
            let task = LvsTask::new(
                cell_name.clone(),
                scaler.db_size,
                poly_res,
                diffusion_res,
                data.poly.base,
                data.diffusion.base,
                data.transistors.clone(),
                shapes_res,
                data.layer_stack.layers(),
                mark_store.clone(),
                data.labels.layers(),
            );
            Ok(VerificationTask::new(task_id, RuleTask::Lvs(task)))
        }
        TaskDef::PolyEnclosure(data) => {
            let float_res = shape_store.resource_by_viewport(&data.outer)?;
            let poly_res = shape_store.resource_by_viewport(&data.inner)?;
            let value = scaler.user_value_to_unit(data.value);

            let task = PolyEnclosureTask::new(float_res, poly_res, value, data.direction);
            Ok(VerificationTask::new(task_id, RuleTask::PolyEnclosure(task)))
        }
        TaskDef::RectEnclosure(data) => {
            let float_res = shape_store.resource_by_viewport(&data.outer)?;
            let rects_res = shape_store.resource_by_viewport(&data.rects)?;
            let value_0 = scaler.user_value_to_unit(data.value_0);
            let value_1 = if let Some(value) = data.value_1 {
                Some(scaler.user_value_to_unit(value))
            } else {
                None
            };
            let task = RectEnclosureTask::new(data.expression.as_str(), float_res, rects_res, value_0, value_1, data.only_inner)?;
            Ok(VerificationTask::new(task_id, RuleTask::RectEnclosure(task)))
        }
        TaskDef::ExactSize(data) => {
            let res = shape_store.resource_by_viewport(&data.viewport)?;
            let width = scaler.user_value_to_unit(data.width);
            let height = scaler.user_value_to_unit(data.height);
            let task = ExactSizeTask::new(res, width, height);
            Ok(VerificationTask::new(task_id, RuleTask::ExactSize(task)))
        }
        TaskDef::Extension(data) => {
            let poly = shape_store.resource_by_viewport(&data.poly)?;
            let diffusion = shape_store.resource_by_viewport(&data.diffusion)?;
            let value = scaler.user_value_to_unit(data.value);
            let task = ExtensionTask::new(poly, diffusion, value);
            Ok(VerificationTask::new(task_id, RuleTask::Extension(task)))
        }
        TaskDef::Filter(data) => {
            let res = shape_store.resource_by_viewport(&data.viewport)?;
            let task = FilterTask::new(res);
            Ok(VerificationTask::new(task_id, RuleTask::Filter(task)))
        }
        TaskDef::MinArea(data) => {
            let res = shape_store.resource_by_viewport(&data.viewport)?;
            let min_area = scaler.user_sqr_value_to_unit(data.min_area);
            let task = MinAreaTask::new(res, min_area);
            Ok(VerificationTask::new(task_id, RuleTask::MinArea(task)))
        }
        TaskDef::MinMaxWidth(data) => {
            let res = shape_store.resource_by_viewport(&data.viewport)?;
            let value = scaler.user_value_to_unit(data.value);
            let task = MinMaxWidthTask::new(res, data.algorithm, value);
            Ok(VerificationTask::new(task_id, RuleTask::MinMaxWidth(task)))
        }
        TaskDef::Notch(data) => {
            let res = shape_store.resource_by_viewport(&data.viewport)?;
            let min_length = scaler.user_value_to_unit(data.min_length);
            let task = NotchTask::new(res, min_length);
            Ok(VerificationTask::new(task_id, RuleTask::Notch(task)))
        }
        TaskDef::OffGrid(data) => {
            let res = shape_store.resource_by_viewport(&data.viewport)?;
            let step = scaler.user_value_to_unit(data.step);
            let task = OffGridTask::new(res, step);
            Ok(VerificationTask::new(task_id, RuleTask::OffGrid(task)))
        }
        TaskDef::SlitLength(data) => {
            let res = shape_store.resource_by_viewport(&data.viewport)?;
            let value = scaler.user_value_to_unit(data.value);
            let task = SlitLengthTask::new(res, data.algorithm, value);
            Ok(VerificationTask::new(task_id, RuleTask::SlitLength(task)))
        }
        TaskDef::Spacing(data) => {
            let spacing = scaler.user_value_to_unit(data.spacing);
            let shape_res_0 = shape_store.resource_by_viewport(&data.viewport_0)?;
            let shape_res_1 = if let Some(viewport) = &data.viewport_1 {
                Some(shape_store.resource_by_viewport(viewport)?)
            } else {
                None
            };

            let task = SpaceTask::new(shape_res_0, shape_res_1, data.direction, spacing);
            Ok(VerificationTask::new(task_id, RuleTask::SpaceOneLayer(task)))
        }
    }
}


fn save(
    task_doc: &TaskDoc,
    map: HashMap<Uuid, VerificationResult>,
    titles: HashMap<Uuid, String>,
    scaler: UnitScaler,
) {
    let file_path = Path::new(&task_doc.output);
    if file_path.exists() {
        match fs::remove_file(file_path) {
            Ok(_) => {
                log::info!("Deleted: {}", task_doc.output);
            }
            Err(e) => {
                log::error!("Can not delete old rve file. {e}");
            }
        }
    }

    match task_doc.format {
        Format::RVE => {
            save_rve(file_path, &task_doc.lvs_output, map, titles, task_doc.cell.clone(), scaler.inv_unit);
        }
        Format::JSON => {
            save_json(file_path, &task_doc.lvs_output, map, titles, task_doc.cell.clone(), scaler);
        }
    }
}

fn save_rve(
    file_path: &Path,
    lvs_output: &Option<String>,
    map: HashMap<Uuid, VerificationResult>,
    titles: HashMap<Uuid, String>,
    cell: String,
    scale: f64,
) {
    let mut rve = RveFile::new(file_path);
    rve.start(&cell, scale as usize);

    let empty = "Empty".to_string();
    for (id, result) in map.iter() {
        let title = titles.get(id).unwrap_or(&empty);
        match result {
            VerificationResult::Content(content) => {
                for content in content.iter() {
                    rve.add_section(&content.text, None, &content.shapes);
                }
            }
            VerificationResult::Shape(shapes) => {
                rve.add_section(title, None, shapes);
            }
            VerificationResult::Fail(error) => {
                log::error!("Fail({}): {}", error, title)
            }
            VerificationResult::NoErrors => {
                log::info!("No data to save: {}", title)
            }
            VerificationResult::Lvs(lvs) => {
                if let Some(path) = &lvs_output {
                    save_lvs(path, lvs);
                } else {
                    log::error!("Lvs path not set");
                }
            }
        }
    }

    match rve.finish() {
        Ok(_) => {
            log::info!("Saved");
        }
        Err(e) => {
            log::error!("Can not save rve file. {e}");
        }
    }
}

fn save_json(
    file_path: &Path,
    lvs_output: &Option<String>,
    map: HashMap<Uuid, VerificationResult>,
    titles: HashMap<Uuid, String>,
    cell_name: String,
    scaler: UnitScaler,
) {
    let mut rules = Vec::new();

    for (id, result) in map.iter() {
        let empty = "Empty".to_string();
        let title = titles.get(id).unwrap_or(&empty);
        match result {
            VerificationResult::Content(content) => {
                for content in content.iter() {
                    let violations = content.shapes.iter().map(|s| Violation::with_shape(s, &scaler)).collect();
                    rules.push(ResultRule {
                        rule_id: id.to_string(),
                        violations,
                    });
                }
            }
            VerificationResult::Shape(shapes) => {
                let violations = shapes.iter().map(|s| Violation::with_shape(s, &scaler)).collect();
                rules.push(ResultRule {
                    rule_id: id.to_string(),
                    violations,
                });
            }
            VerificationResult::Fail(error) => {
                log::error!("Fail({}): {}", error, title)
            }
            VerificationResult::NoErrors => {
                log::info!("No data to save: {}", title)
            }
            VerificationResult::Lvs(lvs) => {
                if let Some(path) = &lvs_output {
                    save_lvs(path, lvs);
                } else {
                    log::error!("Lvs path not set");
                }
            }
        }
    }

    let output = ErrorDoc {
        dimension: scaler.inv_unit,
        cells: [ResultCell {
            cell_name,
            rules,
        }].to_vec(),
    };


    let json = match to_string_pretty(&output) {
        Ok(json) => { json }
        Err(e) => {
            log::error!("Can not serialize. {e}");
            return;
        }
    };
    let mut file = match File::create(file_path) {
        Ok(file) => { file }
        Err(e) => {
            log::error!("Can not save create file. {e}");
            return;
        }
    };

    match file.write_all(json.as_bytes()) {
        Ok(file) => { file }
        Err(e) => {
            log::error!("Can not save json file. {e}");
            return;
        }
    };
}

fn save_lvs(file: &String, lvs: &LvsFile) {
    let file_path = Path::new(file);
    if file_path.exists() {
        match fs::remove_file(file_path) {
            Ok(_) => {
                log::info!("Deleted: {file}");
            }
            Err(e) => {
                log::error!("Can not delete old lvs file. {e}");
            }
        }
    }
    lvs.save(file_path);
}

fn validate_path(path: &Path) {
    if !path.exists() {
        log::warn!("The file is not exist: {}", path.display());
        match env::current_dir() {
            Ok(path) => log::info!("Current working directory is: {}", path.display()),
            Err(e) => log::error!("Error getting current directory: {}", e),
        }
    }
}

impl TaskDoc {
    fn shape_layers(&self) -> Vec<LayerData> {
        let mut layers = Vec::new();
        for task_box in self.tasks.iter() {
            match task_box {
                TaskDef::Angle(data) => {
                    layers.append(&mut data.viewport.layers());
                }
                TaskDef::CheckShape(data) => {
                    layers.append(&mut data.viewport.layers());
                }
                TaskDef::Connectivity(data) => {
                    layers.append(&mut data.shape_layers());
                }
                TaskDef::Density(data) => {
                    layers.append(&mut data.viewport.layers());
                }
                TaskDef::Lvs(data) => {
                    layers.append(&mut data.shape_layers());
                }
                TaskDef::PolyEnclosure(data) => {
                    layers.append(&mut data.outer.layers());
                    layers.append(&mut data.inner.layers());
                }
                TaskDef::RectEnclosure(data) => {
                    layers.append(&mut data.outer.layers());
                    layers.append(&mut data.rects.layers());
                }
                TaskDef::ExactSize(data) => {
                    layers.append(&mut data.viewport.layers());
                }
                TaskDef::Extension(data) => {
                    layers.append(&mut data.poly.layers());
                    layers.append(&mut data.diffusion.layers());
                }
                TaskDef::Filter(data) => {
                    layers.append(&mut data.viewport.layers());
                }
                TaskDef::MinArea(data) => {
                    layers.append(&mut data.viewport.layers());
                }
                TaskDef::MinMaxWidth(data) => {
                    layers.append(&mut data.viewport.layers());
                }
                TaskDef::Notch(data) => {
                    layers.append(&mut data.viewport.layers());
                }
                TaskDef::OffGrid(data) => {
                    layers.append(&mut data.viewport.layers());
                }
                TaskDef::SlitLength(data) => {
                    layers.append(&mut data.viewport.layers());
                }
                TaskDef::Spacing(data) => {
                    layers.append(&mut data.viewport_0.layers());
                    if let Some(viewport_1) = &data.viewport_1 {
                        layers.append(&mut viewport_1.layers());
                    }
                }
            }
        }
        layers
    }

    fn mark_layers(&self) -> Vec<LayerData> {
        let mut layers = Vec::new();
        for task_box in self.tasks.iter() {
            match task_box {
                TaskDef::Connectivity(data) => {
                    let binds = data.labels.layers();
                    for bind in binds.iter() {
                        layers.push(bind[0]);
                        layers.push(bind[1]);
                    }
                }
                _ => {}
            }
        }
        layers
    }
}