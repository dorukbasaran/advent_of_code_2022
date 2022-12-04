use std::fs;

fn calculate_win_part1(opponent_selection: &str, my_selection: &str) -> i32 {
    /*
        A => Rock,
        B => Paper,
        C => Scissors
    */
    if opponent_selection.eq("A") {
        match my_selection {
            "X" => 4,
            "Y" => 8,
            "Z" => 3,
            _ => 0,
        }
    } else if opponent_selection.eq("B") {
        match my_selection {
            "X" => 1,
            "Y" => 5,
            "Z" => 9,
            _ => 0,
        }
    } else {
        match my_selection {
            "X" => 7,
            "Y" => 2,
            "Z" => 6,
            _ => 0,
        }
    }
}

fn calculate_win_part2(opponent_selection: &str, my_selection: &str) -> i32 {
    /*
        A => Rock,
        B => Paper,
        C => Scissors,
        X => Lose,
        Y => Draw,
        Z => Win
    */
    if opponent_selection.eq("A") {
        match my_selection {
            "X" => 3,
            "Y" => 4,
            "Z" => 8,
            _ => 0,
        }
    } else if opponent_selection.eq("B") {
        match my_selection {
            "X" => 1,
            "Y" => 5,
            "Z" => 9,
            _ => 0,
        }
    } else {
        match my_selection {
            "X" => 2,
            "Y" => 6,
            "Z" => 7,
            _ => 0,
        }
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let splitted_input: Vec<&str> = input.split("\n").collect();
    let mut total_point_part1 = 0;
    let mut total_point_part2 = 0;
    for item in splitted_input {
        let temp_item: Vec<&str> = item.split(" ").collect();
        total_point_part1 += calculate_win_part1(temp_item[0], temp_item[1]);
        total_point_part2 += calculate_win_part2(temp_item[0], temp_item[1]);
    }
    println!("{total_point_part1}");
    println!("{total_point_part2}");
}
