use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

pub fn read_lines(filename: &str) -> Lines<BufReader<File>> {
    let file = File::open(filename).unwrap();
    BufReader::new(file).lines()
}
