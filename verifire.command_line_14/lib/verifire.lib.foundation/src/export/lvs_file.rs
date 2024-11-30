use std::fs::File;
use std::io;
use std::io::{Write, BufWriter};
use std::path::Path;
use log::error;
use crate::ext::print::Printer;

pub struct LvsDevice {
    pub id: String,
    pub left: String,
    pub right: String,
    pub gate: String,
    pub body: String,
    pub transistor: String,
    pub width: f64,
    pub length: f64,
}

pub struct LvsFile {
    pub title: String,
    pub devices: Vec<LvsDevice>,
}

impl LvsFile {
    pub fn save(&self, path: &Path) {
        let file_path = path.to_string_lossy().to_string();
        let file = match File::create(path) {
            Ok(file) => {
                file
            }
            Err(err) => {
                error!("Couldn't create {}: {}", file_path, err);
                return;
            }
        };
        let mut writer = BufWriter::new(file);

        let log_error = |err: io::Error| {
            error!("Couldn't write to {}: {}", file_path, err);
        };

        writer.print_log_error(format!(".SUBCKT {} \n", self.title), log_error);

        for device in self.devices.iter() {
            let s = device.size();
            writer.print_log_error(format!("{} {} {} {} {} {} {} \n", device.id, device.left, device.gate, device.right, device.body, device.transistor, s), log_error);
        }

        writer.print_log_error(".ENDS", log_error);

        if let Err(err) = writer.flush() {
            error!("Couldn't save {}: {}", file_path, err);
        }
    }
}

impl LvsDevice {
    const MU: f64 = 1000_000.0;

    fn size(&self) -> String {
        let w = Self::pretty_look(self.width);
        let l = Self::pretty_look(self.length);
        format!("w={} l={}", w, l)
    }

    fn pretty_look(value: f64) -> String {
        let mw = Self::MU * value;
        if mw < 1.0 {
            format!("{:.2}µ", mw)
        } else if mw < 1000.0 {
            format!("{:.1}µ", mw)
        } else {
            format!("{:.2}", value)
        }
    }
}