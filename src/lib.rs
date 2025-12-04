use std::fs::{File};
use std::io::{self, BufRead};
use std::path::Path;

pub fn read_lines(filename: impl AsRef<Path>) -> io::Result<Vec<String>> {
    let file = File::open(filename)?;
    let buf = io::BufReader::new(file);
    buf.lines().collect()
}

pub fn read_line(filename: impl AsRef<Path>) -> io::Result<String> {
    let file = File::open(filename)?;
    let mut buf = io::BufReader::new(file);
    let mut line = String::new();
    buf.read_line(&mut line)?;
    Ok(line)
}

pub fn parse_csv_line(line: &str) -> Vec<String> {
    line.split(',').map(|s| s.trim().to_string()).collect()
}