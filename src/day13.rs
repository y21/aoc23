use crate::grid::ByteGridView;

pub fn part1(input: &str) -> i64 {
    let mut sum = 0;

    for pattern in input.split("\n\n") {
        let grid = ByteGridView::from(pattern);

        // horizontal top side
        let mut mirror = grid.rows() / 2;
        while mirror >= 1 {
            if (0..mirror)
                .zip((0..mirror * 2).rev())
                .all(|(r1, r2)| grid[r1] == grid[r2])
            {
                sum += 100 * mirror as i64;
                break;
            }

            mirror -= 1;
        }

        // horizontal bottom side
        let mut mirror = grid.rows() / 2;
        while mirror >= 1 {
            if (grid.rows() - mirror..grid.rows())
                .rev()
                .zip(grid.rows() - mirror * 2..grid.rows() - mirror)
                .all(|(r1, r2)| grid[r1] == grid[r2])
            {
                sum += 100 * (grid.rows() - mirror) as i64;
                break;
            }

            mirror -= 1;
        }

        // vertical left side
        let mut mirror = grid.columns() / 2;
        while mirror >= 1 {
            if (0..mirror)
                .zip((0..mirror * 2).rev())
                .all(|(c1, c2)| (0..grid.rows()).all(|row| grid[row][c1] == grid[row][c2]))
            {
                sum += mirror as i64;
                break;
            }
            mirror -= 1;
        }

        // vertical right side
        let mut mirror = grid.columns() / 2;
        while mirror >= 1 {
            if (grid.columns() - mirror..grid.columns())
                .rev()
                .zip(grid.columns() - mirror * 2..grid.columns() - mirror)
                .all(|(c1, c2)| (0..grid.rows()).all(|row| grid[row][c1] == grid[row][c2]))
            {
                sum += (grid.columns() - mirror) as i64;
                break;
            }
            mirror -= 1;
        }
    }

    sum
}

pub fn part2(input: &str) -> i64 {
    let mut sum = 0;

    for pattern in input.split("\n\n") {
        fn winnow_row(grid: ByteGridView<'_>, iter: impl Iterator<Item = (usize, usize)>) -> bool {
            let mut mirrored = true;
            let mut used_smudge: bool = false;

            for (r1, r2) in iter {
                let wrong_ones = grid[r1]
                    .iter()
                    .zip(&grid[r2])
                    .filter(|(a, b)| a != b)
                    .count();

                match wrong_ones {
                    0 => {}
                    1 if !used_smudge => {
                        used_smudge = true;
                    }
                    _ => {
                        mirrored = false;
                        break;
                    }
                }
            }
            mirrored && used_smudge
        }

        fn winnow_col(grid: ByteGridView<'_>, iter: impl Iterator<Item = (usize, usize)>) -> bool {
            let mut mirrored = true;
            let mut used_smudge = false;

            for (c1, c2) in iter {
                let wrong_ones = (0..grid.rows())
                    .filter(|&row| grid[row][c1] != grid[row][c2])
                    .count();

                match wrong_ones {
                    0 => {}
                    1 if !used_smudge => {
                        used_smudge = true;
                    }
                    _ => {
                        mirrored = false;
                        break;
                    }
                }
            }

            mirrored && used_smudge
        }

        let grid = ByteGridView::from(pattern);

        // horizontal top side
        let mut mirror = grid.rows() / 2;
        while mirror >= 1 {
            if winnow_row(grid, (0..mirror).zip((0..mirror * 2).rev())) {
                sum += (100 * (mirror)) as i64;
            }
            mirror -= 1;
        }

        // horizontal bottom side
        let mut mirror = grid.rows() / 2;
        while mirror >= 1 {
            if winnow_row(
                grid,
                (grid.rows() - mirror..grid.rows())
                    .rev()
                    .zip(grid.rows() - mirror * 2..grid.rows() - mirror),
            ) {
                sum += (100 * (grid.rows() - mirror)) as i64;
            }
            mirror -= 1;
        }

        // vertical left side
        let mut mirror = grid.columns() / 2;
        while mirror >= 1 {
            if winnow_col(grid, (0..mirror).zip((0..mirror * 2).rev())) {
                sum += mirror as i64;
            }
            mirror -= 1;
        }

        // vertical right side
        let mut mirror = grid.columns() / 2;
        while mirror >= 1 {
            if winnow_col(
                grid,
                (grid.columns() - mirror..grid.columns())
                    .rev()
                    .zip(grid.columns() - mirror * 2..grid.columns() - mirror),
            ) {
                sum += (grid.columns() - mirror) as i64;
            }

            mirror -= 1;
        }
    }
    sum
}

#[cfg(test)]
#[test]
fn p13t() {
    const INPUT: &str = include_str!("../inputs/day13.txt");
    assert_eq!(part1(INPUT.trim()), 43614);
    assert_eq!(part2(INPUT.trim()), 36771);
}
