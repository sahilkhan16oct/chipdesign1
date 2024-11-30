use std::fs::File;
use std::io;
use std::io::{Write, BufWriter};
use std::path::Path;
use log::error;
use crate::export::shape::ExpShape;
use crate::ext::print::Printer;

// https://github.com/KLayout/klayout/blob/01c19048aa2c9392abd696497811635742ec2c4e/src/rdb/rdb/rdbRVEReader.cc#L41
pub struct RveFile {
    writer: BufWriter<File>,
    file_path: String,
}

impl RveFile {
    pub fn new(path: &Path) -> Self {
        let file_path = path.to_string_lossy().to_string();
        let file = File::create(path).unwrap_or_else(|why| panic!("Couldn't create {}: {}", file_path, why));
        let writer = BufWriter::new(file);
        Self { writer, file_path }
    }

    pub fn start(&mut self, header: &str, scale: usize) {
        self.write_log_error(format!("{} {}\n", header, scale));
    }

    pub fn finish(&mut self) -> std::io::Result<()> {
        self.writer.flush()
    }

    pub fn add_section(&mut self, title: &String, desc: Option<Vec<String>>, shapes: &Vec<ExpShape>) {
        self.write_log_error(format!("{}\n", title));

        // Description handling
        if let Some(desc) = desc {
            let mut desc_text = format!("{} 0 {}\n", shapes.len(), desc.len());
            for line in desc.iter() {
                desc_text.push_str(line);
                desc_text.push('\n');
            }
            self.write_log_error(desc_text);
        } else {
            self.write_log_error(format!("{} 0 0\n", shapes.len()));
        }
        let mut id = 1;
        for shape in shapes.iter() {
            self.write_shape(id, shape);
            id += 1;
        }
    }

    fn write_log_error(&mut self, string: String) {
        let log_error = |err: io::Error| {
            error!("Couldn't write to {}: {}", self.file_path, err);
        };
        self.writer.print_log_error(string, log_error);
    }

    fn write_shape(&mut self, id: usize, shape: &ExpShape) {
        match shape {
            ExpShape::Dot(dot) => {
                self.write_log_error(format!("e {} {}\n", id, 1));
                self.write_log_error(format!("{} {} {} {}\n", dot.x, dot.y, dot.x, dot.y));
            }
            ExpShape::Path(path) => {
                self.write_log_error(format!("p {} {}\n", id, path.len()));
                for p in path.iter() {
                    self.write_log_error(format!("{} {}\n", p.x, p.y));
                }
            }
            ExpShape::Edge(e) => {
                self.write_log_error(format!("e {} {}\n", id, 1));
                let a = e[0];
                let b = e[1];
                self.write_log_error(format!("{} {} {} {}\n", a.x, a.y, b.x, b.y));
            }
        }
    }
}