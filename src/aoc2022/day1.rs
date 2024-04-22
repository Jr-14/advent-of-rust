use std::error::Error;

pub fn calorie_counting_part_one(contents: &str) -> Result<i32, Box<dyn Error>> {
    let mut max_calories = 0;
    let mut current_calorie = 0;
    for line in contents.lines() {
        if line == "" {
            if current_calorie > max_calories {
                max_calories = current_calorie;
            }
            current_calorie = 0;
        } else {
            let calorie: i32 = line.parse()?;
            current_calorie += calorie;
        }
    }

    Ok(max_calories)
}

// pub fn calorie_counting_part_two(file: &str) -> Result<i32, Box<dyn Error>> {
//     let contents = fs::read_to_string(file)?;
//     let mut max_calories = [0, 0, 0];
//     let mut current_calory_tally = 0;
//     for line in contents.lines() {
//         if line == "" {
//             if let Some(index) = find_index(&mut max_calories, current_calory_tally) {
//                 insert_at_index(&mut max_calories, index, current_calory_tally);
//             }
//             current_calory_tally = 0;
//         } else {
//             let calorie: i32 = line.parse()?;
//             current_calory_tally += calorie;
//         }
//     }
//     Ok(max_calories.iter().sum())
// }
//
// pub fn update_calorie_array(calorie_array: [i32; 3], calorie: i32) -> [i32; 3] {
//     let mut new_calorie_array = calorie_array.clone();
//     for (index, existing_calorie) in calorie_array.iter().enumerate() {
//         if calorie > *existing_calorie {
//             let mut next_calorie = calorie;
//             let mut array_index = index;
//             while array_index < 3 {
//                 let calorie_to_move = calorie_array[array_index];
//                 new_calorie_array[array_index] = next_calorie;
//                 array_index += 1;
//                 if array_index >= 3 {
//                     break;
//                 }
//                 new_calorie_array[array_index] = calorie_to_move;
//                 next_calorie = new_calorie_array[array_index];
//             }
//             break;
//         }
//     }
//     new_calorie_array
// }
//
// /**
//  * Insert item at index and bubble down the values
//  */
// pub fn insert_at_index(array: &mut [i32; 3], index: usize, item: i32) {
//     let mut i: usize = 2;
//     while i >= index + 1 {
//         let next_item = array[i - 1];
//         array[i] = next_item;
//         i -= 1;
//     }
//     array[index] = item;
// }
//
// /**
//  * Find the index to insert the array element
//  */
// pub fn find_index(array: &mut [i32; 3], calorie: i32) -> Option<usize> {
//     let mut index = 3;
//     while calorie > array[index - 1] {
//         index -= 1;
//         if index == 0 {
//             break
//         }
//     }
//
//     if index > 2 {
//         return None;
//     }
//     if index == 0 && calorie > array[0] {
//         return Some(0);
//     }
//
//     Some(index)
// }
