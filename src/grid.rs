use std::ops::Index;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
pub struct Position {
    pub y: usize,
    pub x: usize,
}

impl Position {
    pub fn right(self) -> Self {
        Self {
            y: self.y,
            x: self.x + 1,
        }
    }
    pub fn down(self) -> Self {
        Self {
            y: self.y + 1,
            x: self.x,
        }
    }
    pub fn left(self) -> Self {
        Self {
            y: self.y,
            x: self.x - 1,
        }
    }
    pub fn up(self) -> Self {
        Self {
            y: self.y - 1,
            x: self.x,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ByteGridView<'a> {
    store: &'a [u8],
    rows: usize,
    columns: usize,
}

impl<'a> From<&'a str> for ByteGridView<'a> {
    fn from(s: &'a str) -> Self {
        s.as_bytes().into()
    }
}

impl<'a> From<&'a [u8]> for ByteGridView<'a> {
    fn from(s: &'a [u8]) -> Self {
        assert!(!s.ends_with(b"\n"), "grid source ends with newline");
        let columns = memchr::memchr(b'\n', s).expect("missing newline in grid source");
        let rows = s.iter().filter(|&&v| v == b'\n').count() + 1;

        Self {
            store: s,
            rows,
            columns,
        }
    }
}

impl<'a> ByteGridView<'a> {
    pub fn rows(&self) -> usize {
        self.rows
    }

    pub fn is_right_edge(&self, c: usize) -> bool {
        self.columns() - 1 == c
    }

    pub fn is_left_edge(&self, c: usize) -> bool {
        0 == c
    }

    pub fn is_top_edge(&self, r: usize) -> bool {
        r == 0
    }

    pub fn is_bottom_edge(&self, r: usize) -> bool {
        self.rows() - 1 == r
    }

    pub fn columns(&self) -> usize {
        self.columns
    }

    /// Normalize to a row index
    pub fn norm_to_row(&self, index: usize) -> usize {
        index / (self.columns() + 1)
    }

    /// Normalize to a column index
    pub fn norm_to_col(&self, index: usize) -> usize {
        index % (self.columns() + 1)
    }

    /// Consider this grid as an example:
    /// ```
    /// .........
    /// ..12345..
    /// ..6+++7..
    /// ..89012..
    /// .........
    /// ```
    /// Given the range of `+++`, the returned iterator will yield all the shown numbers.
    pub fn multi_column_neighbors(
        &self,
        row: usize,
        col_from: usize,
        col_to: usize,
    ) -> impl Iterator<Item = u8> + '_ {
        self.multi_column_neighbors_with_coordinates(row, col_from, col_to)
            .map(|(c, ..)| c)
    }

    pub fn multi_column_neighbors_with_coordinates(
        &self,
        row: usize,
        col_from: usize,
        col_to: usize,
    ) -> impl Iterator<Item = (u8, usize, usize)> + '_ {
        std::iter::from_coroutine(move || {
            // Row up top
            if let Some(above_row) = row.checked_sub(1) {
                let col_start = col_from.saturating_sub(1);
                let col_end = col_to.min(self.columns() - 1);

                for (i, &c) in self[above_row][col_start..=col_end].iter().enumerate() {
                    yield (c, above_row, col_start + i);
                }
            }

            // Row below
            if row != self.rows() - 1 {
                let col_start = col_from.saturating_sub(1);
                let col_end = col_to.min(self.columns() - 1);

                for (i, &c) in self[row + 1][col_start..=col_end].iter().enumerate() {
                    yield (c, row + 1, col_start + i);
                }
            }

            // Left and right columns
            if let Some(left_column) = col_from.checked_sub(1) {
                yield (self[row][left_column], row, left_column);
            }

            if col_to != self.columns() {
                yield (self[row][col_to], row, col_to);
            }
        })
    }

    pub fn non_diagonal_neighbors(
        &self,
        row: usize,
        col: usize,
    ) -> impl Iterator<Item = (u8, usize, usize)> + '_ {
        std::iter::from_coroutine(move || {
            if let Some(left_column) = col.checked_sub(1) {
                yield (self[row][left_column], row, left_column);
            }

            if col != self.columns() - 1 {
                yield (self[row][col + 1], row, col + 1);
            }

            if let Some(top_row) = row.checked_sub(1) {
                yield (self[top_row][col], top_row, col);
            }

            if row != self.rows() - 1 {
                yield (self[row + 1][col], row + 1, col);
            }
        })
    }
}

impl<'a> Index<usize> for ByteGridView<'a> {
    type Output = [u8];

    fn index(&self, index: usize) -> &Self::Output {
        let start = index * (self.columns + 1);
        let end = start + self.columns;
        &self.store[start..end]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid() {
        let grid = ByteGridView::from(
            "467..114...
...*.......
..35..633..
......#....
617*.......
.....+.58..
..592......
......755..
...$.*.....
.664.598...
"
            .trim(),
        );
        assert_eq!(grid.columns(), 11);
        assert_eq!(grid.rows(), 10);
        assert_eq!(&grid[0], b"467..114...");
        assert_eq!(&grid[1], b"...*.......");
        assert_eq!(&grid[9], b".664.598...");
    }
}
