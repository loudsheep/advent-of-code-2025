use std::fs::{self, File};
use std::io::{self, BufRead, Write};
use std::path::Path;

pub fn read_lines(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    buf.lines().collect()
}