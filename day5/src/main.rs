use std::fs;
mod part1;
use part1::CargoStacks;

fn main() {
    // Open file
    let _sample_filename =
        "C:\\Users\\lucas\\repos\\advent-of-code-2022\\day5\\src\\sample_input.txt";
    let _filename = "C:\\Users\\lucas\\repos\\advent-of-code-2022\\day5\\src\\input.txt";

    let lines = fs::read_to_string(_filename).unwrap();
    let lines = lines.lines();

    // Part 1
    let mut cargo_stacks = CargoStacks::new(lines.clone());
    cargo_stacks.play_moves();
    let part1answer: String = cargo_stacks.get_top_of_stacks().into_iter().collect();
    println!("part1answer = {:?}", part1answer);

    // Part 2
    let mut cargo_stacks_part2 = CargoStacks::new(lines);
    cargo_stacks_part2.play_moves_part_2();
    let part2answer: String = cargo_stacks_part2.get_top_of_stacks().into_iter().collect();
    println!("part2answer = {:?}", part2answer);
}
