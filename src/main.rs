mod day1;
mod day2;
mod day3;
mod day_trait;
mod file_utils;

use crate::day_trait::DaySolver;

fn main() {
    day1::Day1.solve_problem();
    day2::Day2.solve_problem();
    day3::Day3.solve_problem();

    day3::Day3.test_problem("testInput.txt");
}
