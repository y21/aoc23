use itertools::Itertools;
use regex::Match;
use regex::Regex;

use crate::grid::ByteGridView;

pub fn part1(input: &str) -> i32 {
    let grid = ByteGridView::from(input);

    Regex::new("\\d+")
        .unwrap()
        .find_iter(input)
        .map(|m| (m, grid.norm_to_row(m.start()), grid.norm_to_col(m.start())))
        .filter(|&(ref number, row, column)| {
            grid.multi_column_neighbors(row, column, column + number.len())
                .any(|c| !c.is_ascii_digit() && c != b'.')
        })
        .map(|(number, ..)| number.as_str().parse::<i32>().unwrap())
        .sum()
}

pub fn part2(input: &str) -> i32 {
    fn filter_star_neighbors<'a, 'b>(
        grid: &'b ByteGridView<'a>,
        row: usize,
        column: usize,
        number: Match<'a>,
    ) -> impl Iterator<Item = (i32, usize, usize)> + 'b {
        grid.multi_column_neighbors_with_coordinates(row, column, column + number.len())
            .filter(|&(c, ..)| c == b'*')
            .map(move |(_, nrow, ncol)| (number.as_str().parse::<i32>().unwrap(), nrow, ncol))
    }

    let grid = ByteGridView::from(input);

    Regex::new("\\d+")
        .unwrap()
        .find_iter(input)
        .map(|m| (m, grid.norm_to_row(m.start()), grid.norm_to_col(m.start())))
        .flat_map(|(number, row, column)| filter_star_neighbors(&grid, row, column, number))
        .into_group_map_by(|&(_, nrow, ncol)| (nrow, ncol))
        .into_values()
        .filter_map(|nums| {
            if let &[(v1, ..), (v2, ..)] = nums.as_slice() {
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
