use std::{collections::HashSet, fs};

pub fn main(filename: &str) {
    let mut total_value_of_things = 0;

    let file = fs::read_to_string(filename).unwrap();
    let lines = file.lines();
    let lines_as_iterator: Vec<&str> = lines.collect();

    for group in lines_as_iterator.chunks(3) {
        let common_char = find_intersection(group[0], group[1], group[2]);
        total_value_of_things += get_value_of_char(common_char);
    }

    println!("The total value is {}", total_value_of_things);
}

fn find_intersection(person1: &str, person2: &str, person3: &str) -> char {
    let person1: HashSet<char> = person1.chars().collect();
    let person2: HashSet<char> = person2.chars().collect();
    let person3: HashSet<char> = person3.chars().collect();

    *person1
        .intersection(&person2)
        .into_iter()
        .copied()
        .collect::<HashSet<char>>()
        .intersection(&person3)
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
