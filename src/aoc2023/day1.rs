use std::fs;

const PART_ONE_FILE_NAME: &'static str = "./inputs/2023-day1.txt";
const PART_TWO_FILE_NAME: &'static str = "./inputs/2023-day1.txt";

pub fn run_part_one() {
    let contents = fs::read_to_string(PART_ONE_FILE_NAME).expect("Cannot readfile");
    let sum = sum_calibration_value(&contents);
    println!("The sum of calibration values are {sum}");
}

fn sum_calibration_value(values: &str) -> u32 {
    let mut sum = 0;
    for line in values.lines() {
        sum += get_calibration_value(line);
    }
    sum
}

fn get_calibration_value(calibration_value: &str) -> u32 {
    let radix = 10;
    let mut first_digit = 0;
    let mut last_digit = 0;
    for c in calibration_value.chars() {
        if first_digit == 0 && c.is_digit(radix) {
            first_digit = c.to_digit(radix).unwrap();
        } 
        if c.is_digit(radix) {
            last_digit = c.to_digit(radix).unwrap();
        }
    }
    first_digit * 10 + last_digit
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn value_with_3_digits() {
        let s = "1te2st3";
        let num = get_calibration_value(&s);
        assert_eq!(num, 13);
    }

    #[test]
    fn value_with_2_digits() {
        let s = "test50";
        let num = get_calibration_value(&s);
        assert_eq!(num, 50);
    }

    #[test]
    fn value_with_1_digit() {
        let s = "test1";
        let num = get_calibration_value(&s);
        assert_eq!(num, 11);
    }

    #[test]
    fn value_with_no_digits() {
        let s = "test";
        let num = get_calibration_value(&s);
        assert_eq!(num, 0);
    }

    #[test]
    fn large_amount_of_digits() {
        let s = "d989asdfakjah20fa015";
        let num = get_calibration_value(&s);
        assert_eq!(num, 95);
    }

    #[test]
    fn sum_calibration() {
        let s = "ninefourone1\n
53sevenvvqm\n
kscpjfdxp895foureightckjjl1\n
72fivebt9ndgq\n
28gtbkszmrtmnineoneightmx";
        let sum = sum_calibration_value(&s);
        assert_eq!(sum, 252);
    }
}
