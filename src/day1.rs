#[path = "file_utils.rs"]
mod file_utils;

use std::path::Path;

pub fn compute_first_problem(path: &Path) -> i32 {
    let lines = file_utils::read_file_lines(path);

    let mut sum = 0;
    for line in lines {
        let mut has_digit = false;
        let mut first_digit = '0';
        let mut last_digit = '0';
        for char in line.unwrap().chars().filter(|c| c.is_digit(10)) {
            if !has_digit {
                has_digit = true;
                first_digit = char;
            }
            last_digit = char;
        }

        if has_digit {
            let str_value: String = [first_digit, last_digit].iter().collect();
            let value: i32 = str_value.parse::<i32>().unwrap();
            sum += value;
        }
    }
    return sum;
}
