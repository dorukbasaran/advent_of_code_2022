use std::fs;

fn part_1(input: &String) -> usize {
    let size_at_most = 100000;
    let mut stack = vec![("/", 0)];
    let mut total_size = 0;

    for line in input.lines().filter(|l| !l.is_empty()) {
        if line.eq("$ cd /") || line.eq("$ ls") {
            continue;
        }

        if line.starts_with("$ cd ") {
            let currrent_directory = &line[5..]; // Take after $ cd
            if currrent_directory == ".." {
                // If we leaving current directory
                let (_, amount) = stack.pop().unwrap();
                if amount <= size_at_most {
                    total_size += amount;
                }
                stack.last_mut().unwrap().1 += amount; // last one must be parent of the current popped dir
            } else {
                stack.push((currrent_directory, 0));
            }
            continue;
        }

        let (amount, _) = line.split_once(" ").unwrap();

        if let Ok(amount) = amount.parse::<usize>() {
            stack.last_mut().unwrap().1 += amount;
        }
    }
    total_size
}

fn part_2(input: &String) -> usize {
    let total_disc_space = 70000000;
    let deleted_folders_space: usize;
    let needed_space = 30000000;
    let mut stack = vec![("/", 0)];
    let mut dirs = vec![];
    for line in input.lines().filter(|l| !l.is_empty()) {
        if line.eq("$ cd /") || line.eq("$ ls") {
            continue;
        }
        if line.starts_with("$ cd ") {
            let currrent_directory = &line[5..];
            if currrent_directory == ".." {
                let (dir_name, amount) = stack.pop().unwrap();
                stack.last_mut().unwrap().1 += amount;
                dirs.push((dir_name, amount));
            } else {
                stack.push((currrent_directory, 0));
            }
            continue;
        }

        let (amount, _) = line.split_once(" ").unwrap();

        if let Ok(amount) = amount.parse::<usize>() {
            stack.last_mut().unwrap().1 += amount;
        }
    }
    while stack.len() > 0 {
        let (dir_name, amount) = stack.pop().unwrap();
        dirs.push((dir_name, amount));
        if stack.len() > 0 {
            stack.last_mut().unwrap().1 += amount;
        }
    }

    let free_space = total_disc_space - dirs.last().unwrap().1;
    let required_space = needed_space - free_space;

    deleted_folders_space = dirs
        .into_iter()
        .filter(move |(_, amount)| *amount >= required_space)
        .map(|(_, amount)| {
            return amount;
        })
        .min()
        .unwrap();
    deleted_folders_space
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let part_1_total_size = part_1(&input);
    println!("{}", part_1_total_size);

    let part_2_deleted_space = part_2(&input);
    println!("{}", part_2_deleted_space);
}
