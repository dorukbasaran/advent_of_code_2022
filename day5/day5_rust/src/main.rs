use std::{collections::HashMap, fs};

fn give_crane_as_hashmap(crane: &str) -> HashMap<char, Vec<char>> {
    let mut crane_map: HashMap<char, Vec<char>> = HashMap::new();
    //let crane = crane.replace(" ", "");
    let crane: Vec<&str> = crane.split("\n").collect();
    for i in (0..crane.len()).rev() {
        //println!("{}", crane[i]);
        if i == (crane.len() - 1) {
            for crane_index in (1..crane[i].len()).step_by(4) {
                let crane_number = crane[i].chars().nth(crane_index).unwrap();
                crane_map.insert(crane_number, Vec::new());
            }
        } else {
            //println!("{}", crane[i]);
            for crate_index in (1..crane[i].len()).step_by(4) {
                let crate_name = crane[i].chars().nth(crate_index).unwrap();
                let crane_index =
                    char::from_digit((((crate_index - 1) / 4) + 1) as u32, 10).unwrap();
                //println!("{0} : {1}", crane_index, crate_name);
                let vec_ref = crane_map.entry(crane_index).or_insert(Vec::new());
                if crate_name.is_whitespace() {
                    continue;
                } else {
                    vec_ref.push(crate_name);
                }
            }
        }
    }
    println!("{:?}", crane_map);
    crane_map
}

fn part_1(crane: &str, instructions: &str) {
    let mut crane_map = give_crane_as_hashmap(crane);

    for instruction in instructions.lines() {
        let splitted_instruction: Vec<&str> = instruction.split(" ").collect();
        let move_amount: u32 = splitted_instruction[1].parse().unwrap();
        let move_from: char = splitted_instruction[3].parse().unwrap();
        let move_to: char = splitted_instruction[5].parse().unwrap();
        // println!(
        //     "{0}, move_amount: {1}, move_from: {2}, move_to: {3}",
        //     instruction, move_amount, move_from, move_to
        // );
        let vec_ref_of_move_from = crane_map.entry(move_from).or_insert(Vec::new());
        let mut about_to_move_crates: Vec<char> = Vec::new();
        let mut moved_crate_count = 0;
        while moved_crate_count < move_amount {
            let moving_crate = vec_ref_of_move_from.pop();
            if moving_crate.is_none() {
                continue;
            } else {
                about_to_move_crates.push(moving_crate.unwrap());
                moved_crate_count += 1;
            }
        }

        let vec_ref_of_move_to = crane_map.entry(move_to).or_insert(Vec::new());
        for moving_crate in about_to_move_crates {
            vec_ref_of_move_to.push(moving_crate);
        }
    }
    println!("{:?}", crane_map);
    let mut ordered_crane_values: Vec<_> = crane_map.into_iter().collect();
    ordered_crane_values.sort_by(|x, y| x.0.cmp(&y.0));
    let mut top_crates = String::new();
    for mut crane in ordered_crane_values {
        top_crates.push(crane.1.pop().unwrap());
    }
    println!("{top_crates}");
}

fn part_2(crane: &str, instructions: &str) {
    let mut crane_map = give_crane_as_hashmap(crane);

    for instruction in instructions.lines() {
        let splitted_instruction: Vec<&str> = instruction.split(" ").collect();
        let move_amount: u32 = splitted_instruction[1].parse().unwrap();
        let move_from: char = splitted_instruction[3].parse().unwrap();
        let move_to: char = splitted_instruction[5].parse().unwrap();
        // println!(
        //     "{0}, move_amount: {1}, move_from: {2}, move_to: {3}",
        //     instruction, move_amount, move_from, move_to
        // );
        let vec_ref_of_move_from = crane_map.entry(move_from).or_insert(Vec::new());
        let mut about_to_move_crates: Vec<char> = Vec::new();
        let mut moved_crate_count = 0;
        while moved_crate_count < move_amount {
            let moving_crate = vec_ref_of_move_from.pop();
            if moving_crate.is_none() {
                continue;
            } else {
                about_to_move_crates.push(moving_crate.unwrap());
                moved_crate_count += 1;
            }
        }
        about_to_move_crates.reverse();

        let vec_ref_of_move_to = crane_map.entry(move_to).or_insert(Vec::new());
        for moving_crate in about_to_move_crates {
            vec_ref_of_move_to.push(moving_crate);
        }
    }
    println!("{:?}", crane_map);
    let mut ordered_crane_values: Vec<_> = crane_map.into_iter().collect();
    ordered_crane_values.sort_by(|x, y| x.0.cmp(&y.0));
    let mut top_crates = String::new();
    for mut crane in ordered_crane_values {
        top_crates.push(crane.1.pop().unwrap());
    }
    println!("{top_crates}");
}

fn main() {
    let raw_input = fs::read_to_string("input.txt").expect("There is no input.txt");
    let (crane, instructions) = raw_input
        .split_once("\n\n")
        .expect("Input is not in right format");
    part_1(crane, instructions);
    part_2(crane, instructions);
}
