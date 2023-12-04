use crate::file_utils;

pub trait DaySolver {
    fn day_of_month(&self) -> i32;
    fn solve_first_problem(&self, file_content: &str) -> String;
    fn solve_second_problem(&self, file_content: &str) -> String;

    fn solve_problem(&self) {
        let day = self.day_of_month();
        let read_input_file = file_utils::read_input_file(day);
        let day_first_solution = self.solve_first_problem(read_input_file.as_str());
        let day_second_solution = self.solve_second_problem(read_input_file.as_str());

        println!("Day {day} : {day_first_solution}, {day_second_solution}");
    }

    fn test_problem(&self, test_file: &str) {
        let day = self.day_of_month();
        let read_input_file = file_utils::read_day_file(day, test_file);
        let day_first_solution = self.solve_first_problem(read_input_file.as_str());
        let day_second_solution = self.solve_second_problem(read_input_file.as_str());

        println!("Test Day {day} with {test_file} : {day_first_solution}, {day_second_solution}");
    }
}
