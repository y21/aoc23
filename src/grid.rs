use std::ops::Index;

#[derive(Debug)]
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
        let rows = s.len() / columns;

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
    pub fn columns(&self) -> usize {
        self.columns
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
