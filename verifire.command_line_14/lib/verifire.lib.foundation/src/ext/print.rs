use std::fs::File;
use std::io;
use std::io::{BufWriter, Write};

pub(crate) trait Printer {
    fn print_log_error<S: AsRef<str>, F>(&mut self, string: S, error_handler: F)
    where
        F: Fn(io::Error);
}

impl Printer for BufWriter<File> {
    fn print_log_error<S: AsRef<str>, F>(&mut self, string: S, error_handler: F)
    where
        F: Fn(io::Error),
    {
        if let Err(err) = self.write_all(string.as_ref().as_bytes()) {
            error_handler(err);
        }
    }
}