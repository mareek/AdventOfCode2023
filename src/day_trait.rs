use crate::file_utils;
use std::time::Instant;

pub trait DaySolver {
    fn day_of_month(&self) -> i32;
    fn solve_first_problem(&self, file_content: &str) -> Option<String>;
    fn solve_second_problem(&self, file_content: &str) -> Option<String>;

    fn solve_problem(&self) {
        let day = self.day_of_month();
        let file_content = file_utils::read_input_file(day);

        self.solve_problems(file_content.as_str(), "");
    }

    fn solve_problem_on_file(&self, input_file: &str) {
        let day = self.day_of_month();
        let file_content = file_utils::read_day_file(day, input_file);

        self.solve_problems(
            file_content.as_str(),
            format!("with {input_file}").as_str(),
        );
    }

    fn solve_problems(&self, file_content: &str, display_context: &str) {
        let day = self.day_of_month();
        let first_start = Instant::now();
        let first_solution = self
            .solve_first_problem(file_content)
            .unwrap_or_else(|| String::from("No Solution yet"));
        let first_duration = first_start.elapsed();

        let second_start = Instant::now();
        let second_solution = self
            .solve_second_problem(file_content)
            .unwrap_or_else(|| String::from("No Solution yet"));
        let second_duration = second_start.elapsed();

        println!("Day {day}\t: {first_solution}\t ({first_duration:.2?}),\t {second_solution}\t ({second_duration:.1?}) {display_context}");
    }
}
