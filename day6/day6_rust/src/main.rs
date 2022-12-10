use std::fs;

fn is_unique(v: &Vec<char>) -> bool {
    let mut set = std::collections::HashSet::new();
    for c in v {
        if set.contains(&c) {
            return false;
        }
        set.insert(c);
    }
    true
}

fn detect_signal_index(input: &String, unique_count: usize) -> usize {
    let mut unique_signals: Vec<char> = Vec::new();
    let mut start_of_packet_marker = 0;
    for i in 0..input.len() {
        unique_signals.push(input.chars().nth(i).unwrap());
        if unique_signals.len() > unique_count {
            unique_signals.drain(0..1);
            if is_unique(&unique_signals) {
                start_of_packet_marker = i + 1;
                break;
            }
        }
    }
    start_of_packet_marker
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("there is no file called input.txtx");

    // Part 1
    let part1_result = detect_signal_index(&input, 4);
    println!("{}", part1_result);

    // Part 2
    let part2_result = detect_signal_index(&input, 14);
    println!("{}", part2_result);
}
