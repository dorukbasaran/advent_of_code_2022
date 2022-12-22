use std::fs;

pub fn part_1(input: &str) -> usize {
    num_visited_by_tail(input, 1)
}

pub fn part_2(input: &str) -> usize {
    num_visited_by_tail(input, 9)
}

fn num_visited_by_tail(input: &str, knots: usize) -> usize {
    std::iter::once((vec![(0, 0); knots + 1], parse(input)))
        .fold(
            std::collections::HashSet::from([(0, 0)]),
            |mut seen, (mut pos, motions)| {
                for ((dx, dy), steps) in motions {
                    for _ in 0..steps {
                        for i in 0..3 {
                            match i {
                                0 => pos[0] = (pos[0].0 + dx, pos[0].1 + dy),
                                1 => (1..pos.len()).for_each(|i| {
                                    std::iter::once((
                                        pos[i - 1].0 - pos[i].0,
                                        pos[i - 1].1 - pos[i].1,
                                    ))
                                    .for_each(|(dx, dy)| {
                                        if dx.abs().max(dy.abs()) > 1 {
                                            pos[i] =
                                                (pos[i].0 + dx.signum(), pos[i].1 + dy.signum())
                                        }
                                    })
                                }),
                                _ => seen.extend(std::iter::once(pos[knots])),
                            }
                        }
                    }
                }
                seen
            },
        )
        .len()
}

fn parse(input: &str) -> Vec<((isize, isize), isize)> {
    input
        .lines()
        .flat_map(|line| {
            line.split_once(' ')
                .and_then(|(d, n)| match (d, n.parse().ok()?) {
                    ("U", n) => Some(((0, 1), n)),
                    ("D", n) => Some(((0, -1), n)),
                    ("L", n) => Some(((-1, 0), n)),
                    ("R", n) => Some(((1, 0), n)),
                    _ => None,
                })
        })
        .collect()
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("{}", part_1(&input));
    println!("{}", part_2(&input));
}
