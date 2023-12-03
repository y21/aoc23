use std::collections::HashMap;

use regex::Regex;

pub fn part1(input: &str) -> i32 {
    let mut sum = 0;

    // TODO: use Vec<_>. Better yet, make a type for it because we'll need it more later on
    let grid: Vec<Vec<_>> = input.lines().map(|v| v.bytes().collect()).collect();

    let columns = grid[0].len();
    let rows = grid.len();

    let digits = Regex::new("\\d+").unwrap();

    for mat in digits.find_iter(input) {
        let row = mat.start() / (columns + 1);
        let column = mat.start() % (columns + 1);

        let mut has_sym = false;

        let is_sym = |c: u8| !c.is_ascii_digit() && c != b'.';

        if let Some(above_row) = row.checked_sub(1) {
            let col_start = column.saturating_sub(1);
            let col_end = (column + mat.len()).min(columns - 1);

            has_sym |= grid[above_row][col_start..=col_end]
                .iter()
                .copied()
                .any(is_sym);
        }

        if !has_sym && row != rows - 1 {
            let col_start = column.saturating_sub(1);
            let col_end = (column + mat.len()).min(columns - 1);

            has_sym |= grid[row + 1][col_start..=col_end]
                .iter()
                .copied()
                .any(is_sym);
        }

        if let (Some(left_column), false) = (column.checked_sub(1), has_sym) {
            has_sym |= is_sym(grid[row][left_column]);
        }

        let right_column = column + mat.len();
        if !has_sym && right_column != columns {
            has_sym |= is_sym(grid[row][right_column]);
        }

        if has_sym {
            sum += mat.as_str().parse::<i32>().unwrap();
        }
    }

    sum
}

pub fn part2(input: &str) -> i32 {
    // TODO: use Vec<_>. Better yet, make a type for it because we'll need it more later on
    let grid: Vec<Vec<_>> = input.lines().map(|v| v.bytes().collect()).collect();

    let columns = grid[0].len();
    let rows = grid.len();

    let mut star_coords = HashMap::<(_, _), Vec<_>>::new();
    let digits = Regex::new("\\d+").unwrap();

    let mut add_at = |val: i32, y: usize, x: usize| {
        star_coords.entry((y, x)).or_default().push(val);
    };

    for mat in digits.find_iter(input) {
        let row = mat.start() / (columns + 1);
        let column = mat.start() % (columns + 1);
        let num = mat.as_str().parse::<i32>().unwrap();

        if let Some(above_row) = row.checked_sub(1) {
            let col_start = column.saturating_sub(1);
            let col_end = (column + mat.len()).min(columns - 1);

            for star_col in col_start..=col_end {
                if grid[above_row][star_col] == b'*' {
                    add_at(num, above_row, star_col);
                }
            }
        }

        if row != rows - 1 {
            let col_start = column.saturating_sub(1);
            let col_end = (column + mat.len()).min(columns - 1);

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
        if right_column != columns && grid[row][right_column] == b'*' {
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
    assert_eq!(part1(INPUT), 539590);
    assert_eq!(part2(inp), 467835);
    assert_eq!(part2(INPUT), 80703636);
}
