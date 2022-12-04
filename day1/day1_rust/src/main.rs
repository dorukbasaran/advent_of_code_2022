use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").expect("boom");
    let split_input: Vec<&str> = input.split("\n").collect();
    let mut temp_count = 0;
    let mut count = 0;
    let mut inputs: Vec<i32> = Vec::new();

    for i in 0..split_input.len() {
        let i = split_input[i].parse().unwrap_or_else(|_error| {
            return 0;
        });
        if i == 0 {
            inputs.push(count);
            temp_count = 0;
            count = 0;
        }
        temp_count += i;
        if temp_count > count {
            count = temp_count;
        }
        if i == (split_input.len() - 1) {
            inputs.push(count);
        }
    }

    inputs.sort();

    let mut top_three = 0;
    for i in ((inputs.len() - 3)..inputs.len()).rev() {
        top_three += inputs[i];
    }

    println!("{top_three}");
}
