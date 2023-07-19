use std::fs;

#[derive(Debug, Clone)]
struct Elf {
    calories: Vec<i32>,
    id: i32,
}

impl Elf {
    fn new(id: i32) -> Elf {
        Elf {
            calories: Vec::new(),
            id,
        }
    }
    // TODO: sum
    fn get_total_calories(&self) -> i32 {
        self.calories.iter().sum()
    }
}

#[derive(Debug)]
struct Elves {
    elves: Vec<Elf>,
}

impl Elves {
    fn new() -> Elves {
        Elves { elves: Vec::new() }
    }
    fn get_biggest_one(&self) -> (i32, i32) {
        let mut id_of_biggest = 0;
        let mut calories_of_biggest = 0;

        for elf in &self.elves {
            let current_elf_calories = elf.get_total_calories().clone();
            if current_elf_calories > calories_of_biggest {
                calories_of_biggest = current_elf_calories;
                id_of_biggest = elf.id;
            }
        }
        return (id_of_biggest, calories_of_biggest);
    }
}

fn main() {
    let filepath = "C:\\Users\\lucas\\repos\\advent-of-code-2022\\day1-problem1\\src\\input.txt";
    let sample_filepath =
        "C:\\Users\\lucas\\repos\\advent-of-code-2022\\day1-problem1\\src\\sample_input.txt";

    let mut elves = Elves::new();
    let mut temp_elf: Elf = Elf::new(1);

    for line in fs::read_to_string(filepath).unwrap().trim().lines() {
        match line {
            "" => {
                elves.elves.push(temp_elf.clone());
                temp_elf.id += 1;
                temp_elf.calories.clear();
            }
            _ => {
                // println!("Not a new line: {}", line);
                temp_elf
                    .calories
                    .push(line.trim().to_string().parse::<i32>().unwrap());
            }
        }
        // println!("{}", line);
    }
    // eprintln!("elves = {:?}", elves);
    let res = elves.get_biggest_one();
    // eprintln!("res = {:?}", res);
    // eprintln!("{:?}", elves);
    let mut to_be_sorted = elves.elves.clone();

    elves.elves.sort_by_key(|x| x.calories.iter().sum::<i32>());
    elves.elves.reverse();
    let mut total = 0;
    for index in 0..3 {
        total += elves.elves[index].get_total_calories();
    }
    eprintln!("total = {:?}", total);
    // to_be_sorted.sort_by_key(|x| x.calories.iter().sum::<i32>());
    // eprintln!("to_be_sorted = {:?}", to_be_sorted);
    // eprintln!("elves.elves = {:?}", elves.elves);
}
