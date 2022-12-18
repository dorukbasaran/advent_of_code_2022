use std::collections::{HashMap, HashSet};
use std::fs;

fn part_1(input_map: &Vec<Vec<u32>>) -> usize {
    let input_map_vertical_length = input_map.len();
    let input_map_horizontal_length = input_map[0].len();
    let edge_tree_counts =
        (input_map_vertical_length * 2) + ((input_map_horizontal_length - 2) * 2);

    let mut inside_visible_trees: HashSet<String> = HashSet::new();
    let mut temp_horizontal_index = 0;
    for i in 1..(input_map_vertical_length - 1) {
        if (input_map_horizontal_length - 1) > temp_horizontal_index {
            temp_horizontal_index += 1;
        }
        // Visible from left
        let mut temp_tree_list = Vec::new();
        temp_tree_list.push(input_map[i][0]);
        for j in 1..(input_map_horizontal_length - 1) {
            if &input_map[i][j] > temp_tree_list.iter().max().unwrap() {
                inside_visible_trees.insert(format!("{},{},{}", i, j, input_map[i][j]));
                temp_tree_list.push(input_map[i][j]);
            }
        }
        // Visible from right
        temp_tree_list.clear();
        temp_tree_list.push(input_map[i][input_map_horizontal_length - 1]);
        for j in (1..(input_map_horizontal_length - 1)).rev() {
            if &input_map[i][j] > temp_tree_list.iter().max().unwrap() {
                inside_visible_trees.insert(format!("{},{},{}", i, j, input_map[i][j]));
                temp_tree_list.push(input_map[i][j]);
            }
        }
        // Visible from top
        temp_tree_list.clear();
        temp_tree_list.push(input_map[0][temp_horizontal_index]);
        for k in 1..(input_map_vertical_length - 1) {
            if &input_map[k][temp_horizontal_index] > temp_tree_list.iter().max().unwrap() {
                inside_visible_trees.insert(format!(
                    "{},{},{}",
                    k, temp_horizontal_index, input_map[k][temp_horizontal_index]
                ));
                temp_tree_list.push(input_map[k][temp_horizontal_index]);
            }
        }
        // Visible from bottom
        temp_tree_list.clear();
        temp_tree_list.push(input_map[input_map_vertical_length - 1][temp_horizontal_index]);
        for k in (1..(input_map_vertical_length - 1)).rev() {
            if &input_map[k][temp_horizontal_index] > temp_tree_list.iter().max().unwrap() {
                inside_visible_trees.insert(format!(
                    "{},{},{}",
                    k, temp_horizontal_index, input_map[k][temp_horizontal_index]
                ));
                temp_tree_list.push(input_map[k][temp_horizontal_index]);
            }
        }
    }
    println!("{}", edge_tree_counts + inside_visible_trees.len());
    edge_tree_counts + inside_visible_trees.len()
}

fn part_2(input_map: &Vec<Vec<u32>>) -> usize {
    let input_map_vertical_length = input_map.len();
    let input_map_horizontal_length = input_map[0].len();

    let mut scenic_scores_of_trees: HashMap<String, u32> = HashMap::new();
    for vertical_index in 0..input_map_vertical_length {
        for horizontal_index in 0..input_map_horizontal_length {
            let mut visible_tree_count_from_left = 0;
            let mut visible_tree_count_from_right = 0;
            // Visible from left, right
            let mut dont_look_for_left = false;
            let mut dont_look_for_rigth = false;

            for i in (0..horizontal_index).rev() {
                // Left count
                if !dont_look_for_left {
                    visible_tree_count_from_left += 1;
                    if input_map[vertical_index][horizontal_index] <= input_map[vertical_index][i] {
                        dont_look_for_left = true;
                    }
                }
            }
            for j in (horizontal_index + 1)..input_map_horizontal_length {
                //Right count
                if !dont_look_for_rigth {
                    visible_tree_count_from_right += 1;
                    if input_map[vertical_index][horizontal_index] <= input_map[vertical_index][j] {
                        dont_look_for_rigth = true;
                    }
                }
            }

            // Visible from top, bottom
            let mut visible_tree_count_from_top = 0;
            let mut visible_tree_count_from_bottom = 0;
            let mut dont_look_for_top = false;
            let mut dont_look_for_bottom = false;

            // Top count
            for k in (0..vertical_index).rev() {
                if !dont_look_for_top {
                    visible_tree_count_from_top += 1;
                    if input_map[vertical_index][horizontal_index] <= input_map[k][horizontal_index]
                    {
                        dont_look_for_top = true;
                    }
                }
            }

            // Bottom count
            for l in (vertical_index + 1)..input_map_vertical_length {
                if !dont_look_for_bottom {
                    visible_tree_count_from_bottom += 1;
                    if input_map[vertical_index][horizontal_index] <= input_map[l][horizontal_index]
                    {
                        dont_look_for_bottom = true;
                    }
                }
            }
            //println!("{},{}", vertical_index, horizontal_index);
            scenic_scores_of_trees.insert(
                format!(
                    "{},{},{}",
                    vertical_index, horizontal_index, input_map[vertical_index][horizontal_index]
                ),
                visible_tree_count_from_left
                    * visible_tree_count_from_right
                    * visible_tree_count_from_top
                    * visible_tree_count_from_bottom,
            );
        }
    }
    //println!("{:?}", scenic_scores_of_trees);
    println!("{:?}", scenic_scores_of_trees.values().max().unwrap());

    *scenic_scores_of_trees.values().max().unwrap() as usize
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    let mut input_map: Vec<Vec<u32>> = Vec::new();
    for line in input.lines() {
        let mut temp_tree_line: Vec<u32> = Vec::new();
        for tree_height in line.chars() {
            temp_tree_line.push(char::to_digit(tree_height, 10).unwrap());
        }
        input_map.push(temp_tree_line);
    }
    part_1(&input_map);
    part_2(&input_map);
}
