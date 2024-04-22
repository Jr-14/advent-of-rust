// use std::{
//     fs::File,
//     io::{self, BufRead, BufReader},
// };
//
// enum PlayTypes {
//     OpponentRock,
//     OpponentPaper,
//     OpponentScissors,
//     ResponseRock,
//     ResponsePaper,
//     ResponseScissors
// }
//
// struct OpponentRock<'a> {
//     symbol: &'a str,
//     value: i8,
// }
// struct OpponentPaper<'a> {
//     symbol: &'a str,
//     value: i8,
// }
// struct OpponentScissors<'a> {
//     symbol: &'a str,
//     value: i8,
// }
//
// struct ResponseRock<'a> {
//     symbol: &'a str,
//     value: i8,
// }
// struct ResponsePaper<'a> {
//     symbol: &'a str,
//     value: i8,
// }
// struct ResponseScissors<'a> {
//     symbol: &'a str,
//     value: i8,
// }
//
// pub fn parse_play<'a>(play_types: PlayTypes) -> PlayTypes {
//     let play_types = match play_types {
//         PlayTypes::OpponentRock => OpponentRock { symbol: "", value: 3 },
//         PlayTypes::OpponentPaper => OpponentPaper { symbol: "", value: 2 },
//         PlayTypes::OpponentScissors => OpponentScissors { symbol: "", value: 1},
//         PlayTypes::PlayerRock => ResponseRock { symbol: "", value: 3 },
//         PlayTypes::PlayerPaper => ResponsePaper{ symbol: "", value: 2 },
//         PlayTypes::PlayerScissors => ResponseScissors { symbol: "", value: 1},
//     };
//
//     play_types
// }
//
// pub fn rock_paper_scissors(file_path: &str) -> io::Result<()> {
//     let f = File::open(file_path)?;
//     let f = BufReader::new(f);
//
//     for line in f.lines() {
//         println!("{}", line.unwrap());
//     }
//
//     Ok(())
// }
//
// #[cfg(test)]
// mod tests {
//     mod day_1_calorie_counting {
//         use crate::day_1;
//         #[test]
//         fn bubble_borrowed_test() {
//             let mut array: [i32; 3] = [3,2,1];
//             day_1::insert_at_index(&mut array, 0, 5);
//             assert_eq!(array, [5, 3, 2]);
//         }
//
//         #[test]
//         fn test_find_index() {
//             let mut array: [i32; 3] = [7, 5, 3];
//             let index = day_1::find_index(&mut array, 8);
//             match index {
//                 Some(i) => assert_eq!(i, 0),
//                 _ => panic!("Failed to run test."),
//             }
//
//             let mut array: [i32; 3] = [7, 5, 3];
//             let index = day_1::find_index(&mut array, 6);
//             match index {
//                 Some(i) => assert_eq!(i, 1),
//                 _ => panic!("Failed to run test."),
//             }
//
//             let mut array: [i32; 3] = [7, 5, 3];
//             let index = day_1::find_index(&mut array, 4);
//             match index {
//                 Some(i) => assert_eq!(i, 2),
//                 _ => panic!("Failed to run test."),
//             }
//
//             let mut array: [i32; 3] = [7, 5, 3];
//             let index = day_1::find_index(&mut array, 2);
//             match index {
//                 Some(_) => panic!("Failed to run test case."),
//                 None => assert_eq!(index, None),
//             }
//         }
//
//         #[test]
//         fn test_insert_at_index() {
//             let mut array: [i32; 3] = [7, 5, 3];
//             day_1::insert_at_index(&mut array, 0, 8);
//             assert_eq!(array, [8, 7, 5]);
//
//             let mut array: [i32; 3] = [7, 5, 3];
//             day_1::insert_at_index(&mut array, 1, 8);
//             assert_eq!(array, [7, 8, 5]);
//
//             let mut array: [i32; 3] = [7, 5, 3];
//             day_1::insert_at_index(&mut array, 2, 8);
//             assert_eq!(array, [7, 5, 8]);
//         }
//
//         #[test]
//         fn test_calorie_counting_part_two() {
//             let file_path = "./inputs/day_1_test.txt";
//             let sum_calories = day_1::calorie_counting_part_two(&file_path);
//             match sum_calories {
//                 Ok(total_calories) => assert_eq!(total_calories, 690),
//                 Err(e) => panic!("Failed test {}", e),
//             }
//         }
//     }
//
//     
// }
