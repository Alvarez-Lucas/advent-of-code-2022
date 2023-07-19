use std::fs;

fn main() {
    let filepath = "C:\\Users\\lucas\\repos\\advent-of-code-2022\\day2\\src\\input.txt";
    let mut total_score_part_1 = 0;

    for line in fs::read_to_string(filepath).unwrap().lines() {
        let enemy_move = &line[0..1];
        let my_move = &line[2..3];
        total_score_part_1 += calculate_score_part_1(enemy_move, my_move);
    }
    eprintln!("total_score = {:?}", total_score_part_1);

    let mut total_score_part_2 = 0;
    for line in fs::read_to_string(filepath).unwrap().lines() {
        let enemy_move = &line[0..1];
        let my_move = &line[2..3];
        total_score_part_2 += calculate_score_part_2(enemy_move, my_move);
    }
    eprintln!("total_score = {:?}", total_score_part_2);
}

fn calculate_score_part_1(enemy_move: &str, my_move: &str) -> i32 {
    let played_score = match my_move {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    };

    let win_score = match enemy_move {
        "A" => match my_move {
            "X" => 3,
            "Y" => 6,
            "Z" => 0,
            _ => 0,
        },
        "B" => match my_move {
            "X" => 0,
            "Y" => 3,
            "Z" => 6,
            _ => 0,
        },

        "C" => match my_move {
            "X" => 6,
            "Y" => 0,
            "Z" => 3,
            _ => 0,
        },
        _ => 0,
    };

    return played_score + win_score;
}

fn calculate_score_part_2(enemy_move: &str, my_move: &str) -> i32 {
    let win_score = match my_move {
        "X" => 0,
        "Y" => 3,
        "Z" => 6,
        _ => 0,
    };

    let played_score = match enemy_move {
        "A" => match my_move {
            "X" => 3,
            "Y" => 1,
            "Z" => 2,
            _ => 0,
        },
        "B" => match my_move {
            "X" => 1,
            "Y" => 2,
            "Z" => 3,
            _ => 0,
        },

        "C" => match my_move {
            "X" => 2,
            "Y" => 3,
            "Z" => 1,
            _ => 0,
        },
        _ => 0,
    };

    return played_score + win_score;
}
