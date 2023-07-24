use std::{collections::HashSet, fs};

fn main() {
    let filename = "C:\\Users\\lucas\\repos\\advent-of-code-2022\\day3\\src\\input.txt";
    // let sample_filename =
    //     "C:\\Users\\lucas\\repos\\advent-of-code-2022\\day3\\src\\sample_input.txt";
    let mut total_value_of_priorities = 0;

    for line in fs::read_to_string(filename).unwrap().lines() {
        let (left_slice, right_slice) = get_string_slices(line);
        let common_char = find_intersection(left_slice, right_slice);
        total_value_of_priorities += get_value_of_char(common_char);
    }
    println!("The total value is {}", total_value_of_priorities);
}

fn get_string_slices(line: &str) -> (&str, &str) {
    let (middle_index, ending_index) = (line.len() / 2, line.len());
    (&line[0..middle_index], &line[middle_index..ending_index])
}

fn find_intersection(left_slice: &str, right_slice: &str) -> char {
    let left_slice_as_set: HashSet<char> = left_slice.chars().collect();
    let right_slice_as_set: HashSet<char> = right_slice.chars().collect();

    *left_slice_as_set
        .intersection(&right_slice_as_set)
        .into_iter()
        .next()
        .unwrap()
}

fn get_value_of_char(common_char: char) -> i32 {
    if common_char as i32 > 96 {
        common_char as i32 - 96
    } else {
        common_char as i32 - 38
    }
}
