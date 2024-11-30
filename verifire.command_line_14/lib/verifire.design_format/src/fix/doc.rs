use std::collections::HashMap;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use gds21::{GdsElement, GdsLibrary, GdsStruct, GdsUnits};
use i_overlay::i_shape::int::shape::IntShapes;
use crate::fix::array::ArrayRef;
use crate::fix::cell::Cell;
use crate::fix::layer::LayerData;
use crate::fix::mark::Mark;
use crate::fix::path_data::PathData;
use crate::fix::polygon_res::PolygonRes;
use crate::fix::reference::Reference;
use crate::fix::resource::Resource;
use crate::fix::resource::Resource::Polygons;
use crate::fix::scaler::UnitScaler;
use crate::raw::cell::RawCell;
use crate::raw::document::RawDocument;
use crate::raw::property::Property;
use crate::raw::unit_size::UnitSize;

pub struct FixDoc {
    pub name: String,
    pub scaler: UnitScaler,
    pub cells: HashMap<String, Cell>,
    pub(super) shape_cache: HashMap<String, HashMap<LayerData, IntShapes>>,
    pub(super) mark_cache: HashMap<String, HashMap<LayerData, Vec<Mark>>>,
}

pub enum FileType {
    GDS,
    RAW,
}

impl FixDoc {
    pub fn resource_type(path: &Path) -> Option<FileType> {
        match path.extension().and_then(std::ffi::OsStr::to_str) {
            Some("gds") => Some(FileType::GDS),
            Some("json") => Some(FileType::RAW),
            Some(_) => None,
            None => None,
        }
    }

    pub fn load_from_file(path: &Path) -> Result<Self, String> {
        match path.extension().and_then(std::ffi::OsStr::to_str) {
            Some("gds") => Self::with_gds_file(path),
            Some("json") => Self::with_raw_file(path),
            Some(ext) => Err(format!("Unsupported file extension: .{}", ext)),
            None => Err("File has no extension".to_string()),
        }
    }

    pub fn load_from_gds_bytes(vec: Vec<u8>) -> Result<Self, String> {
        let data = match GdsLibrary::from_bytes(vec) {
            Ok(data) => {
                data
            }
            Err(error) => {
                return Err(error.to_string());
            }
        };
        Self::read_gds_data(data)
    }

    pub fn with_gds_file(path: &Path) -> Result<Self, String> {
        let data = match GdsLibrary::load(path) {
            Ok(data) => {
                data
            }
            Err(error) => {
                return Err(error.to_string());
            }
        };
        Self::read_gds_data(data)
    }

    fn read_gds_data(data: GdsLibrary) -> Result<Self, String> {
        let scaler = UnitScaler::new(data.units.user_unit() * data.units.db_unit(), data.units.db_unit());

        let mut cells = HashMap::with_capacity(data.structs.len());
        for st in data.structs.iter() {
            let mut resources = Vec::new();
            let mut poly_map = HashMap::new();
            for e in st.elems.iter() {
                match e {
                    GdsElement::GdsBoundary(raw) => {
                        let layer = LayerData { number: raw.layer as i32, datatype: raw.datatype as i32 };
                        let poly = poly_map.entry(layer).or_insert(PolygonRes { layer, paths: Vec::new() });
                        poly.add_boundary(raw, scaler);
                    }
                    GdsElement::GdsPath(raw) => {
                        resources.push(Resource::PathData(PathData::with_gds(raw, scaler)));
                    }
                    GdsElement::GdsStructRef(raw) => {
                        resources.push(Resource::Reference(Reference::with_gds(raw, scaler)));
                    }
                    GdsElement::GdsArrayRef(raw) => {
                        resources.push(Resource::Array(ArrayRef::with_gds(raw, scaler)));
                    }
                    GdsElement::GdsTextElem(raw) => {
                        resources.push(Resource::Mark(Mark::with_gds(st.name.clone(), raw, &scaler)));
                    }
                    GdsElement::GdsNode(_) => {
                        log::error!("GdsNode is not implemented");
                    }
                    GdsElement::GdsBox(_) => {
                        log::error!("GdsBox is not implemented");
                    }
                }
            }

            for item in poly_map.into_iter() {
                resources.push(Polygons(item.1))
            }

            let cell = Cell { id: "".to_owned(), name: st.name.clone(), resources };

            cells.insert(cell.name.clone(), cell);
        }

        let doc = Self {
            shape_cache: Default::default(),
            name: data.name,
            cells,
            scaler,
            mark_cache: Default::default(),
        };

        Ok(doc)
    }

    pub fn with_raw_file(path: &Path) -> Result<Self, String> {
        let json = match std::fs::read_to_string(path) {
            Ok(json) => {
                json
            }
            Err(error) => {
                return Err(error.to_string());
            }
        };

        let json = Self::convert_json_to_raw_content(&json)?;

        Self::with_raw_content(&json)
    }

    pub fn convert_json_to_raw_content(json: &str) -> Result<RawDocument, String> {
        match serde_json::from_str(json) {
            Ok(data) => Ok(data),
            Err(error) => Err(error.to_string()),
        }
    }

    pub fn with_raw_content(data: &RawDocument) -> Result<Self, String> {
        let scaler = UnitScaler::new(data.units.user_size, data.units.db_size);

        let mut cells = HashMap::with_capacity(data.cells.len());
        for cell in data.cells.iter() {
            let mut resources = Vec::new();
            let mut poly_map = HashMap::new();
            for prop in cell.properties.iter() {
                match prop {
                    Property::Instance(raw) => {
                        resources.push(Resource::Reference(Reference::with_raw(raw, scaler)));
                    }
                    Property::Polygon(raw) => {
                        let layer = LayerData {
                            number: raw.layer_number,
                            datatype: raw.datatype_number,
                        };
                        let poly = poly_map
                            .entry(layer)
                            .or_insert(PolygonRes { layer, paths: Vec::new() });
                        poly.add_polygon(raw, scaler);
                    }
                    Property::Rectangle(raw) => {
                        let layer = LayerData {
                            number: raw.layer_number,
                            datatype: raw.datatype_number,
                        };
                        let poly = poly_map
                            .entry(layer)
                            .or_insert(PolygonRes { layer, paths: Vec::new() });
                        poly.add_rectangle(raw, scaler);
                    }
                    Property::Label(_text) => {}
                }
            }

            for item in poly_map.into_iter() {
                resources.push(Polygons(item.1))
            }

            let cell = Cell { id: cell.cell_id.clone(), name: cell.cell_name.clone(), resources };

            cells.insert(cell.name.clone(), cell);
        }

        let doc = Self {
            shape_cache: Default::default(),
            name: data.name.to_owned(),
            cells,
            scaler,
            mark_cache: Default::default(),
        };

        Ok(doc)
    }

    pub fn save(&self, path: &Path, pretty_print: bool) -> Result<(), String> {
        match path.extension().and_then(std::ffi::OsStr::to_str) {
            Some("gds") => self.save_gds(path),
            Some("json") => self.save_raw(path, pretty_print),
            Some(ext) => Err(format!("Unsupported file extension: .{}", ext)),
            None => Err("File has no extension".to_string()),
        }
    }

    pub fn save_gds(&self, path: &Path) -> Result<(), String> {
        let mut gds = GdsLibrary::new(&self.name);
        gds.units = GdsUnits::new(self.scaler.unit_size, self.scaler.db_size);

        for cell in self.cells.values() {
            let mut st = GdsStruct::new(cell.name.clone());
            for rs in cell.resources.iter() {
                match rs {
                    Resource::Array(array) => {
                        st.elems.push(GdsElement::GdsArrayRef(array.as_gds(self.scaler)));
                    }
                    Resource::Reference(reference) => {
                        st.elems.push(GdsElement::GdsStructRef(reference.as_gds(self.scaler)));
                    }
                    Polygons(polygon) => {
                        for bnd in polygon.as_gds(self.scaler).into_iter() {
                            st.elems.push(GdsElement::GdsBoundary(bnd));
                        }
                    }
                    Resource::Mark(mark_data) => {
                        st.elems.push(GdsElement::GdsTextElem(mark_data.as_gds(self.scaler)));
                    }
                    Resource::PathData(path_data) => {
                        st.elems.push(GdsElement::GdsPath(path_data.as_gds(self.scaler)));
                    }
                }
            }

            gds.structs.push(st);
        }

        match gds.save(path) {
            Ok(_) => {
                Ok(())
            }
            Err(error) => {
                Err(error.to_string())
            }
        }
    }

    pub fn save_raw(&self, path: &Path, pretty_print: bool) -> Result<(), String> {
        let mut cells = Vec::with_capacity(self.cells.len());

        for item in self.cells.values() {
            let mut properties = Vec::new();

            for res in item.resources.iter() {
                match res {
                    Resource::Array(_) => {
                        log::error!("ArrayRef is not implemented for save as raw(json)");
                    }
                    Resource::Reference(reference) => {
                        properties.push(Property::Instance(reference.as_raw(self.scaler)));
                    }
                    Polygons(res) => {
                        let props = res.as_raw(self.scaler);
                        properties.extend(props);
                    }
                    Resource::Mark(_) => {
                        log::error!("Mark is not implemented for save as raw(json)");
                    }
                    Resource::PathData(_) => {
                        log::error!("PathData is not implemented for save as raw(json)");
                    }
                }
            }

            cells.push(RawCell { cell_id: item.id.clone(), cell_name: item.name.clone(), properties });
        }

        let doc = RawDocument {
            name: self.name.clone(),
            units: UnitSize { user_size: self.scaler.unit_size, db_size: self.scaler.db_size },
            cells,
        };

        let data = if pretty_print {
            match serde_json::to_string_pretty(&doc) {
                Ok(json) => json,
                Err(error) => {
                    return Err(error.to_string());
                }
            }
        } else {
            match serde_json::to_string(&doc) {
                Ok(json) => json,
                Err(error) => {
                    return Err(error.to_string());
                }
            }
        };

        let mut file = match File::create(path) {
            Ok(file) => file,
            Err(error) => {
                return Err(error.to_string());
            }
        };

        match file.write_all(data.as_bytes()) {
            Ok(_) => Ok(()),
            Err(error) => Err(error.to_string()),
        }
    }
}
