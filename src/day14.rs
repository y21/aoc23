use std::fmt::Debug;

use itertools::Itertools;
use memchr::memchr;

#[derive(Copy, Clone, PartialEq, Eq)]
enum Rock {
    Rounded,
    Cube,
    Space,
}

impl Debug for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rock::Rounded => write!(f, "O"),
            Rock::Cube => write!(f, "#"),
            Rock::Space => write!(f, "."),
        }
    }
}

pub fn part1(input: &str) -> i64 {
    let mut grid = input
        .bytes()
        .filter(|&b| b != b'\n')
        .map(|c| match c {
            b'#' => Rock::Cube,
            b'O' => Rock::Rounded,
            b'.' => Rock::Space,
            _ => unreachable!(),
        })
        .collect_vec();

    // TODO: write something like ByteGridView but allowing mutability
    let columns = memchr(b'\n', input.as_bytes()).unwrap();
    let rows = input[columns + 1..].bytes().filter(|&v| v == b'\n').count() + 2;
    let toi = |r, c| r * columns + c;
    let mut sum = 0;

    for row in 0..rows {
        for col in 0..columns {
            if let Rock::Rounded = grid[toi(row, col)] {
                if row > 0
                    && let Some(nrow) = (0..=row - 1)
                        .rev()
                        .take_while(|&row| grid[toi(row, col)] == Rock::Space)
                        .last()
                {
                    sum += (rows - nrow) as i64;
                    grid[toi(row, col)] = Rock::Space;
                    grid[toi(nrow, col)] = Rock::Rounded;
                } else {
                    sum += (rows - row) as i64;
                }
            }
        }
    }

    sum
}

pub fn _part2(_input: &str) -> i64 {
    todo!()
}

#[cfg(test)]
#[test]
fn p14t() {
    const INPUT: &str = include_str!("../inputs/day14.txt");
    assert_eq!(part1(INPUT.trim()), 106186);
}
