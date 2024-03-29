mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day18;
mod day_trait;
mod file_utils;

use crate::day_trait::DaySolver;

fn main() {
    day1::Day1.solve_problem();
    day2::Day2.solve_problem();
    day3::Day3.solve_problem();
    day4::Day4.solve_problem();
    day5::Day5.solve_problem_on_file("testInput.txt");
    day6::Day6.solve_problem();
    day7::Day7.solve_problem();
    day8::Day8.solve_problem();
    day9::Day9.solve_problem();
    day10::Day10.solve_problem();
    day11::Day11.solve_problem_on_file("testInput.txt");
    day12::Day12.solve_problem_on_file("testInput.txt");
    day13::Day13.solve_problem();
    day14::Day14.solve_problem_on_file("testInput.txt");
    day15::Day15.solve_problem();
    day16::Day16.solve_problem();
    
    day18::Day18.solve_problem();

    day18::Day18.solve_problem_on_file("testInput.txt");
}
