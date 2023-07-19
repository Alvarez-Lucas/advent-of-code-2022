use std::fs;

fn main() {
    let filepath = "C:\\Users\\lucas\\repos\\advent-of-code-2022\\day1-problem1\\src\\input.txt";
    let sample_filepath =
        "C:\\Users\\lucas\\repos\\advent-of-code-2022\\day1-problem1\\src\\sample_input.txt";
    let mut list_of_calories = parse_elves_from_file(&filepath);

    list_of_calories.sort();
    list_of_calories.reverse();

    println!(
        "The Elf carrying the most Calories is carrying {} calories.",
        list_of_calories[0]
    );
    println!(
        "The total of the top three elves's calories is {}",
        list_of_calories[0..=2].iter().sum::<i32>()
    );
}

fn parse_elves_from_file(filepath: &str) -> Vec<i32> {
    let mut list_of_calories: Vec<i32> = Vec::new();
    let mut current_calories = 0;
    for line in fs::read_to_string(filepath).unwrap().trim().lines() {
        match line {
            "" => {
                // TODO: understand that this is passing by value, not by
                // reference,  since it is a primative, but why does it keep
                // the value after it pushes? Should it not transfer ownership?
                list_of_calories.push(current_calories);
                current_calories = 0;
            }
            _ => {
                current_calories += line.parse::<i32>().unwrap();
            }
        }
    }
    return list_of_calories;
}
