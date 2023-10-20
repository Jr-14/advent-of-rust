
pub mod day_1 {
    use std::error::Error;

    pub fn calorie_counting_part_one(file: String) -> Result<i32, Box<dyn Error>> {
        let contents = std::fs::read_to_string(file)?;

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
}