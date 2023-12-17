use std::collections::VecDeque;

use rustc_hash::FxHashSet;

use crate::grid::ByteGridView;
use crate::grid::Position;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn count_energnized_tiles(
    grid: ByteGridView<'_>,
    start_pos: Position,
    start_direction: Direction,
) -> i64 {
    let mut points = FxHashSet::default();
    // only used for finding potential cycles
    let mut seen = FxHashSet::default();
    let mut queue = VecDeque::new();

    queue.push_back((start_pos, start_direction));

    while let Some((pos, dir)) = queue.pop_front() {
        if !seen.insert((pos, dir)) {
            continue; // if we run into a loop, nothing to do really?
        }
        points.insert(pos);

        match (grid[pos.y][pos.x], dir) {
            (b'.', Direction::Right) if !grid.is_right_edge(pos.x) => {
                queue.push_back((pos.right(), dir))
            }
            (b'.', Direction::Down) if !grid.is_bottom_edge(pos.y) => {
                queue.push_back((pos.down(), dir))
            }
            (b'.', Direction::Left) if !grid.is_left_edge(pos.x) => {
                queue.push_back((pos.left(), dir))
            }
            (b'.', Direction::Up) if !grid.is_top_edge(pos.y) => queue.push_back((pos.up(), dir)),
            (b'.', _) => {}
            (b'|', Direction::Right | Direction::Left) => {
                if !grid.is_top_edge(pos.y) {
                    queue.push_back((pos.up(), Direction::Up));
                }
                if !grid.is_bottom_edge(pos.y) {
                    queue.push_back((pos.down(), Direction::Down));
                }
            }
            (b'|', Direction::Up) if !grid.is_top_edge(pos.y) => queue.push_back((pos.up(), dir)),
            (b'|', Direction::Down) if !grid.is_bottom_edge(pos.y) => {
                queue.push_back((pos.down(), dir))
            }
            (b'|', _) => {}
            (b'-', Direction::Up | Direction::Down) => {
                if !grid.is_left_edge(pos.x) {
                    queue.push_back((pos.left(), Direction::Left));
                }
                if !grid.is_right_edge(pos.x) {
                    queue.push_back((pos.right(), Direction::Right));
                }
            }
            (b'-', Direction::Right) if !grid.is_right_edge(pos.x) => {
                queue.push_back((pos.right(), dir));
            }
            (b'-', Direction::Left) if !grid.is_left_edge(pos.x) => {
                queue.push_back((pos.left(), dir));
            }
            (b'-', _) => {}
            (b'/', Direction::Right) if !grid.is_top_edge(pos.y) => {
                queue.push_back((pos.up(), Direction::Up));
            }
            (b'/', Direction::Up) if !grid.is_right_edge(pos.x) => {
                queue.push_back((pos.right(), Direction::Right));
            }
            (b'/', Direction::Down) if !grid.is_left_edge(pos.x) => {
                queue.push_back((pos.left(), Direction::Left));
            }
            (b'/', Direction::Left) if !grid.is_bottom_edge(pos.y) => {
                queue.push_back((pos.down(), Direction::Down));
            }
            (b'/', _) => {}
            (b'\\', Direction::Right) if !grid.is_bottom_edge(pos.y) => {
                queue.push_back((pos.down(), Direction::Down));
            }
            (b'\\', Direction::Up) if !grid.is_left_edge(pos.x) => {
                queue.push_back((pos.left(), Direction::Left));
            }
            (b'\\', Direction::Down) if !grid.is_right_edge(pos.x) => {
                queue.push_back((pos.right(), Direction::Right));
            }
            (b'\\', Direction::Left) if !grid.is_top_edge(pos.y) => {
                queue.push_back((pos.up(), Direction::Up));
            }
            (b'\\', _) => {}
            _ => unreachable!(),
        }
    }

    points.len() as i64
}

pub fn part1(input: &str) -> i64 {
    let grid = ByteGridView::from(input);
    count_energnized_tiles(grid, Position { y: 0, x: 0 }, Direction::Right)
}

pub fn part2(input: &str) -> i64 {
    let grid = ByteGridView::from(input);

    let mut combinations = vec![
        (Position { y: 0, x: 0 }, Direction::Right),
        (Position { y: 0, x: 0 }, Direction::Down),
        (
            Position {
                y: 0,
                x: grid.columns() - 1,
            },
            Direction::Left,
        ),
        (
            Position {
                y: 0,
                x: grid.columns() - 1,
            },
            Direction::Down,
        ),
        (
            Position {
                y: grid.rows() - 1,
                x: 0,
            },
            Direction::Right,
        ),
        (
            Position {
                y: grid.rows() - 1,
                x: 0,
            },
            Direction::Up,
        ),
        (
            Position {
                y: grid.rows() - 1,
                x: grid.columns() - 1,
            },
            Direction::Left,
        ),
        (
            Position {
                y: grid.rows() - 1,
                x: grid.columns() - 1,
            },
            Direction::Up,
        ),
    ];

    // top and bottom edge
    for x in 1..grid.columns() - 1 {
        combinations.push((Position { y: 0, x }, Direction::Down));
        combinations.push((Position { y: 0, x }, Direction::Left));
        combinations.push((Position { y: 0, x }, Direction::Right));

        combinations.push((
            Position {
                y: grid.rows() - 1,
                x,
            },
            Direction::Up,
        ));
        combinations.push((
            Position {
                y: grid.rows() - 1,
                x,
            },
            Direction::Left,
        ));
        combinations.push((
            Position {
                y: grid.rows() - 1,
                x,
            },
            Direction::Right,
        ));
    }

    // left and right edge
    for y in 1..grid.rows() - 1 {
        combinations.push((Position { y, x: 0 }, Direction::Up));
        combinations.push((Position { y, x: 0 }, Direction::Right));
        combinations.push((Position { y, x: 0 }, Direction::Down));

        let x = grid.columns() - 1;
        combinations.push((Position { y, x }, Direction::Up));
        combinations.push((Position { y, x }, Direction::Left));
        combinations.push((Position { y, x }, Direction::Down));
    }

    combinations
        .into_iter()
        .map(|(pos, dir)| count_energnized_tiles(grid, pos, dir))
        .max()
        .unwrap()
}

#[cfg(test)]
#[test]
fn p16t() {
    const INPUT: &str = include_str!("../inputs/day16.txt");
    const SAMPLE: &str = r#"
.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;
    assert_eq!(part1(INPUT.trim()), 6994);
    assert_eq!(part1(SAMPLE.trim()), 46);
    assert_eq!(part2(INPUT.trim()), 7488);
}
