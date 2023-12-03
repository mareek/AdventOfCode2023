pub trait DaySolver {
    fn day_of_month(&self) -> i32;
    fn solve_first_problem(&self, file_content: &str) -> String;
    fn solve_second_problem(&self, file_content: &str) -> String;
}
