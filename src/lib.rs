mod aoc2022;
pub mod aoc2023;
pub mod aoc2024;

use std::fs;

const DAY_1_INPUT: &'static str = "./inputs/day1.txt";

pub fn run_day_1_part_1() {
    let contents = fs::read_to_string(DAY_1_INPUT).unwrap_or_else(|err| {
        eprint!("Problem reading input for day 1 part 1 {}", err);
        std::process::exit(1);
    });

    let calorie_result = aoc2022::day1::calorie_counting_part_one(&contents).unwrap_or_else(|err| {
        eprint!("Problem counting calorie, {}", err);
        std::process::exit(1);
    });
    println!("Max calorie is {}", calorie_result);
}
