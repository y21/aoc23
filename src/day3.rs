use std::collections::HashMap;

use regex::Regex;

use crate::grid::ByteGridView;

pub fn part1(input: &str) -> i32 {
    let grid = ByteGridView::from(input);
    let digits = Regex::new("\\d+").unwrap();

    let mut sum = 0;

    for mat in digits.find_iter(input) {
        let row = mat.start() / (grid.columns() + 1);
        let column = mat.start() % (grid.columns() + 1);

        let mut has_sym = false;

        let is_sym = |c: u8| !c.is_ascii_digit() && c != b'.';

        if let Some(above_row) = row.checked_sub(1) {
            let col_start = column.saturating_sub(1);
            let col_end = (column + mat.len()).min(grid.columns() - 1);

            has_sym |= grid[above_row][col_start..=col_end]
                .iter()
                .copied()
                .any(is_sym);
        }

        if !has_sym && row != grid.rows() - 1 {
            let col_start = column.saturating_sub(1);
            let col_end = (column + mat.len()).min(grid.columns() - 1);

            has_sym |= grid[row + 1][col_start..=col_end]
                .iter()
                .copied()
                .any(is_sym);
        }

        if let (Some(left_column), false) = (column.checked_sub(1), has_sym) {
            has_sym |= is_sym(grid[row][left_column]);
        }

        let right_column = column + mat.len();
        if !has_sym && right_column != grid.columns() {
            has_sym |= is_sym(grid[row][right_column]);
        }

        if has_sym {
            sum += mat.as_str().parse::<i32>().unwrap();
        }
    }

    sum
}

pub fn part2(input: &str) -> i32 {
    let grid = ByteGridView::from(input);

    let mut star_coords = HashMap::<(_, _), Vec<_>>::new();
    let digits = Regex::new("\\d+").unwrap();

    let mut add_at = |val: i32, y: usize, x: usize| {
        star_coords.entry((y, x)).or_default().push(val);
    };

    for mat in digits.find_iter(input) {
        let row = mat.start() / (grid.columns() + 1);
        let column = mat.start() % (grid.columns() + 1);
        let num = mat.as_str().parse::<i32>().unwrap();

        if let Some(above_row) = row.checked_sub(1) {
            let col_start = column.saturating_sub(1);
            let col_end = (column + mat.len()).min(grid.columns() - 1);

            for star_col in col_start..=col_end {
                if grid[above_row][star_col] == b'*' {
                    add_at(num, above_row, star_col);
                }
            }
        }

        if row != grid.rows() - 1 {
            let col_start = column.saturating_sub(1);
            let col_end = (column + mat.len()).min(grid.columns() - 1);

            for star_col in col_start..=col_end {
                if grid[row + 1][star_col] == b'*' {
                    add_at(num, row + 1, star_col);
                }
            }
        }

        if let Some(left_column) = column.checked_sub(1) {
            if grid[row][left_column] == b'*' {
                add_at(num, row, left_column);
            }
        }

        let right_column = column + mat.len();
        if right_column != grid.columns() && grid[row][right_column] == b'*' {
            add_at(num, row, right_column);
        }
    }

    star_coords
        .into_values()
        .filter_map(|nums| {
            if let &[v1, v2] = nums.as_slice() {
                Some(v1 * v2)
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
#[test]
fn p3t() {
    const INPUT: &str = include_str!("../inputs/day3.txt");
    let inp = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"
    .trim();
    assert_eq!(part1(inp), 4361);
    assert_eq!(part1(INPUT.trim()), 539590);
    assert_eq!(part2(inp), 467835);
    assert_eq!(part2(INPUT.trim()), 80703636);
}
