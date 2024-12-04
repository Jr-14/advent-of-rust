use std::{error::Error, fs};
use regex::Regex;

const DAY_3_2024_INPUT: &'static str = "./inputs/2024/day3";

fn parse_token(string_to_parse: &str, token: &str) -> Option<String> {
    let mut parsed_token = String::from("");

    for c in string_to_parse.chars() {
        
    }
}

pub fn day3_part2(contents: &str) -> Result<i32, Box<dyn Error>> {
    let mut total_string = String::from("");

    let re_do = Regex::new(r"do\(\).*(mul\((\d+),(\d+)\))+").unwrap();
    let re = Regex::new(r"mul\((\d+),(\d+)+\)").unwrap();
    let split_content: Vec<&str> = contents.split("don't").collect();
    let mut sum = 0;
    
    for content in re.captures_iter(split_content[0]).map(|c| c.extract()) {
        let (_, [first_digit, second_digit]) = content;
        println!("Content: {:?}", content);
        let first_digit = first_digit.parse::<i32>()?;
        let second_digit = second_digit.parse::<i32>()?;
        sum += first_digit * second_digit;
    }
    Ok(1)
}

pub fn day3_part1(contents: &str) -> Result<i32, Box<dyn Error>> {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut sum = 0;
    for content in re.captures_iter(contents).map(|c| c.extract()) {
        let (_, [first_digit, second_digit]) = content;
        let first_digit = first_digit.parse::<i32>()?;
        let second_digit = second_digit.parse::<i32>()?;
        sum += first_digit * second_digit;
    }
    Ok(sum)
}

pub fn run_day_3() {
   let contents = fs::read_to_string(DAY_3_2024_INPUT).unwrap_or_else(|err| {
        eprint!("Problem reading input for AoC Day 2 Part 1 {}", err);
        std::process::exit(1);
    });
    
    let answer_part_1 = match day3_part1(&contents) {
        Ok(ans) => ans,
        Err(error) => panic!("Problem running AoC 2024 Day 3 Part 1 - {:?}", error)
    };

    let answer_part_2 = match day3_part2(&contents) {
        Ok(ans) => ans,
        Err(error) => panic!("Problem running AoC 2024 Day 3 Part 1 - {:?}", error)
    };

    println!("AoC 2024 Day 3 Part 1 answer is {:?}", answer_part_1);
    println!("AoC 2024 Day 3 Part 2 answer is {:?}", answer_part_2);
}

#[cfg(test)]
mod tests {
    use super::*;
}
