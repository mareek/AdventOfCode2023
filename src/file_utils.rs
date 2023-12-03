use std::fs::File;
use std::io::prelude::*;

pub fn read_input_file(day: i32) -> String {
    return read_day_file(day, "input.txt");
}

pub fn read_day_file(day: i32, file_name: &str) -> String {
    let path = format!("C:\\Source\\AdventOfCode2023\\data\\day{day}\\{file_name}");
    let mut file = File::open(path).unwrap();
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).unwrap();
    return  buffer;
}
