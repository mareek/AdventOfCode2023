mod day1;
mod file_utils;

fn main() {
    let read_input_file = &file_utils::read_input_file(1);
    let day1_file_content = read_input_file.as_str();
    let day1_first_problem = day1::compute_first_problem(day1_file_content);
    let day1_second_problem = day1::compute_second_problem(day1_file_content);
    println!("Day 1 : {day1_first_problem}, {day1_second_problem}");
}
