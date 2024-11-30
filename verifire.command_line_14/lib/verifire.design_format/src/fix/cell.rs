use crate::fix::resource::Resource;

pub struct Cell {
    pub id: String,
    pub name: String,
    pub resources: Vec<Resource>,
}

