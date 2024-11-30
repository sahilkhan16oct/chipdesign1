use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct UnitSize {
    pub user_size: f64,
    pub db_size: f64,
}
