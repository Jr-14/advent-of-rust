use advent_of_rust::day_1;

fn main() {
}

fn run_day_1_part_1() {
    let file_name = "./inputs/day1.txt";
    let calorie_result = day_1::calorie_counting_part_one(file_name).unwrap_or_else(|err| {
        eprint!("Problem counting calorie, {}", err);
        std::process::exit(1);
    });
    println!("Max calorie is {}", calorie_result);
}

fn run_day_2_part_2() {
    let file_name = "./inputs/day1.txt";
    let calorie_result = day_1::calorie_counting_part_two(file_name).unwrap_or_else(|err| {
        eprint!("Problem counting calorie, {}", err);
        std::process::exit(1);
    });
    println!("Max calorie is {}", calorie_result);
}