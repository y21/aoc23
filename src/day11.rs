use std::cmp::Ordering;

use crate::grid::ByteGridView;
use crate::grid::Position;

fn normalize_coords(
    cols: &[bool],
    rows: &[bool],
    x: usize,
    y: usize,
    expansions: usize,
) -> (usize, usize) {
    let mut nx = x;
    let mut ny = y;
    for _ in cols.iter().take(x).filter(|&&v| !v) {
        nx += expansions - 1;
    }
    for _ in rows.iter().take(y).filter(|&&v| !v) {
        ny += expansions - 1;
    }
    (nx, ny)
}

// Part 1 and part 2 are literally the same except for the amount of expansions per empty row/column
fn solve(input: &str, expansions: usize) -> i64 {
    let grid = ByteGridView::from(input);
    let mut galaxy_rows = vec![false; grid.rows()];
    let mut galaxy_cols = vec![false; grid.columns()];
    let mut galaxies = Vec::new();

    for y in 0..grid.rows() {
        for (x, &col) in grid[y].iter().enumerate() {
            let is_galaxy = col == b'#';
            galaxy_rows[y] |= is_galaxy;
            galaxy_cols[x] |= is_galaxy;
            if is_galaxy {
                galaxies.push(Position { x, y });
            }
        }
    }

    let mut steps = 0;

    for (i, galaxy) in galaxies.iter().copied().enumerate() {
        let (tx, ty) = normalize_coords(&galaxy_cols, &galaxy_rows, galaxy.x, galaxy.y, expansions);

        for &galaxy in &galaxies[i..] {
            let (ox, oy) =
                normalize_coords(&galaxy_cols, &galaxy_rows, galaxy.x, galaxy.y, expansions);

            let xdist = tx.abs_diff(ox);
            let ydist = ty.abs_diff(oy);
            steps += match xdist.cmp(&ydist) {
                Ordering::Greater | Ordering::Equal => (ydist * 2) + xdist - ydist,
                Ordering::Less => (xdist * 2) + ydist - xdist,
            };
        }
    }

    steps as i64
}

pub fn part1(input: &str) -> i64 {
    solve(input, 2)
}

pub fn part2(input: &str) -> i64 {
    solve(input, 1_000_000)
}

#[cfg(test)]
#[test]
fn p11t() {
    const INPUT: &str = include_str!("../inputs/day11.txt");
    assert_eq!(part1(INPUT.trim()), 9509330);
    assert_eq!(part2(INPUT.trim()), 635832237682);
}
