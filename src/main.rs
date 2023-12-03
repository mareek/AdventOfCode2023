mod file_utils;
mod day_trait;
mod day1;

fn main() {
    solve_day_problem(day1::Day1);
}

fn solve_day_problem(day_solver: impl day_trait::DaySolver) {
    let day = day_solver.day_of_month();
    let read_input_file = &file_utils::read_input_file(day);
    let day_file_content = read_input_file.as_str();
    let day_first_problem = day_solver.solve_first_problem(day_file_content);
    let day_second_problem = day_solver.solve_second_problem(day_file_content);
    
    println!("Day {day} : {day_first_problem}, {day_second_problem}");

}
