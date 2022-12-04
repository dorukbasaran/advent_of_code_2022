use std::fs;

fn check_if_overlaps_part1(pair1: &Vec<&str>, pair2: &Vec<&str>) -> bool {
    let pair_one_first: usize = pair1[0].parse().unwrap();
    let pair_one_last: usize = pair1[1].parse().unwrap();

    let pair_two_first: usize = pair2[0].parse().unwrap();
    let pair_two_last: usize = pair2[1].parse().unwrap();

    ((pair_one_first <= pair_two_first) && (pair_one_last >= pair_two_last))
        || ((pair_two_first <= pair_one_first) && (pair_two_last >= pair_one_last))
}

fn check_if_overlaps_part2(pair1: &Vec<&str>, pair2: &Vec<&str>) -> bool {
    let pair_one_first: usize = pair1[0].parse().unwrap();
    let pair_one_last: usize = pair1[1].parse().unwrap();

    let pair_two_first: usize = pair2[0].parse().unwrap();
    let pair_two_last: usize = pair2[1].parse().unwrap();

    (pair_two_first <= pair_one_last && pair_two_first >= pair_one_first)
        || (pair_one_first <= pair_two_last && pair_one_first >= pair_two_first)
}

fn main() {
    let mut counter_part1 = 0;
    let mut counter_part2 = 0;
    let input = fs::read_to_string("input.txt").unwrap();
    let input_lines: Vec<&str> = input.lines().into_iter().collect();
    for i in 0..input_lines.len() {
        let pairs: Vec<&str> = input_lines[i].split(",").collect();
        let pair_one: Vec<&str> = pairs[0].split("-").collect();
        let pair_two: Vec<&str> = pairs[1].split("-").collect();
        let result_part1 = check_if_overlaps_part1(&pair_one, &pair_two);
        if result_part1 {
            counter_part1 += 1;
        }
        let result_part2 = check_if_overlaps_part2(&pair_one, &pair_two);
        if result_part2 {
            counter_part2 += 1;
        }
    }
    println!("{counter_part1}");
    println!("{counter_part2}");
}
