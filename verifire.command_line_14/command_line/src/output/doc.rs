use serde::Serialize;
use foundation::export::err_file::Violation;

#[derive(Serialize, Debug)]
pub(crate) struct ErrorDoc {
    pub(crate) dimension: f64,
    pub(crate) cells: Vec<ResultCell>,
}

#[derive(Serialize, Debug, Clone)]
pub(crate) struct ResultCell {
    pub(crate) cell_name: String,
    pub(crate) rules: Vec<ResultRule>,
}

#[derive(Serialize, Debug, Clone)]
pub(crate) struct ResultRule {
    pub(crate) rule_id: String,
    pub(crate) violations: Vec<Violation>,
}