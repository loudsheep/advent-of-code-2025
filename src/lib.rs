use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    buf.lines().collect()
}