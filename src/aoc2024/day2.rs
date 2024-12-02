use std::{error::Error, fs};

const DAY_2_2024_INPUT: &'static str = "./inputs/2024/day2";

fn is_strictly_decreasing(v: &[i32]) -> bool {
    v.windows(2).all(|w| w[0] > w[1])
}

fn is_strictly_increasing(v: &[i32]) -> bool {
    v.windows(2).all(|w| w[0] < w[1])
}

fn is_safe_level(v: &[i32]) -> bool {
    let correct_order = is_strictly_decreasing(&v) || is_strictly_increasing(&v);
    correct_order && v.windows(2).all(|w| (1..=3).contains(&(w[1] - w[0]).abs()))
}

fn is_safe_level_with_dampener(v: &[i32]) -> bool {
    (0..v.len()).any(|index| {
        let mut report = v.to_vec();
        report.remove(index);
        is_safe_level(&report)
    })
}

pub fn day2_part1(contents: &str) -> Result<i32, Box<dyn Error>> {
    let mut safe_reports_count = 0;
    for line in contents.lines() {
        let report: Vec<i32> = line.split(" ").map(|val| val.parse().unwrap()).collect();
        if is_safe_level(&report) {
            safe_reports_count += 1;
        }
    }

    Ok(safe_reports_count)
}

pub fn day2_part2(contents: &str) -> Result<i32, Box<dyn Error>> {
    let mut safe_reports_count = 0;
    for line in contents.lines() {
        let report: Vec<i32> = line.split(" ").map(|val| val.parse().unwrap()).collect();
        
        if is_safe_level_with_dampener(&report) {
            safe_reports_count += 1;
        }

    }

    Ok(safe_reports_count)
}

pub fn run_day_2_part_1() {
   let contents = fs::read_to_string(DAY_2_2024_INPUT).unwrap_or_else(|err| {
        eprint!("Problem reading input for AoC Day 2 Part 1 {}", err);
        std::process::exit(1);
    });

    
    let answer = match day2_part1(&contents) {
        Ok(ans) => ans,
        Err(error) => panic!("Problem running AoC 2024 Day 2 Part 1 - {:?}", error)
    };

    println!("AoC 2024 Day 2 Part 1 answer is {:?}", answer);
}

pub fn run_day_2_part_2() {
   let contents = fs::read_to_string(DAY_2_2024_INPUT).unwrap_or_else(|err| {
        eprint!("Problem reading input for AoC Day 2 Part 2 {}", err);
        std::process::exit(1);
    });
    
    let answer = match day2_part2(&contents) {
        Ok(score) => score,
        Err(error) => panic!("Problem running AoC 2024 Day 2 part 2 - {:?}", error)
    };

    println!("AoC 2024 Day 2 Part 2 answer is {:?}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_strictly_increasing_test() {
        let arr = [1,2,3,4,5];
        let result = is_strictly_increasing(&arr);
        assert_eq!(result, true);

        let arr = [1,2,2,4,5];
        let result = is_strictly_increasing(&arr);
        assert_eq!(result, false);

        let arr = [1,2,1,4,5];
        let result = is_strictly_increasing(&arr);
        assert_eq!(result, false);

        let arr = [5,4,3,2,1];
        let result = is_strictly_increasing(&arr);
        assert_eq!(result, false);
    }

    #[test]
    fn is_strictly_decreasing_test() {
        let arr = [5,4,3,2,1];
        let result = is_strictly_decreasing(&arr);
        assert_eq!(result, true);

        let arr = [5,4,4,3,2,1];
        let result = is_strictly_decreasing(&arr);
        assert_eq!(result, false);

        let arr = [5,4,3,5,2,1];
        let result = is_strictly_decreasing(&arr);
        assert_eq!(result, false);

        let arr = [1,2,3,4,5];
        let result = is_strictly_decreasing(&arr);
        assert_eq!(result, false);

    }
}
