use std::fs;

fn part_1(splitted_input: &Vec<&str>) -> u32 {
    let mut priorities: Vec<char> = Vec::new();
    for compartment in splitted_input {
        let input_len = compartment.len() / 2;
        let splitted_compartment = compartment.split_at(input_len);
        let mut found = false;
        let mut priority = 'a';
        for i in 0..input_len {
            for j in 0..input_len {
                if splitted_compartment.0.chars().nth(i).unwrap()
                    == splitted_compartment.1.chars().nth(j).unwrap()
                {
                    priority = splitted_compartment.0.chars().nth(i).unwrap();
                    found = true;
                }
            }
            if found {
                priorities.push(priority);
                break;
            }
        }
    }
    let mut sum_of_priorties = 0;
    for priority in priorities {
        if priority.is_lowercase() {
            sum_of_priorties += priority as u32 - 96;
        } else {
            sum_of_priorties += priority as u32 - 38;
        }
    }
    sum_of_priorties
}

fn part_2(splitted_input: &Vec<&str>) -> u32 {
    let input_len = splitted_input.len();
    let mut priorities: Vec<char> = Vec::new();
    let mut priority;

    for i in (0..input_len).step_by(3) {
        let mut found = false;
        if i + 2 == input_len {
            break;
        }
        for j in 0..splitted_input[i].len() {
            for k in 0..splitted_input[i + 1].len() {
                for l in 0..splitted_input[i + 2].len() {
                    if splitted_input[i].chars().nth(j).unwrap()
                        == splitted_input[i + 1].chars().nth(k).unwrap()
                        && splitted_input[i + 1].chars().nth(k).unwrap()
                            == splitted_input[i + 2].chars().nth(l).unwrap()
                    {
                        priority = splitted_input[i + 2].chars().nth(l).unwrap();
                        found = true;
                        priorities.push(priority);
                        break;
                    }
                }
                if found {
                    break;
                }
            }
            if found {
                break;
            }
        }
    }
    let mut sum_of_priorties = 0;
    for priority in priorities {
        if priority.is_lowercase() {
            sum_of_priorties += priority as u32 - 96;
        } else {
            sum_of_priorties += priority as u32 - 38;
        }
    }
    sum_of_priorties
}
fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let splitted_input: Vec<&str> = input.split("\n").collect();

    println!("{}", part_1(&splitted_input));
    println!("{}", part_2(&splitted_input));
}
