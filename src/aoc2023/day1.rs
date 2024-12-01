// use std::{fs, usize};
// use std::str::FromStr;
//
// const PART_ONE_FILE_NAME: &'static str = "./inputs/2023-day1.txt";
// const PART_TWO_FILE_NAME: &'static str = "./inputs/2023-day1.txt";
//
// #[derive(Debug, PartialEq)]
// enum LetteredNumbers {
//     One,
//     Two,
//     Three,
//     Four,
//     Five,
//     Six,
//     Seven,
//     Eight,
//     Nine,
// }
//
// impl LetteredNumbers {
//     fn to_str(&self) -> String {
//         match self {
//             self::LetteredNumbers::One => String::from("one"),
//             self::LetteredNumbers::Two => String::from("two"),
//             self::LetteredNumbers::Three => String::from("three"),
//             self::LetteredNumbers::Four => String::from("four"),
//             self::LetteredNumbers::Five => String::from("five"),
//             self::LetteredNumbers::Six => String::from("six"),
//             self::LetteredNumbers::Seven => String::from("seven"),
//             self::LetteredNumbers::Eight => String::from("eight"),
//             self::LetteredNumbers::Nine => String::from("nine"),
//         }
//     }
// }
//
// impl FromStr for LetteredNumbers {
//     type Err = ();
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "one" => Ok(Self::One),
//             "two" => Ok(Self::Two),
//             "three" => Ok(Self::Three),
//             "four" => Ok(Self::Four),
//             "five" => Ok(Self::Five),
//             "six" => Ok(Self::Six),
//             "seven" => Ok(Self::Seven),
//             "eight" => Ok(Self::Eight),
//             "nine" => Ok(Self::Nine),
//             _ => Err(())
//         }
//     }
// }
//
// pub fn run_part_one() {
//     let contents = fs::read_to_string(PART_ONE_FILE_NAME).expect("Cannot readfile");
//     let sum = sum_calibration_value(&contents);
//     println!("The sum of calibration values are {sum}");
// }
//
// fn find_right_string_number(calibration_value: &str, letter_number: LetteredNumbers) -> Option<usize> {
//     let s = calibration_value.rfind(&letter_number.to_str());
//     match s {
//         Some(index) => Some(index),
//         None => None,
//     }
// }
//
// fn get_calibration_value_with_string(calibration_value: &str) -> u32 {
//     let lettered_numbers: Vec<String> = vec![
//         "one".to_string(), 
//         "two".to_string(),
//         "three".to_string(),
//         "four".to_string(),
//         "five".to_string(),
//         "six".to_string(),
//         "seven".to_string(),
//         "eight".to_string(),
//         "nine".to_string()
//     ];
//     let s = String::from(calibration_value);
//     let left_string_num = lettered_numbers.iter().map(|x| {
//         match calibration_value.find(x) {
//             Some(index) => Some(index),
//             None => None,
//         }
//     }).collect::<Option<usize>>();
//     0
// }
//
// fn sum_calibration_value(values: &str) -> u32 {
//     let mut sum = 0;
//     for line in values.lines() {
//         sum += get_calibration_value(line);
//     }
//     sum
// }
//
// fn get_calibration_value(calibration_value: &str) -> u32 {
//     let radix = 10;
//     let mut first_digit = 0;
//     let mut last_digit = 0;
//     for c in calibration_value.chars() {
//         if first_digit == 0 && c.is_digit(radix) {
//             first_digit = c.to_digit(radix).unwrap();
//         } 
//         if c.is_digit(radix) {
//             last_digit = c.to_digit(radix).unwrap();
//         }
//     }
//     first_digit * 10 + last_digit
// }
//
//
// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn value_with_3_digits() {
//         let s = "1te2st3";
//         let num = get_calibration_value(&s);
//         assert_eq!(num, 13);
//     }
//
//     #[test]
//     fn value_with_2_digits() {
//         let s = "test50";
//         let num = get_calibration_value(&s);
//         assert_eq!(num, 50);
//     }
//
//     #[test]
//     fn value_with_1_digit() {
//         let s = "test1";
//         let num = get_calibration_value(&s);
//         assert_eq!(num, 11);
//     }
//
//     #[test]
//     fn value_with_no_digits() {
//         let s = "test";
//         let num = get_calibration_value(&s);
//         assert_eq!(num, 0);
//     }
//
//     #[test]
//     fn large_amount_of_digits() {
//         let s = "d989asdfakjah20fa015";
//         let num = get_calibration_value(&s);
//         assert_eq!(num, 95);
//     }
//
//     #[test]
//     fn sum_calibration() {
//         let s = "ninefourone1\n
// 53sevenvvqm\n
// kscpjfdxp895foureightckjjl1\n
// 72fivebt9ndgq\n
// 28gtbkszmrtmnineoneightmx";
//         let sum = sum_calibration_value(&s);
//         assert_eq!(sum, 252);
//     }
//
//     #[test]
//     fn letter_as_characters() {
//         let s = "eigh2two3three";
//         let num = get_calibration_value_with_string(s);
//         assert_eq!(num, 83);
//     }
// }
