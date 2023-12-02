use std::path::Path;

mod day1;

fn main() {
    let day1_result =
        day1::compute_first_problem(Path::new("C:\\Source\\AdventOfCode2023\\data\\day1\\input.txt"));
    println!("Day 1 : {}", day1_result);
}
