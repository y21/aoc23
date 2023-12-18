use itertools::Itertools;
use owo_colors::OwoColorize;
use rustc_hash::FxHashSet;

use crate::grid::Direction;

#[derive(Default, Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub struct Position {
    pub y: i64,
    pub x: i64,
}

#[allow(dead_code, reason = "debugging")]
fn visualize(edges: &FxHashSet<Position>, min_x: i64, max_x: i64, min_y: i64, max_y: i64) {
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if edges.contains(&Position { x, y }) {
                print!("{}", "#".red().bold());
            } else {
                print!("{}", ".".bright_black());
            }
        }
        println!();
    }
}

pub fn part1(input: &str) -> i64 {
    let mut edges = FxHashSet::<Position>::default();
    let mut x = 0;
    let mut y = 0;

    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;

    // find edges
    for line in input.lines() {
        let (direction, count, _) = line.split_ascii_whitespace().collect_tuple().unwrap();
        let count = count.parse::<i64>().unwrap();
        match direction {
            "R" => {
                for _ in 0..count {
                    edges.insert(Position { x, y });
                    x += 1;
                }
            }
            "L" => {
                for _ in 0..count {
                    edges.insert(Position { x, y });
                    x -= 1;
                }
            }
            "U" => {
                for _ in 0..count {
                    edges.insert(Position { x, y });
                    y -= 1;
                }
            }
            "D" => {
                for _ in 0..count {
                    edges.insert(Position { x, y });
                    y += 1;
                }
            }
            _ => todo!(),
        }

        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    let mut extra = 0;

    // fill everything within the loop
    for y in min_y..=max_y {
        let mut within = false;
        let mut x = min_x;
        while x < max_x {
            let pos = Position { y, x };

            if edges.contains(&pos) {
                // Found a hash

                if edges.contains(&Position { y, x: x + 1 }) {
                    // More than one hash next to each other, line

                    let top_left = edges.contains(&Position { y: y - 1, x });
                    let bottom_left = edges.contains(&Position { y: y + 1, x });

                    // Skip to the end of the line
                    while edges.contains(&Position { y, x }) {
                        x += 1;
                    }
                    x -= 1;

                    let top_right = edges.contains(&Position { y: y - 1, x });
                    let bottom_right = edges.contains(&Position { y: y + 1, x });

                    if (top_left && bottom_right) || (top_right && bottom_left) {
                        within = !within;
                    } else if (bottom_left && bottom_right) || (top_left && top_right) {
                        // Nothing.
                    } else {
                        // sanity check
                        unreachable!()
                    }
                } else {
                    within = !within;
                }
            } else if within {
                extra += 1;
            }

            x += 1;
        }
    }

    (extra + edges.len()) as i64
}

#[allow(
    dead_code,
    reason = "requires some further manual hacking around not in the code to get to the actual answer, TODO fully automate it"
)]
pub fn part2(input: &str) -> i64 {
    let mut edges = Vec::new();
    let mut x = 0;
    let mut y = 0;

    let mut min_x = i64::MAX;
    let mut max_x = i64::MIN;
    let mut min_y = i64::MAX;
    let mut max_y = i64::MIN;

    // find edges
    for line in input.lines() {
        let hex = line
            .rsplit_once(' ')
            .unwrap()
            .1
            .trim_start_matches("(#")
            .trim_end_matches(')');

        let direction = match hex.as_bytes().last().unwrap() {
            b'0' => Direction::Right,
            b'1' => Direction::Down,
            b'2' => Direction::Left,
            b'3' => Direction::Up,
            _ => unreachable!(),
        };
        let count = i64::from_str_radix(&hex[..hex.len() - 1], 16).unwrap();
        match direction {
            Direction::Right => {
                edges.push((x..=x + count, y..=y));
                x += count;
            }
            Direction::Left => {
                edges.push((x - count..=x, y..=y));
                x -= count;
            }
            Direction::Up => {
                edges.push((x..=x, y - count..=y));
                y -= count;
            }
            Direction::Down => {
                edges.push((x..=x, y..=y + count));
                y += count;
            }
        }

        min_x = min_x.min(x);
        min_y = min_y.min(y);
        max_x = max_x.max(x);
        max_y = max_y.max(y);
    }

    let mut extra = 0;

    let find_edge = |x, y| {
        edges
            .iter()
            .find(|(xr, yr)| xr.contains(&x) && yr.contains(&y))
    };

    // fill everything within the loop
    for y in min_y..=max_y {
        let mut within = false;
        let mut x = min_x;
        while x < max_x {
            if let Some((hx, hy)) = find_edge(x, y) {
                // Found a hash

                if hx.end() - hx.start() > 0 {
                    debug_assert!(hy.start() == hy.end());
                    // More than one hash next to each other, line
                    let top_left = find_edge(*hx.start(), hy.start() - 1).is_some();
                    let bottom_left = find_edge(*hx.start(), hy.start() + 1).is_some();
                    x = *hx.end();
                    let top_right = find_edge(*hx.end(), hy.start() - 1).is_some();
                    let bottom_right = find_edge(*hx.end(), hy.start() + 1).is_some();

                    if (top_left && bottom_right) || (top_right && bottom_left) {
                        within = !within;
                    } else if (bottom_left && bottom_right) || (top_left && top_right) {
                        // Nothing.
                    } else {
                        // sanity check
                        unreachable!()
                    }
                } else {
                    within = !within;
                }
            } else if let Some((xr, _)) = edges
                .iter()
                .find(|(xr, yr)| *xr.start() > x && yr.contains(&y))
            {
                if within {
                    extra += xr.start() - x;
                }
                x = *xr.start();
                continue;
            } else {
                break;
            }
            x += 1;
        }
    }

    extra + edges.len() as i64
}

#[cfg(test)]
#[test]
fn p18t() {
    const INPUT: &str = include_str!("../inputs/day18.txt");
    assert_eq!(part1(INPUT.trim()), 61661);
}
