use std::str::Lines;

#[derive(Debug)]
pub struct CargoStacks {
    stacks: Vec<Vec<char>>,
    moves: Vec<Move>,
}

#[derive(Debug)]
struct Move {
    occurances: i32,
    from_index: i32,
    to_index: i32,
}

impl Move {
    fn new(occurances: i32, from_index: i32, to_index: i32) -> Self {
        Move {
            occurances,
            from_index,
            to_index,
        }
    }
}

impl CargoStacks {
    pub fn new(mut lines: Lines<'_>) -> Self {
        // get initial state lines
        let mut initial_state_lines: Vec<String> = Vec::new();
        while let Some(line) = lines.next() {
            if line.len() == 0 {
                break;
            }
            initial_state_lines.push(line.to_string());
        }
        let line_with_stack_numbers = initial_state_lines.pop().unwrap();
        let mut stacks = CargoStacks::parse_line_with_stack_number(line_with_stack_numbers);
        CargoStacks::parse_initial_state_lines(&mut stacks, initial_state_lines);
        let moves = CargoStacks::parse_move_lines(lines);
        let res = CargoStacks { stacks, moves };
        return res;
    }

    fn parse_move_lines(lines: Lines<'_>) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        for line in lines {
            // CargoStacks::parse_move_line(line.to_string());
            let list: Vec<&str> = line.split(" ").collect();

            let occurances = list[1].parse::<i32>().unwrap();
            let from_index = list[3].parse::<i32>().unwrap() - 1;
            let to_index = list[5].parse::<i32>().unwrap() - 1;
            moves.push(Move::new(occurances, from_index, to_index))
        }
        return moves;
    }

    fn parse_line_with_stack_number(line: String) -> Vec<Vec<char>> {
        let mut stacks: Vec<Vec<char>> = Vec::new();
        let values: Vec<i32> = line
            .trim()
            .split("   ")
            .map(|x| x.parse().unwrap())
            .collect();

        for _ in values {
            let stack: Vec<char> = Vec::new();
            stacks.push(stack);
        }
        return stacks;
    }

    fn parse_initial_state_lines(stacks: &mut Vec<Vec<char>>, lines: Vec<String>) {
        for line in lines.iter().rev() {
            let mut index = 1;
            let mut stack_index = 0;

            while index < line.len() {
                let charater = line.chars().nth(index).unwrap();
                if charater != ' ' {
                    stacks[stack_index].push(charater);
                };
                index += 4;
                stack_index += 1;
            }
        }
    }

    pub fn play_moves(&mut self) {
        for play in &self.moves {
            for _ in 0..play.occurances {
                let stack_to_pop: &mut Vec<char> =
                    self.stacks.get_mut(play.from_index as usize).unwrap();
                let character = stack_to_pop.pop().unwrap();
                let stack_to_push: &mut Vec<char> =
                    self.stacks.get_mut(play.to_index as usize).unwrap();
                stack_to_push.push(character);
            }
        }
    }

    pub fn get_top_of_stacks(&self) -> Vec<char> {
        let mut res: Vec<char> = Vec::new();
        for stack in self.stacks.iter() {
            res.push(stack.get(stack.len() - 1).unwrap().clone());
        }
        return res;
    }
}
