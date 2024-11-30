use std::path::{Path, PathBuf};
use serde::Deserialize;
use serde_json::Value;
use crate::task::task_connectivity::ConnectivityTaskDef;
use crate::task::task_angle::AngleTaskDef;
use crate::task::task_check_shape::CheckShapeTaskDef;
use crate::task::task_density::DensityTaskDef;
use crate::task::task_enc_poly::PolyEnclosureTaskDef;
use crate::task::task_enc_rect::RectEnclosureTaskDef;
use crate::task::task_exact_size::ExactSizeTaskDef;
use crate::task::task_extension::ExtensionTaskDef;
use crate::task::task_filter::FilterTaskDef;
use crate::task::task_lvs::LvsTaskDef;
use crate::task::task_min_area::MinAreaTaskDef;
use crate::task::task_min_max_width::MinMaxWidthTaskDef;
use crate::task::task_notch_error::NotchTaskDef;
use crate::task::task_off_grid::OffGridTaskDef;
use crate::task::task_slit_length::SlitLengthTaskDef;
use crate::task::task_spacing::SpacingTaskDef;

#[derive(Deserialize, Debug)]
#[serde(tag = "rule_type")]
#[serde(rename_all = "snake_case")]
pub enum TaskDef {
    Angle(AngleTaskDef),
    CheckShape(CheckShapeTaskDef),
    Connectivity(ConnectivityTaskDef),
    Density(DensityTaskDef),
    Lvs(LvsTaskDef),
    PolyEnclosure(PolyEnclosureTaskDef),
    RectEnclosure(RectEnclosureTaskDef),
    ExactSize(ExactSizeTaskDef),
    Extension(ExtensionTaskDef),
    Filter(FilterTaskDef),
    MinArea(MinAreaTaskDef),
    MinMaxWidth(MinMaxWidthTaskDef),
    Notch(NotchTaskDef),
    OffGrid(OffGridTaskDef),
    SlitLength(SlitLengthTaskDef),
    Spacing(SpacingTaskDef),
}

impl TaskDef {
    pub fn title(&self) -> String {
        match self {
            TaskDef::Angle(def) => def.title(),
            TaskDef::CheckShape(def) => def.title(),
            TaskDef::Connectivity(def) => def.title(),
            TaskDef::Density(def) => def.title(),
            TaskDef::Lvs(def) => def.title(),
            TaskDef::PolyEnclosure(def) => def.title(),
            TaskDef::RectEnclosure(def) => def.title(),
            TaskDef::ExactSize(def) => def.title(),
            TaskDef::Extension(def) => def.title(),
            TaskDef::Filter(def) => def.title(),
            TaskDef::MinArea(def) => def.title(),
            TaskDef::MinMaxWidth(def) => def.title(),
            TaskDef::Notch(def) => def.title(),
            TaskDef::OffGrid(def) => def.title(),
            TaskDef::SlitLength(def) => def.title(),
            TaskDef::Spacing(def) => def.title(),
        }
    }
}

#[derive(Deserialize, Debug)]
pub enum Format {
    #[serde(rename = "rve")]
    RVE,
    #[serde(rename = "json")]
    JSON,
}

#[derive(Deserialize, Debug)]
pub struct TaskDoc {
    pub input: String,
    pub output: String,
    pub lvs_output: Option<String>,
    pub format: Format,
    pub cell: String,
    pub tasks: Vec<TaskDef>,
}

impl TaskDoc {
    pub fn load(file_path: &Path) -> Self {
        let path_buf = PathBuf::from(file_path);
        let data = match std::fs::read_to_string(path_buf.as_path()) {
            Ok(data) => {
                data
            }
            Err(e) => {
                log::error!("Failed to read file: {}", e);
                panic!("{:?}", e);
            }
        };

        let result: Result<TaskDoc, _> = serde_json::from_str(&data);
        match result {
            Ok(test) => test,
            Err(e) => {
                log::error!("Failed to parse JSON: {}", e);
                panic!("can not parse file");
            }
        }
    }

    pub fn is_valid(file_path: &Path) -> bool {
        let path_buf = PathBuf::from(file_path);
        let data = match std::fs::read_to_string(path_buf.as_path()) {
            Ok(data) => {
                data
            }
            Err(e) => {
                log::error!("Failed to read file: {}", e);
                panic!("{:?}", e);
            }
        };

        let json_value: Value = serde_json::from_str(data.as_str()).unwrap();

        if let Some(_) = json_value.get("tasks") {
            true
        } else {
            false
        }
    }
}