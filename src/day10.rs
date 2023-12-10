use std::cmp::Ordering;
use std::collections::VecDeque;

use rustc_hash::FxHashSet;

use crate::grid::ByteGridView;

#[derive(Clone, Copy, Hash, Debug, PartialEq, Eq)]
struct Position {
    y: usize,
    x: usize,
}

#[derive(Copy, Clone)]
enum Direction {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Edge {
    Top,
    Bottom,
    Left,
    Right,
}

impl Edge {
    const ALL: &'static [Self] = &[Self::Top, Self::Bottom, Self::Left, Self::Right];

    fn of(b: u8) -> &'static [Self] {
        match b {
            b'|' => &[Self::Top, Self::Bottom],
            b'-' => &[Self::Left, Self::Right],
            b'L' => &[Self::Top, Self::Right],
            b'J' => &[Self::Top, Self::Left],
            b'7' => &[Self::Left, Self::Bottom],
            b'F' => &[Self::Bottom, Self::Right],
            b'.' => &[],
            b'S' => Self::ALL,
            other => panic!("no edge for {}", other as char),
        }
    }

    fn invert_horizontal(self) -> Self {
        match self {
            Self::Left => Self::Right,
            Self::Right => Self::Left,
            x => x,
        }
    }

    /// 7F, JL => true
    /// 7L, JF => false
    fn horizontal_symmetric(this: &[Self], other: &[Self]) -> bool {
        this.len() == other.len() && this.iter().all(|v| other.contains(&v.invert_horizontal()))
    }

    fn connects_to(direction: Direction, this: &[Self], other: &[Self]) -> bool {
        this.iter().any(|v| match (v, direction) {
            (Edge::Bottom, Direction::Top) => other.contains(&Edge::Top),
            (Edge::Top, Direction::Bottom) => other.contains(&Edge::Bottom),
            (Edge::Right, Direction::Left) => other.contains(&Edge::Left),
            (Edge::Left, Direction::Right) => other.contains(&Edge::Right),
            _ => false,
        })
    }
}

pub fn part1(input: &str) -> i64 {
    let grid = ByteGridView::from(input);
    let start = input.bytes().position(|v| v == b'S').unwrap();
    let sx = grid.norm_to_col(start);
    let sy = grid.norm_to_row(start);

    let mut seen = FxHashSet::default();
    let mut queue = VecDeque::<(Position, usize)>::new();
    queue.push_back((Position { x: sx, y: sy }, 0));

    while let Some((pos, length)) = queue.pop_front() {
        seen.insert(pos);

        let sym = grid[pos.y][pos.x];

        let neighbors = grid
            .non_diagonal_neighbors(pos.y, pos.x)
            .filter(|&(v, ..)| v != b'.')
            .filter(|&(_, y, x)| !seen.contains(&Position { y, x }));

        let mut had_neighbor = false;
        for (dir, dy, dx) in neighbors {
            let dest = Position { y: dy, x: dx };

            had_neighbor = true;

            let direction = match (dy.cmp(&pos.y), dx.cmp(&pos.x)) {
                (Ordering::Less, _) => Direction::Bottom,
                (Ordering::Greater, _) => Direction::Top,
                (_, Ordering::Less) => Direction::Right,
                (_, Ordering::Greater) => Direction::Left,
                _ => unreachable!(),
            };
            let this = Edge::of(sym);
            let other = Edge::of(dir);

            if Edge::connects_to(direction, this, other) {
                queue.push_back((dest, length + 1));
            }
        }

        if !had_neighbor {
            return length as i64;
        }
    }

    unreachable!()
}

pub fn part2(input: &str) -> i64 {
    let grid = ByteGridView::from(input);
    let start = input.bytes().position(|v| v == b'S').unwrap();
    let sx = grid.norm_to_col(start);
    let sy = grid.norm_to_row(start);

    let mut seen = FxHashSet::default();
    let mut queue = VecDeque::<(Position, usize)>::new();
    queue.push_back((Position { x: sx, y: sy }, 0));

    let mut min_x = usize::MAX;
    let mut min_y = usize::MAX;
    let mut max_x = usize::MIN;
    let mut max_y = usize::MIN;

    while let Some((pos, length)) = queue.pop_front() {
        min_x = min_x.min(pos.x);
        min_y = min_y.min(pos.y);
        max_x = max_x.max(pos.x);
        max_y = max_y.max(pos.y);

        seen.insert(pos);

        let sym = grid[pos.y][pos.x];

        let neighbors = grid
            .non_diagonal_neighbors(pos.y, pos.x)
            .filter(|&(v, ..)| v != b'.')
            .filter(|&(_, y, x)| !seen.contains(&Position { y, x }));

        let mut had_neighbor = false;
        for (dir, dy, dx) in neighbors {
            let dest = Position { y: dy, x: dx };

            had_neighbor = true;

            let direction = match (dy.cmp(&pos.y), dx.cmp(&pos.x)) {
                (Ordering::Less, _) => Direction::Bottom,
                (Ordering::Greater, _) => Direction::Top,
                (_, Ordering::Less) => Direction::Right,
                (_, Ordering::Greater) => Direction::Left,
                _ => unreachable!(),
            };
            let this = Edge::of(sym);
            let other = Edge::of(dir);

            if Edge::connects_to(direction, this, other) {
                queue.push_back((dest, length + 1));
            }
        }

        if !had_neighbor {
            break;
        }
    }

    let mut depth = 0;
    let mut points = 0;
    for y in min_y..=max_y {
        let mut last_pipe = None;

        for x in (min_x..=max_x).rev() {
            let pos = Position { x, y };
            let c = grid[y][x];

            if seen.contains(&pos) {
                let e1 = Edge::of(c);
                let e2 = last_pipe.map(Edge::of);

                if e2.is_some_and(|e2| Edge::horizontal_symmetric(e1, e2)) {
                    depth += 1;
                } else if c != b'-'
                    && !(last_pipe == Some(b'7') && c == b'L')
                    && !(last_pipe == Some(b'J') && c == b'F')
                {
                    last_pipe = Some(c);
                    depth += 1;
                }
            } else if depth % 2 == 1 {
                points += 1;
            }
        }
    }

    points
}

#[cfg(test)]
#[test]
fn p10t() {
    assert!(Edge::connects_to(
        Direction::Bottom,
        Edge::of(b'J'),
        Edge::of(b'F')
    ));
    assert!(!Edge::connects_to(
        Direction::Top,
        Edge::of(b'J'),
        Edge::of(b'F')
    ));

    const INPUT: &str = include_str!("../inputs/day10.txt");
    assert_eq!(part1(INPUT.trim()), 7086);
    assert_eq!(part2(INPUT.trim()), 317);
}
