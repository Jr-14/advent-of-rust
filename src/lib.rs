
pub mod day_1 {
    use std::error::Error;
    use std::fs;

    pub fn calorie_counting_part_one(file: &str) -> Result<i32, Box<dyn Error>> {
        let contents = fs::read_to_string(file)?;

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

    pub fn calorie_counting_part_two(file: &str) -> Result<i32, Box<dyn Error>> {
        let contents = fs::read_to_string(file)?;
        let mut max_calories = [0, 0, 0];
        let mut current_calory_tally = 0;
        for line in contents.lines() {
            if line == "" {
                if let Some(index) = find_index(&mut max_calories, current_calory_tally) {
                    insert_at_index(&mut max_calories, index, current_calory_tally);
                }
                current_calory_tally = 0;
            } else {
                let calorie: i32 = line.parse()?;
                current_calory_tally += calorie;
            }
        }
        Ok(max_calories.iter().sum())
    }

    pub fn update_calorie_array(calorie_array: [i32; 3], calorie: i32) -> [i32; 3] {
        let mut new_calorie_array = calorie_array.clone();
        for (index, existing_calorie) in calorie_array.iter().enumerate() {
            if calorie > *existing_calorie {
                let mut next_calorie = calorie;
                let mut array_index = index;
                while array_index < 3 {
                    let calorie_to_move = calorie_array[array_index];
                    new_calorie_array[array_index] = next_calorie;
                    array_index += 1;
                    if array_index >= 3 {
                        break;
                    }
                    new_calorie_array[array_index] = calorie_to_move;
                    next_calorie = new_calorie_array[array_index];
                }
                break;
            }
        }
        new_calorie_array
    }

    /**
     * Insert item at index and bubble down the values
     */
    pub fn insert_at_index(array: &mut [i32; 3], index: usize, item: i32) {
        let mut i: usize = 2;
        while i >= index + 1 {
            let next_item = array[i - 1];
            array[i] = next_item;
            i -= 1;
        }
        array[index] = item;
    }

    /**
     * Find the index to insert the array element
     */
    pub fn find_index(array: &mut [i32; 3], calorie: i32) -> Option<usize> {
        let mut index = 3;
        while calorie > array[index - 1] {
            index -= 1;
            if index == 0 {
                break
            }
        }

        if index > 2 {
            return None;
        }
        if index == 0 && calorie > array[0] {
            return Some(0);
        }

        Some(index)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bubble_borrowed_test() {
        let mut array: [i32; 3] = [3,2,1];
        day_1::insert_at_index(&mut array, 0, 5);
        assert_eq!(array, [5, 3, 2]);
    }

    #[test]
    fn test_find_index() {
        let mut array: [i32; 3] = [7, 5, 3];
        let index = day_1::find_index(&mut array, 8);
        match index {
            Some(i) => assert_eq!(i, 0),
            _ => panic!("Failed to run test."),
        }

        let mut array: [i32; 3] = [7, 5, 3];
        let index = day_1::find_index(&mut array, 6);
        match index {
            Some(i) => assert_eq!(i, 1),
            _ => panic!("Failed to run test."),
        }

        let mut array: [i32; 3] = [7, 5, 3];
        let index = day_1::find_index(&mut array, 4);
        match index {
            Some(i) => assert_eq!(i, 2),
            _ => panic!("Failed to run test."),
        }

        let mut array: [i32; 3] = [7, 5, 3];
        let index = day_1::find_index(&mut array, 2);
        match index {
            Some(_) => panic!("Failed to run test case."),
            None => assert_eq!(index, None),
        }
    }

    #[test]
    fn test_insert_at_index() {
        let mut array: [i32; 3] = [7, 5, 3];
        day_1::insert_at_index(&mut array, 0, 8);
        assert_eq!(array, [8, 7, 5]);

        let mut array: [i32; 3] = [7, 5, 3];
        day_1::insert_at_index(&mut array, 1, 8);
        assert_eq!(array, [7, 8, 5]);

        let mut array: [i32; 3] = [7, 5, 3];
        day_1::insert_at_index(&mut array, 2, 8);
        assert_eq!(array, [7, 5, 8]);
    }

    #[test]
    fn test_calorie_counting_part_two() {
        let file_path = "./inputs/day_1_test.txt";
        let sum_calories = day_1::calorie_counting_part_two(&file_path);
        match sum_calories {
            Ok(total_calories) => assert_eq!(total_calories, 690),
            Err(e) => panic!("Failed test {}", e),
        }
    }
}