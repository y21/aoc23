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

        let has_symbol = grid
            .multi_column_neighbors(row, column, column + mat.len())
            .any(|c| !c.is_ascii_digit() && c != b'.');

        if has_symbol {
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

        for (neighbor, nrow, ncol) in
            grid.multi_column_neighbors_with_coordinates(row, column, column + mat.len())
        {
            if neighbor == b'*' {
                add_at(num, nrow, ncol);
            }
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
