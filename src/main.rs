mod file_utils;
mod day_trait;
mod day1;
mod day2;

fn main() {
    solve_problem(day1::Day1);
    solve_problem(day2::Day2);

    test_problem(day2::Day2, "testInput.txt");
}

fn solve_problem(day_solver: impl day_trait::DaySolver) {
    let day = day_solver.day_of_month();
    let read_input_file = file_utils::read_input_file(day);
    let day_first_solution = day_solver.solve_first_problem(read_input_file.as_str());
    let day_second_solution = day_solver.solve_second_problem(read_input_file.as_str());

    println!("Day {day} : {day_first_solution}, {day_second_solution}");
}

fn test_problem(day_solver: impl day_trait::DaySolver, test_file: &str) {
    let day = day_solver.day_of_month();
    let read_input_file = file_utils::read_day_file(day, test_file);
    let day_first_solution = day_solver.solve_first_problem(read_input_file.as_str());
    let day_second_solution = day_solver.solve_second_problem(read_input_file.as_str());

    println!("Test Day {day} with {test_file} : {day_first_solution}, {day_second_solution}");
}
