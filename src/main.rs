use advent_of_rust::day_1;

fn main() {
    let file_name = "./inputs/day1.txt";
    let calorie_result = day_1::calorie_counting_part_two(file_name).unwrap_or_else(|err| {
        eprint!("Problem counting calorie, {}", err);
        std::process::exit(1);
    });
    println!("Max calorie is {}", calorie_result);
}