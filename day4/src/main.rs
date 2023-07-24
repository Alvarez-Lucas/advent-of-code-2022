use std::{collections::HashSet, fs};

fn main() {
    let _filename = "C:\\Users\\lucas\\repos\\advent-of-code-2022\\day4\\src\\input.txt";
    let _sample_filename =
        "C:\\Users\\lucas\\repos\\advent-of-code-2022\\day4\\src\\sample_input.txt";

    let mut part_1_answer = 0;
    let mut part_2_answer = 0;

    for line in fs::read_to_string(_filename).unwrap().lines() {
        let (elf1, elf2) = parse_line_into_elves(line);
        if elf1.contains(&elf2) || elf2.contains(&elf1) {
            part_1_answer += 1;
        };
        if elf1.overlaps(&elf2) {
            part_2_answer += 1;
        }
    }

    println!("Part one: {part_1_answer}");
    println!("Part two: {part_2_answer}");
}

#[derive(Debug)]
struct Elf {
    start_range: i32,
    end_range: i32,
}

impl Elf {
    fn new(start_range: i32, end_range: i32) -> Elf {
        Elf {
            start_range,
            end_range,
        }
    }

    fn contains(&self, other: &Elf) -> bool {
        self.start_range >= other.start_range && self.end_range <= other.end_range
    }

    fn overlaps(&self, other: &Elf) -> bool {
        let self_range = self.start_range..=self.end_range;
        let other_range = other.start_range..=other.end_range;

        let self_set: HashSet<i32> = self_range.collect();
        let other_set: HashSet<i32> = other_range.collect();

        return self_set.intersection(&other_set).count() > 0;
    }
}

fn parse_line_into_elves(line: &str) -> (Elf, Elf) {
    let pairs: Vec<&str> = line.split(",").collect();

    let range1: Vec<i32> = pairs[0]
        .split("-")
        .map(|index| index.parse().unwrap())
        .collect();
    let range2: Vec<i32> = pairs[1]
        .split("-")
        .map(|index| index.parse().unwrap())
        .collect();

    return (
        Elf::new(range1[0], range1[1]),
        Elf::new(range2[0], range2[1]),
    );
}
