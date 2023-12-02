use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::Error;
use std::path::Path;

pub fn read_file_lines(path: &Path) -> Vec<Result<String, Error>> {
    return BufReader::new(File::open(path).unwrap())
        .lines()
        .collect::<Vec<_>>();
}
