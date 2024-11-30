use std::path::{Path, PathBuf};
use serde::Deserialize;
use serde_json::Value;
use crate::action::action_slice::SliceActionDef;


#[derive(Deserialize, Debug)]
#[serde(tag = "action_type")]
#[serde(rename_all = "snake_case")]
pub enum ActionDef {
    Slice(SliceActionDef),
}



#[derive(Deserialize, Debug)]
pub struct ActionDoc {
    pub file: String,
    pub actions: Vec<ActionDef>,
}

impl ActionDoc {
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

        let result: Result<ActionDoc, _> = serde_json::from_str(&data);
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

        if let Some(_) = json_value.get("actions") {
            true
        } else {
            false
        }
    }
}