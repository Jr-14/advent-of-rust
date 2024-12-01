use std::{error::Error, fs};
use std::collections::{HashMap};

const DAY_1_2024_INPUT: &'static str = "./inputs/2024/day1";

pub fn calculate_distances(contents: &str) -> Result<i32, Box<dyn Error>> {
    let mut left_loc: Vec<i32> = Vec::new();
    let mut right_loc: Vec<i32> = Vec::new();
    for line in contents.lines() {
        let v: Vec<&str> = line.split("   ").collect();
        let left_dist: i32 = v[0].parse()?;
        let right_dist: i32 = v[1].parse()?;
        left_loc.push(left_dist); 
        right_loc.push(right_dist); 
    }

    left_loc.sort();
    right_loc.sort();

    let mut sum = 0;
    let total_dist_iter = left_loc.iter().zip(right_loc.iter());
    for dist_pair in total_dist_iter {
        let distance = (dist_pair.0 - dist_pair.1).abs();
        sum += distance;
    }

    Ok(sum)
}

pub fn calculate_similarity_score(contents: &str) -> Result<i32, Box<dyn Error>> {
    let mut left_locations: Vec<i32> = Vec::new();
    let mut locations: HashMap<i32, i32> = HashMap::new();

    for line in contents.lines() {
        let v: Vec<&str> = line.split("   ").collect();
        let left_dist: i32 = v[0].parse()?;
        let right_dist: i32 = v[1].parse()?;
        left_locations.push(left_dist); 

        locations.insert(right_dist, 0);
    }

    for loc in &left_locations {
        match locations.get(&loc) {
            Some(count) => {
                println!("Found {:?} with count - {:?}", loc, count);
                locations.insert(*loc, count + 1);
            },
            None => ()
        };
    }
    println!("{:?}", locations);

    let mut similarity_score = 0;
    for loc in left_locations {
        match locations.get(&loc) {
            Some(count) => similarity_score += loc * count,
            None => ()
        };
    }

    Ok(similarity_score)
}

pub fn run_day_1_part_1() {
   let contents = fs::read_to_string(DAY_1_2024_INPUT) .unwrap_or_else(|err| {
        eprint!("Problem reading input for AoC Day 1 Part 1 {}", err);
        std::process::exit(1);
    });
    
    let distance = match calculate_distances(&contents) {
        Ok(dist) => dist,
        Err(error) => panic!("Problem running AoC 2024 Day 1 - {:?}", error)
    };

    println!("AoC Day 1 Part 1 answer is {:?}", distance);
}

pub fn run_day_1_part_2() {
   let contents = fs::read_to_string(DAY_1_2024_INPUT) .unwrap_or_else(|err| {
        eprint!("Problem reading input for AoC Day 1 Part 2 {}", err);
        std::process::exit(1);
    });
    
    let similarity_score = match calculate_similarity_score(&contents) {
        Ok(score) => score,
        Err(error) => panic!("Problem running AoC 2024 Day 1 part 2 - {:?}", error)
    };

    println!("AoC Day 1 Part 2 answer is {:?}", similarity_score);
}
